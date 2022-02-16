# fionread

This crate provides an abstraction over the `FIONREAD` ioctl. This is used to tell how many
bytes are in the read queue for a given socket.

This is an "unsafe-quarantine microcrate", since I want 
[`breadx`](https://github.com/bread-graphics/breadx) to be able to be `forbid(unsafe_code)`,
and it seems a little silly to throw that away for a single syscall that probably won't be
used that often in the common case. That's not to say that this may not be useful
elsewhere.

## Licensing

Licensed under the MIT and Apache 2.0 licenses, just like Rust proper is.