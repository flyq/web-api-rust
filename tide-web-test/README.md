# tide-web-test

Failed

the error message:
```shell
Compiling futures-io-preview v0.3.0-alpha.19 (registry `https://mirrors.ustc.edu.cn/crates.io-index/`)
error[E0106]: missing lifetime specifier
   --> /home/flyq/.cargo/registry/src/mirrors.ustc.edu.cn-07d6cef1b9e54f94/futures-io-preview-0.3.0-alpha.19/src/lib.rs:292:28
    |
292 |             -> Poll<Result<&[u8]>>;
    |                            ^ expected lifetime parameter
    |
    = help: this function's return type contains a borrowed value, but the signature does not say which one of argument 2's 2 lifetimes it is borrowed from

error[E0106]: missing lifetime specifier
   --> /home/flyq/.cargo/registry/src/mirrors.ustc.edu.cn-07d6cef1b9e54f94/futures-io-preview-0.3.0-alpha.19/src/lib.rs:559:32
    |
559 |                 -> Poll<Result<&[u8]>>
    |                                ^ expected lifetime parameter
...
571 |         deref_async_buf_read!();
    |         ------------------------ in this macro invocation
    |
    = help: this function's return type contains a borrowed value, but the signature does not say which one of `cx`'s 2 lifetimes it is borrowed from

error[E0106]: missing lifetime specifier
   --> /home/flyq/.cargo/registry/src/mirrors.ustc.edu.cn-07d6cef1b9e54f94/futures-io-preview-0.3.0-alpha.19/src/lib.rs:559:32
    |
559 |                 -> Poll<Result<&[u8]>>
    |                                ^ expected lifetime parameter
...
575 |         deref_async_buf_read!();
    |         ------------------------ in this macro invocation
    |
    = help: this function's return type contains a borrowed value, but the signature does not say which one of `cx`'s 2 lifetimes it is borrowed from

error[E0106]: missing lifetime specifier
   --> /home/flyq/.cargo/registry/src/mirrors.ustc.edu.cn-07d6cef1b9e54f94/futures-io-preview-0.3.0-alpha.19/src/lib.rs:584:28
    |
584 |             -> Poll<Result<&[u8]>>
    |                            ^ expected lifetime parameter
    |
    = help: this function's return type contains a borrowed value, but the signature does not say which one of `cx`'s 2 lifetimes it is borrowed from

error[E0106]: missing lifetime specifier
   --> /home/flyq/.cargo/registry/src/mirrors.ustc.edu.cn-07d6cef1b9e54f94/futures-io-preview-0.3.0-alpha.19/src/lib.rs:597:32
    |
597 |                 -> Poll<Result<&[u8]>>
    |                                ^ expected lifetime parameter
...
609 |         delegate_async_buf_read_to_stdio!();
    |         ------------------------------------ in this macro invocation
    |
    = help: this function's return type contains a borrowed value, but the signature does not say which one of argument 2's 2 lifetimes it is borrowed from

error[E0106]: missing lifetime specifier
   --> /home/flyq/.cargo/registry/src/mirrors.ustc.edu.cn-07d6cef1b9e54f94/futures-io-preview-0.3.0-alpha.19/src/lib.rs:597:32
    |
597 |                 -> Poll<Result<&[u8]>>
    |                                ^ expected lifetime parameter
...
613 |         delegate_async_buf_read_to_stdio!();
    |         ------------------------------------ in this macro invocation
    |
    = help: this function's return type contains a borrowed value, but the signature does not say which one of argument 2's 2 lifetimes it is borrowed from

error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0106`.
error: Could not compile `futures-io-preview`.
warning: build failed, waiting for other jobs to finish...
error: build failed
```