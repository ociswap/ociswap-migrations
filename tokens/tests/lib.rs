mod helper;

#[cfg(test)]
mod token_swap {
    use scrypto::*;
    use scrypto_test::prelude::*;
    use super::*;
    use helper::*;

    #[test]
    fn test_token_swap() -> Result<(), RuntimeError> {
        let mut env = EnvDefault::new()?;
        let old_token = env.old_token.take(dec!(10), &mut env.env)?;
        let (output, remainder) = env.token_swap.swap(old_token, &mut env.env)?;

        env.assert_bucket_eq(&output, env.new_address, dec!(10))?;
        env.assert_bucket_eq(&remainder, env.old_address, dec!(0))?;

        Ok(())
    }

    #[test]
    fn test_token_swap_with_remainder() -> Result<(), RuntimeError> {
        let mut env = EnvDefault::new()?;
        let old_token = env.old_token.take(dec!(1001), &mut env.env)?;
        let (output, remainder) = env.token_swap.swap(old_token, &mut env.env)?;

        env.assert_bucket_eq(&output, env.new_address, dec!(1000))?;
        env.assert_bucket_eq(&remainder, env.old_address, dec!(1))?;

        Ok(())
    }

    #[test]
    fn test_token_swap_deposit() -> Result<(), RuntimeError> {
        let mut env = EnvDefault::new()?;
        env.token_swap.deposit(env.new_token.take(dec!(1), &mut env.env)?, &mut env.env)?;
        let old_token = env.old_token.take(dec!(1001), &mut env.env)?;
        let (output, remainder) = env.token_swap.swap(old_token, &mut env.env)?;

        env.assert_bucket_eq(&output, env.new_address, dec!(1001))?;
        env.assert_bucket_eq(&remainder, env.old_address, dec!(0))?;

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_token_swap_withdraw_unauthorized() {
        let mut env = EnvDefault::new().unwrap();
        let old_token = env.old_token.take(dec!(1000), &mut env.env).unwrap();
        let _ = env.token_swap.swap(old_token, &mut env.env).unwrap();
        let new_token = env.token_swap.withdraw(dec!(50), &mut env.env).unwrap();
        env.assert_bucket_eq(&new_token, env.new_address, dec!(1001)).unwrap();
    }

    #[test]
    #[ignore = "Missing authentication"]
    fn test_token_swap_withdraw_authorized() -> Result<(), RuntimeError> {
        let mut env = EnvDefault::new()?;
        let old_token = env.old_token.take(dec!(1000), &mut env.env)?;
        let _ = env.token_swap.swap(old_token, &mut env.env)?;
        // TODO authenticate
        let new_token = env.token_swap.withdraw(dec!(50), &mut env.env)?;
        env.assert_bucket_eq(&new_token, env.new_address, dec!(1001))?;

        Ok(())
    }

    #[test]
    fn test_token_swap_multiple() -> Result<(), RuntimeError> {
        let mut env = EnvDefault::new()?;
        let old_token = env.old_token.take(dec!(10), &mut env.env)?;
        let (output, remainder) = env.token_swap.swap(old_token, &mut env.env)?;

        env.assert_bucket_eq(&output, env.new_address, dec!(10))?;
        env.assert_bucket_eq(&remainder, env.old_address, dec!(0))?;

        let old_token = env.old_token.take(dec!(10), &mut env.env)?;
        let (output, remainder) = env.token_swap.swap(old_token, &mut env.env)?;

        env.assert_bucket_eq(&output, env.new_address, dec!(10))?;
        env.assert_bucket_eq(&remainder, env.old_address, dec!(0))?;

        let old_token = env.old_token.take(dec!(981), &mut env.env)?;
        let (output, remainder) = env.token_swap.swap(old_token, &mut env.env)?;

        env.assert_bucket_eq(&output, env.new_address, dec!(980))?;
        env.assert_bucket_eq(&remainder, env.old_address, dec!(1))?;

        Ok(())
    }
}
