mod helper;

#[cfg(test)]
mod token_migration {
    use scrypto::*;
    use scrypto_test::prelude::*;
    use super::*;
    use helper::*;

    #[test]
    fn test_instantiate() {
        let mut helper = MigrationHelper::new().unwrap();
        let new_token = helper.y_token.take(dec!(1000), &mut helper.env).unwrap();
        helper.instantiate(helper.x_address, new_token).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_instantiate_new_amount_lesser_old_supply() {
        let mut helper = MigrationHelper::new().unwrap();
        let new_token = helper.y_token.take(dec!(999), &mut helper.env).unwrap();
        helper.instantiate(helper.x_address, new_token).unwrap()
    }

    #[test]
    #[should_panic]
    fn test_instantiate_new_amount_greater_old_supply() {
        let mut helper = MigrationHelper::new().unwrap();
        let new_token = helper.y_token.take(dec!(1001), &mut helper.env).unwrap();
        helper.instantiate(helper.x_address, new_token).unwrap()
    }

    #[test]
    fn test_swap() -> Result<(), RuntimeError> {
        let mut helper = MigrationHelper::new()?;
        helper.instantiate_default()?;
        let old_token = helper.x_token.take(dec!(10), &mut helper.env)?;
        let output = helper.token_migration.unwrap().swap(old_token, &mut helper.env)?;

        helper.assert_bucket_eq(&output, helper.y_address, dec!(10))?;

        Ok(())
    }

    #[test]
    fn test_swap_multiple() -> Result<(), RuntimeError> {
        let mut helper = MigrationHelper::new()?;
        helper.instantiate_default()?;
        let old_token = helper.x_token.take(dec!(10), &mut helper.env)?;
        let output = helper.token_migration.unwrap().swap(old_token, &mut helper.env)?;

        helper.assert_bucket_eq(&output, helper.y_address, dec!(10))?;

        let old_token = helper.x_token.take(dec!(10), &mut helper.env)?;
        let output = helper.token_migration.unwrap().swap(old_token, &mut helper.env)?;

        helper.assert_bucket_eq(&output, helper.y_address, dec!(10))?;

        Ok(())
    }

    #[test]
    fn test_swap_all() -> Result<(), RuntimeError> {
        let mut helper = MigrationHelper::new()?;
        helper.instantiate_default()?;
        let old_token = helper.x_token.take(dec!(1000), &mut helper.env)?;
        let output = helper.token_migration.unwrap().swap(old_token, &mut helper.env)?;

        helper.assert_bucket_eq(&output, helper.y_address, dec!(1000))?;

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_swap_with_new_token() {
        let mut helper = MigrationHelper::new().unwrap();
        helper.instantiate_default().unwrap();
        let new_token = helper.y_token.take(dec!(10), &mut helper.env).unwrap();
        let _ = helper.token_migration.unwrap().swap(new_token, &mut helper.env).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_swap_with_other_token() {
        let mut helper = MigrationHelper::new().unwrap();
        helper.instantiate_default().unwrap();
        let other_token = helper.z_token.take(dec!(10), &mut helper.env).unwrap();
        let _ = helper.token_migration.unwrap().swap(other_token, &mut helper.env).unwrap();
    }
}
