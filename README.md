# parser-rs

My learning rust project repo.

An attempt to make a [NSPredicate](https://developer.apple.com/documentation/foundation/nspredicate) style parser.

```rust
    let expr = parse("size > 100 and tags contains 'aardvark'").unwrap();
    println!("{:?}", expr.evaluate(my_object).unwrap());
```

## Notes

* `cargo` > `swift package`: swift package very very badly needs `swift package add <package name>`.
* crates.io > swiftpackageindex.com: spi has a clumsy name & url, doesn't contain any more useful info/functionality than a github page.
* development only dependencies are great and swift package desparetly needs them.
* no function overloading
* no default arguments
* rust error handling (result handling) is not as horrible as expected
* cannot extend types outside of main crate
* crates, modules, packages etc are just as confusing as python. Too fidly to express what you want to do.
* rust's ACL seems very crude compared to swift's
* `println!` is a mouthful and kind of horrible (`Swift.print` can handle anything you throw at it).

### Crates

Look into http://lalrpop.github.io instead of nom maybe?
