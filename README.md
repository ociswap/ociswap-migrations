# Token Migration

This Blueprint aims to transition tokens from Olympia to Babylon, leveraging all the advanced resource capabilities of the Radix Babylon Engine.

## Instantiation
To use it you need to create your own new token first (preferrable with the same total supply than your old token) and then call the `instantiate` function on the blueprint passing the `old_address` (Babylon address of your old token) and `new_token` (full new total supply):

```rust
pub fn instantiate(old_address: ResourceAddress, new_token: Bucket) -> Global<TokenMigration>
```
At instantiation the blueprint checks that the amount of `new_token` is greater than the total supply of the old token for additional safety.

If you have a mutable old token you should not mint or burn any of the old tokens after instantiating the `TokenMigration` blueprint. If you have minted more old tokens after instantiation you would need to create a second instance of the `TokenMigration` blueprint.

## Token Migration Swaps
To migrate and swap your old tokens to the new ones you can simply send a bucket of old tokens to the `swap` method on the instantiated `TokenMigration` component.

```rust
pub fn swap(old_token: Bucket) -> Bucket {
```

You get the new tokens returned then which you can deposit in your user's wallet in the transaction manifest.

## Ociswap Integration

For every Radix project we are offering to add their `TokenMigration` component to Ociswap website which would then automatically ask users after connecting their wallet whether they want to migrate their old tokens to the new ones. This will be implmented as a simple one-click solution to make the user journey as smooth as possible.

Please reach out to us either via `info@ociswap.com` or on [Telegram](https://t.me/ociswap) if you want us to add your own `TokenMigration` component to `ociswap.com`.