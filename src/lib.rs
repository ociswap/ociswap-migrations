use scrypto::prelude::*;
use std::cmp::min;

#[blueprint]
mod token_swap {
    enable_method_auth! {
        methods {
            swap => PUBLIC; 
            deposit => PUBLIC; 
            withdraw => restrict_to: [OWNER];
        }
    }

    struct TokenSwap {
        new_token: Vault,
        old_token: Vault,
    }

    impl TokenSwap {
        pub fn instantiate(
            new_token: Bucket,
            old_token: ResourceAddress,
            owner_badge: Option<ResourceAddress>
        ) -> Global<TokenSwap> {
            let owner_role = match owner_badge {
                Some(owner_badge_) => OwnerRole::Fixed(rule!(require(owner_badge_))),
                None => OwnerRole::None,
            };
            (Self {
                new_token: Vault::with_bucket(new_token),
                old_token: Vault::new(old_token),
            })
                .instantiate()
                .prepare_to_globalize(owner_role)
                .globalize()
        }

        pub fn swap(&mut self, mut old_token: Bucket) -> (Bucket, Bucket) {
            let swap_amount = min(old_token.amount(), self.new_token.amount());
            let new_token = self.new_token.take(swap_amount);
            self.old_token.put(old_token.take(swap_amount));
            (new_token, old_token)
        }

        pub fn deposit(&mut self, new_token: Bucket) {
            self.new_token.put(new_token);
        }

        pub fn withdraw(&mut self, amount: Decimal) -> Bucket {
            self.new_token.take(amount)
        }
    }
}
