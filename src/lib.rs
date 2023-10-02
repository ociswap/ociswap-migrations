use scrypto::prelude::*;

#[blueprint]
mod token_migration {
    struct TokenMigration {
        old_token: Vault,
        new_token: Vault,
    }

    impl TokenMigration {
        pub fn instantiate(
            old_address: ResourceAddress,
            new_token: Bucket
        ) -> Global<TokenMigration> {
            if let Some(old_total_supply) = ResourceManager::from_address(old_address).total_supply() {
                assert_eq!(
                    old_total_supply,
                    new_token.amount(),
                    "New token amount needs to be equal to the total supply of the old token."
                );
            }
            Self::instantiate_without_supply_validation(old_address, new_token)
        }

        pub fn instantiate_without_supply_validation(
            old_address: ResourceAddress,
            new_token: Bucket
        ) -> Global<TokenMigration> {
            assert_ne!(
                old_address,
                new_token.resource_address(),
                "Old and new token addresses are not allowed to be equal."
            );
            (Self {
                old_token: Vault::new(old_address),
                new_token: Vault::with_bucket(new_token),
            })
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize()
        }

        pub fn swap(&mut self, old_token: Bucket) -> Bucket {
            let old_amount = old_token.amount();
            self.old_token.put(old_token);
            self.new_token.take(old_amount)
        }
    }
}
