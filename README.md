# fionread

[![crates.io][crates-badge]][crates-url]
[![Documentation][docs-badge]][docs-url]
[![Build Status][build-badge]][build-url]

[build-badge]: https://img.shields.io/github/workflow/status/notgull/fionread/CI
[build-url]: https://github.com/notgull/fionread/actions?query=workflow%3ACI+branch%3Amaster
[docs-badge]: https://img.shields.io/docsrs/fionread
[docs-url]: https://docs.rs/fionread
[crates-badge]: https://img.shields.io/crates/v/fionread
[crates-url]: https://crates.io/crates/fionread

This crate provides an abstraction over the `FIONREAD` ioctl. This is used to tell how many
bytes are in the read queue for a given socket.

This is an "unsafe-quarantine microcrate", since I want 
[`breadx`](https://github.com/bread-graphics/breadx) to be able to be `forbid(unsafe_code)`,
and it seems a little silly to throw that away for a single syscall that probably won't be
used that often in the common case. That's not to say that this may not be useful
elsewhere.

MSRV is currently 1.46.0. This MSRV will not change without a minor version bump.

## License

This package is distributed under the Boost Software License Version 1.0.
Consult the [LICENSE](./LICENSE) file or consult the [web mirror] for
more information.

[web mirror]: https://www.boost.org/LICENSE_1_0.txt