use scrypto::prelude::*;

#[blueprint]
mod dummy_account {
    struct DummyAccount {
        account_address: ComponentAddress,
    }

    impl DummyAccount {
        pub fn instantiate() -> Global<DummyAccount> {
            let (address_reservation, account_address) = Runtime::allocate_component_address(
                DummyAccount::blueprint_id()
            );

            (Self {
                account_address,
            })
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .with_address(address_reservation)
                .globalize()
        }

        pub fn address(&self) -> ComponentAddress {
            self.account_address
        }
    }
}
