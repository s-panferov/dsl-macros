This macro can be used with various builders that use closures for configuration. Macro rewrites only simple method calls without a path and leaves other expressions as-is. It do not touch calls like `builder.method()`, `module::method()` or `Class::method()`. Type parameters work as expected, so you can use `method<T>()` and the macro converts it to `context.method<T>()`.

## Сompatible libraries

1. [Rustless](https://github.com/rustless/rustless) — REST-like API micro-framework for Rust. Works with Iron.
2. [jsonway](https://github.com/rustless/jsonway) — JSON building DSL and configurable serializers for Rust.
3. [valico](https://github.com/rustless/valico) —  JSON Schema validator and JSON coercer.
4. Any other library that uses the same pattern for building blocks.

## Usage

**Source:**

```rust
dsl!(|context, other_arg1, other_arg2, /* .. */| {
    method_of_context(other_arg1);
    another_method_of_context(other_arg2);
})
```

**Result:**

```rust
|context, other_arg1, other_arg2, /* .. */| {
    context.method_of_context(other_arg1);
    context.another_method_of_context(other_arg2);
}
```

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