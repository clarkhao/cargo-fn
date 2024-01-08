# install
rustup update
rustup component list
# formatting, linting and test
cargo fmt
cargo clippy --fix
cargo test
# documenting
#[doc = "description"]
cargo doc --no-deps --open
# update crates used
cargo update
or manually change version inside Cargo.toml
# about Cargo.locak
if library, don't commit
if exucutable, commit it
# test library locally
[dependencies]
hello-fn = {path = "../hello-fn"}
# build
cargo build --verbose
# library
cargo package
cargo package --list
cargo login token
cargo publish