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