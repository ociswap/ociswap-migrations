use scrypto::prelude::*;

#[blueprint]
mod token_swap {
    struct TokenSwap {
        new_token: Vault,
        old_token: Vault,
    }

    impl TokenSwap {
        pub fn instantiate(new_token: Bucket, old_address: ResourceAddress) -> Global<TokenSwap> {
            if let Some(old_total_supply) = ResourceManager::from_address(old_address).total_supply() {
                assert_eq!(
                    new_token.amount(),
                    old_total_supply,
                    "New token amount needs to be equal to the total supply of the old token."
                );
            }
            assert_ne!(new_token.resource_address(), old_address, "New and old token are not allowed to be equal.");
            (Self {
                new_token: Vault::with_bucket(new_token),
                old_token: Vault::new(old_address),
            })
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize()
        }

        pub fn swap(&mut self, old_token: Bucket) -> Bucket {
            let new_token = self.new_token.take(old_token.amount());
            self.old_token.put(old_token);
            new_token
        }
    }
}
