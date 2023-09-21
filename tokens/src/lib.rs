use scrypto::prelude::*;

#[blueprint]
mod token_migration {
    struct TokenMigration {
        new_token: Vault,
        old_token: Vault,
    }

    impl TokenMigration {
        pub fn instantiate(old_address: ResourceAddress, new_token: Bucket) -> Global<TokenMigration> {
            assert_ne!(old_address, new_token.resource_address(), "Old and new token address are not allowed to be equal.");
            if let Some(old_total_supply) = ResourceManager::from_address(old_address).total_supply() {
                assert_eq!(
                    new_token.amount(),
                    old_total_supply,
                    "New token amount needs to be equal to the total supply of the old token."
                );
            }
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
