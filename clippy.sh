#!/bin/sh

# the following line is an workaround, because of: https://github.com/rust-lang/rust-clippy/issues/4612
find . -type f | grep "\.rs$" | xargs touch -c --
CLIPPY_DISABLE_DOCS_LINKS=1 cargo clippy --all-features -Z unstable-options "$@" -- -D clippy::correctness -W clippy::style -W clippy::complexity -W clippy::perf -A clippy::needless_return -D clippy::implicit_return
