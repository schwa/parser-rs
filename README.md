# parser-rs

My learning rust project repo.

An attempt to make an [NSPredicate](https://developer.apple.com/documentation/foundation/nspredicate) style parser.

```rust
    let expr = parse("size > 100 and tags contains 'aardvark'").unwrap();
    println!("{:?}", expr.evaluate(my_object).unwrap());
```

## Notes

* The VS-Code copilot plugin makes writing code in a language you are not fluent in easy. This really is an amazing tool for learning new languages (or working in languages you perpetually forget the syntax of - hello shell scripts).
* `cargo` > `swift package`: Swift Package Manager very very badly needs `swift package add <package name>`.
* crates.io > swiftpackageindex.com: spi has a clumsy name and URL and doesn't contain any more useful info/functionality than a GitHub page.
* development only dependencies are great and Swift Package Manager needs this feature.
* no function overloading ('evaluate', 'evaluate2', 'yetAnotherEvalulate``)
* no default arguments (makes no function overloading worse)
* no named parameters (makes function names overly verbose)
* Rust error handling (result handling) is not as horrible as expected
* cannot extend types outside of the main crate. Often need to wrap basic types to extend them (workaround: use traits instead).
* crates, modules, packages etc are just as confusing as Python. Too fiddly to express what you want to do.
* rust's ACL seems very crude compared to what we have in Swift.
* `println!` is a mouthful and kind of horrible (`Swift.print` can handle anything you throw at it).
* `panic!` is fun to call (I noticed there's an `unimplemented!()` too - something which should be in Wwift)
* no built-in code coverage. Very painful to set-up with llvm-cov and shell scripts to extract paths.
* doc-comments would be useful but scope rules prevent them from being easy to use.
* The attribute system seems extremely advance with fine-grained control to enable what code is compiled where.
* The `derive` system is super useful - but makes explicit something we get for free in Swift.
* Embedded Rust seems like it could give Micropython/Circuitpython a run for its money. no-std is something that would be difficult to add to Swift.
* String vs &str and oh my. So much `.to_string()` silliness.
* Memory management is a big learning curve and not sure the gain is worth it outside embedded/OS work. If you're replacing Python with Rust what's the point?
* Rust lifetime attributes impose a lot of complication on-to the users.
* Oh, but we have smart pointers and reference-counting anyway. But don't use them! Unless you have to.
* It's nice using the same asserts in unit tests as you would elsewhere. Muscle memory wants to use XCT prefixes.
* I keep trying to use `rust run` instead of `cargo run`.
* VS Code integration is of course amazing.
* The small amount of async I used (in another repo) was surprisingly painless but was weird having to bring in dependencies (`tokio`) to do it.
* `match`` is kind of ugly compared to Swift switch. `_ =>` instead of `default:` is rather unfortunate.
* In general Swift is far more user-friendly than Rust. Rust syntax is very fiddly and I wouldn't have been able to get anywhere without VS-Code/copilot offering me good suggestions.
* Closure-syntax is weird too.
* Trying to return a closure from a function is difficult. A box is required, sometimes it isn't? Sometimes you need a `dyn` keyword. I dunno. Swift 'A -> B -> C` is a breeze in comparison.
* snake_case. What is this? The 1940s?
* Between stuff like no named parameters, snake_case and all those `to_string`, `unwrap` and so on the code gets very verbose very quickly.
* if you have to have not one but two tables explaining all the modifiers on `Result` (<https://doc.rust-lang.org/std/result/>) (`and`, `or` etc etc) then maybe something is wrong somewhere.
* Swift's auto predicate gets rid of a lot of silliness like `and` and `and_else` naming nonsense.
* Again, Swift feels much more ergonomic.

### Crates

* Look into <http://lalrpop.github.io> instead of nom maybe?
