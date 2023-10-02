use pretty_assertions::assert_eq;
use token_migration::test_bindings::*;
use dummy_account::test_bindings::*;
use scrypto::*;
use scrypto_test::prelude::*;

pub struct MigrationHelper {
    pub env: TestEnvironment,
    pub dapp_definition: ComponentAddress,
    pub package_address: PackageAddress,
    pub x_token: Bucket,
    pub y_token: Bucket,
    pub z_token: Bucket,
    pub x_address: ResourceAddress,
    pub y_address: ResourceAddress,
    pub z_address: ResourceAddress,
    pub token_migration: Option<TokenMigration>,
}

impl MigrationHelper {
    pub fn new() -> Result<Self, RuntimeError> {
        let mut env = TestEnvironment::new();

        let package_address = Package::compile_and_publish(this_package!(), &mut env)?;
        
        let dummy_account_package_address = Package::compile_and_publish(
            "./dummy_account",
            &mut env
        )?;
        let account = DummyAccount::instantiate(dummy_account_package_address, &mut env)?;
        let dapp_definition = account.address(&mut env)?;

        let x_token = ResourceBuilder::new_fungible(OwnerRole::None)
            .divisibility(18)
            .mint_initial_supply(1000, &mut env)?;
        let y_token = ResourceBuilder::new_fungible(OwnerRole::None)
            .divisibility(18)
            .mint_initial_supply(2000, &mut env)?;
        let z_token = ResourceBuilder::new_fungible(OwnerRole::None)
            .divisibility(18)
            .mint_initial_supply(3000, &mut env)?;

        let x_address = x_token.resource_address(&mut env)?;
        let y_address = y_token.resource_address(&mut env)?;
        let z_address = z_token.resource_address(&mut env)?;

        Ok(Self {
            env,
            dapp_definition,
            package_address,
            x_token,
            y_token,
            z_token,
            x_address,
            y_address,
            z_address,
            token_migration: None,
        })
    }

    pub fn instantiate(
        &mut self,
        old_address: ResourceAddress,
        new_token: Bucket
    ) -> Result<(), RuntimeError> {
        self.token_migration = Some(
            TokenMigration::instantiate(
                old_address,
                new_token,
                self.dapp_definition,
                self.package_address,
                &mut self.env
            )?
        );

        Ok(())
    }

    pub fn instantiate_without_supply_validation(
        &mut self,
        old_address: ResourceAddress,
        new_token: Bucket
    ) -> Result<(), RuntimeError> {
        self.token_migration = Some(
            TokenMigration::instantiate_without_supply_validation(
                old_address,
                new_token,
                self.dapp_definition,
                self.package_address,
                &mut self.env
            )?
        );

        Ok(())
    }

    pub fn instantiate_default(&mut self) -> Result<(), RuntimeError> {
        let new_token = self.y_token.take(dec!(1000), &mut self.env)?;
        self.instantiate(self.x_address, new_token)?;

        Ok(())
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
