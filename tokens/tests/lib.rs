mod helper;

#[cfg(test)]
mod token_swap {
    use scrypto::*;
    use scrypto_test::prelude::*;
    use super::*;
    use helper::*;

    #[test]
    #[ignore = "TODO"]
    fn test_instantiate_same_address() -> Result<(), RuntimeError> {
        Ok(())
    }

    #[test]
    #[ignore = "TODO"]
    fn test_instantiate_not_enough_supply() -> Result<(), RuntimeError> {
        Ok(())
    }

    #[test]
    fn test_token_swap() -> Result<(), RuntimeError> {
        let mut env = EnvDefault::new()?;
        let old_token = env.old_token.take(dec!(10), &mut env.env)?;
        let output = env.token_swap.swap(old_token, &mut env.env)?;

        env.assert_bucket_eq(&output, env.new_address, dec!(10))?;

        Ok(())
    }

    #[test]
    #[should_panic]
    fn test_token_swap_too_much() {
        let mut env = EnvDefault::new().unwrap();
        let old_token = env.old_token.take(dec!(1001), &mut env.env).unwrap();
        let _ = env.token_swap.swap(old_token, &mut env.env);
    }

    #[test]
    fn test_token_swap_multiple() -> Result<(), RuntimeError> {
        let mut env = EnvDefault::new()?;
        let old_token = env.old_token.take(dec!(10), &mut env.env)?;
        let output = env.token_swap.swap(old_token, &mut env.env)?;

        env.assert_bucket_eq(&output, env.new_address, dec!(10))?;

        let old_token = env.old_token.take(dec!(10), &mut env.env)?;
        let output = env.token_swap.swap(old_token, &mut env.env)?;

        env.assert_bucket_eq(&output, env.new_address, dec!(10))?;

        Ok(())
    }
}
