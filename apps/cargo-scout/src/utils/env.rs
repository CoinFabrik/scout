#![allow(dead_code)]

macro_rules! declare_const {
    ($var: ident) => {
        pub const $var: &str = stringify!($var);
    };
}

declare_const!(CARGO_HOME);
declare_const!(CARGO_MANIFEST_DIR);
declare_const!(CARGO_PKG_NAME);
declare_const!(CARGO_TARGET_DIR);
declare_const!(CARGO_TERM_COLOR);
declare_const!(CLIPPY_DISABLE_DOCS_LINKS);
declare_const!(CLIPPY_DRIVER_PATH);
declare_const!(DOCS_RS);
declare_const!(DYLINT_DRIVER_PATH);
declare_const!(DYLINT_LIBRARY_PATH);
declare_const!(DYLINT_LIBS);
declare_const!(DYLINT_LIST);
declare_const!(DYLINT_RUSTFLAGS);
declare_const!(DYLINT_TOML);
declare_const!(OUT_DIR);
declare_const!(PATH);
declare_const!(RUSTC);
declare_const!(RUSTC_WORKSPACE_WRAPPER);
declare_const!(RUSTFLAGS);
declare_const!(RUSTUP_HOME);
declare_const!(RUSTUP_TOOLCHAIN);
declare_const!(RUST_BACKTRACE);
declare_const!(TARGET);
