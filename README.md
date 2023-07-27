# parser-rs

My learning rust project repo.

An attempt to make a [NSPredicate](https://developer.apple.com/documentation/foundation/nspredicate) style parser.

```rust
    let expr = parse("size > 100 and tags contains 'aardvark'").unwrap();
    println!("{:?}", expr.evaluate(my_object).unwrap());
```
