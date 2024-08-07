# thirtyfour_download_file

## download file from url

## init project

```bash
mkdir thirtyfour_download_file && cd $_
touch README.md
ln -s README.md README
cargo init .
cargo add rustfmt
rustup component add rustfmt
mkdir examples
cp src/main.rs examples/example.rs
# not understood
# sudo apt update
# sudo apt install libssl-dev pkg-config
# cargo install cargo-udeps
# cargo udeps --all-targets --backend depinfo
cargo add tokio --features full
cargo add thirtyfour
cargo update
```