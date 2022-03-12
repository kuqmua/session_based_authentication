# session_based_authentication
by tutorial https://www.lpalmieri.com/posts/session-based-authentication-in-rust/

### start deleopment 
cargo watch -x check -x test -x "run"

### launch Postgres
sudo ./scripts/init_db.sh
(sudo coz got pesmission denied error)

### install sqlx-cli
cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres

### example sqlx migration 
sqlx migrate add create_subscriptions_table

### how to use logger
log provides five macros: trace, debug, info, warn and error.
RUST_LOG=debug cargo run, for example, will surface all logs at debug-level or higher emitted by our application or the crates we are using. RUST_LOG=session_based_authentication, instead, would filter out all records emitted by our dependencies.

### subscribe route test (change email and name)
curl -i -X POST -d 'email=thomas_mann@hotmail.com&name=Tom'  http://127.0.0.1:8000/subscriptions

### how to install remove unused dependencies tool
cargo install cargo-udeps
usage:
cargo +nightly udeps

### how to install logs formatter?
cargo install bunyan

### docker build 
sudo docker build --tag session_based_authentication --file Dockerfile .

### Generate query metadata to support offline compile-time verification.
sqlx prepare
or 
cargo sqlx prepare -- --lib
(to use generated .json query data - env var SQLX_OFFLINE must be true)

### run docker container
sudo docker run -p 8000:8000 session_based_authentication

### smaller rust docker builds
We could go even smaller by using rust:1.59.0-alpine, but we would have to cross-compile to the linux-musl target - out of scope for now. Check out rust-musl-builder if you are interested in generating tiny Docker images.
Another option to reduce the size of our binary further is stripping symbols from it - you can find more information about it here.

### ignore Digital Ocean for now

### property-based testing
There are two mainstream options for property-based testing in the Rust ecosystem: quickcheck and proptest.