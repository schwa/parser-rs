# parser-rs

My learning rust project repo.

An attempt to make a [NSPredicate](https://developer.apple.com/documentation/foundation/nspredicate) style parser.

```rust
    let expr = parse("1 == 1 and 2 == 2").unwrap();
    println!("{:?}", expr.evaluate(my_object).unwrap());
```
