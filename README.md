# Token Migration

This [TokenMigration](token_migration/src/lib.rs) blueprint aims to transition tokens from Olympia to Babylon, leveraging all the advanced resource capabilities of the Radix Babylon Engine (for further details refer to [Metadata Standard](https://docs-babylon.radixdlt.com/main/standards/metadata-standard-introduction.html)).

## Instantiation
To use it you need to create your own new token first (preferrable with the same total supply than your old token) and then call the `instantiate` function on the blueprint passing the `old_address` (Babylon address of your old token - every Olympia token will have a new address on Bablyon) and `new_token` (full new total supply):

```rust
pub fn instantiate(old_address: ResourceAddress, new_token: Bucket) -> Global<TokenMigration>
```
At instantiation the blueprint checks that the amount of `new_token` bucket provided is equal to the total supply of the old token for additional safety.

If you have a mutable old token you should not mint or burn any of the old tokens after instantiating the `TokenMigration` blueprint. If you have minted more old tokens after instantiation you would need to create another new instance of the `TokenMigration` blueprint for the same addresses.

## Token Migration
To swap your old tokens to the new ones you can simply send a bucket of old tokens to the `swap` method on the instantiated `TokenMigration` component.

```rust
pub fn swap(old_token: Bucket) -> Bucket
```

You get the new tokens returned then which you need to deposit in your user's wallet in the transaction manifest.

## Ociswap Integration

For every Radix project we are offering to add their `TokenMigration` component to Ociswap website which would then automatically ask users after connecting their wallet whether they want to migrate their old tokens to the new ones. This will be implemented as a simple one-click solution to make the user journey as smooth as possible.

Please reach out to us either via `info@ociswap.com` or on [Telegram](https://t.me/ociswap) if you want us to add your own `TokenMigration` component to `ociswap.com`.

## Disable Total Supply Validation
In some cases, there may be a need to migrate to a new token with a reduced total supply. For instance, when you have 'burned' tokens from the old supply by sending them to a permanent lock address
To accommodate such scenarios, you can utilize the following 'instantiate' method:
```rust
pub fn instantiate_without_supply_validation(
    old_address: ResourceAddress,
    new_token: Bucket
) -> Global<TokenMigration>
```
However, if you're uncertain about whether this method is suitable for your project, it's advisable to opt for the standard `instantiate` method. You'll realize the need for the former when it's necessary.

Exercise caution when using this method, as it could potentially result in a situation where not everyone can migrate to the new token due to a higher number of old tokens still in circulation compared to the new tokens you've introduced. In such cases, deploying another migration contract may be required."