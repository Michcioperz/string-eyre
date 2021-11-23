# string-eyre

Has this happened to you?

```
error[E0599]: the method `wrap_err` exists for enum `Result<(), tauri::Error>`, but its trait bounds were not satisfied
   --> src/main.rs:60:6
    |
60  |     .wrap_err("error while running tauri application")?;
    |      ^^^^^^^^
    |
   ::: /home/michcioperz/.cargo/registry/src/github.com-1ecc6299db9ec823/tauri-1.0.0-beta.8/src/error.rs:10:1
    |
10  | pub enum Error {
    | --------------
    | |
    | doesn't satisfy `tauri::Error: Sync`
    | doesn't satisfy `tauri::Error: eyre::context::ext::StdError`
    |
   ::: /home/michcioperz/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/result.rs:503:1
    |
503 | pub enum Result<T, E> {
    | --------------------- doesn't satisfy `_: WrapErr<(), tauri::Error>`
    |
    = note: the following trait bounds were not satisfied:
            `tauri::Error: eyre::context::ext::StdError`
            which is required by `Result<(), tauri::Error>: WrapErr<(), tauri::Error>`
            `tauri::Error: Sync`
            which is required by `Result<(), tauri::Error>: WrapErr<(), tauri::Error>`

For more information about this error, try `rustc --explain E0599`.
error: could not compile `app` due to previous error
```

Sometimes you feel very tired and decide that, okay, perhaps I will need to stringify the error:

```rust
    .map_err(|e| e.to_string())
```

And that works okay, unless the error doesn't stringify, at which point you grow angrier and just:

```rust
    .map_err(|e| format!("{:?}", e))
```

But String isn't actually an error type, so eyre rightfully doesn't see it as such, so you actually want:

```rust
    .map_err(|e| eyre!("{:?}", e))
```

And probably more people should scream at me, but I'd like to have shorthands for this where I don't type this heresy over and over again, but instead write:

```rust
    .debug_to_eyre()
```
