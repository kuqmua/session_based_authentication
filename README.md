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
curl -v http://127.0.0.1:8000/subscribe -i -X POST -d 'email=thomas_miiiann@hotmail.com&name=Tomy'

### how to install remove unused dependencies tool
cargo install cargo-udeps
usage:
cargo +nightly udeps

### how to install logs formatter?
cargo install bunyan