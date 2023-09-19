use pretty_assertions::assert_eq;
use token_swap::test_bindings::*;
use scrypto::*;
use scrypto_test::prelude::*;

pub struct EnvDefault {
    pub env: TestEnvironment,
    pub old_token: Bucket,
    pub new_token: Bucket,
    pub old_address: ResourceAddress,
    pub new_address: ResourceAddress,
    pub owner_badge: Bucket,
    pub token_swap: TokenSwap,
}

impl EnvDefault {
    pub fn new() -> Result<Self, RuntimeError> {
        let mut env = TestEnvironment::new();
        let package_address = Package::compile_and_publish(this_package!(), &mut env)?;

        let old_token = ResourceBuilder::new_fungible(OwnerRole::None)
            .divisibility(18)
            .mint_initial_supply(2000, &mut env)?;
        let new_token = ResourceBuilder::new_fungible(OwnerRole::None)
            .divisibility(18)
            .mint_initial_supply(2000, &mut env)?;
        let owner_badge = ResourceBuilder::new_fungible(OwnerRole::None)
            .divisibility(1)
            .mint_initial_supply(1, &mut env)?;

        let new_address = new_token.resource_address(&mut env)?;
        let old_address = old_token.resource_address(&mut env)?;

        let token_swap = TokenSwap::instantiate(
            new_token.take(dec!(1000), &mut env)?,
            old_address,
            None,
            package_address,
            &mut env
        )?;

        Ok(Self {
            env,
            old_address,
            new_address,
            old_token: old_token,
            new_token: new_token,
            owner_badge,
            token_swap,
        })
    }

    pub fn assert_bucket_eq(
        &mut self,
        bucket: &Bucket,
        address: ResourceAddress,
        amount: Decimal
    ) -> Result<(), RuntimeError> {
        assert_eq!(bucket.resource_address(&mut self.env)?, address);
        assert_eq!(bucket.amount(&mut self.env)?, amount);
        Ok(())
    }
}