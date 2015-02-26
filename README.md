This macro can be used with various builders that use closures for configuration.

## Examples

**Without DSL**:

```rust
rustless::Namespace::build("tweets", |tweets| {
    tweets.get("latest", |endpoint| {
        endpoint.desc("Get latest tweets");
        endpoint.handle(|client, _params| {
            // body omitted
        })
    });

    tweet.post(/* omitted */);
    tweet.delete(/* omitted */);
})
```

**With DSL**:

```rust
rustless::Namespace::build("tweets", dsl!(|tweets| {
    get("latest", dsl!(|endpoint| {
        desc("Get latest tweets");
        handle(|client, _params| {
            // body omitted
        })
    }));

    post(/* omitted */);
    delete(/* omitted */);
}))
```