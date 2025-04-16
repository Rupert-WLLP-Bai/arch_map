# Archmap

## require

sudo apt update
sudo apt install pkg-config libssl-dev 

rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sqlx: cargo install sqlx-cli
postgresql: sudo apt install postgresql postgresql-contrib

## setup db
sudo -u postgres psql -c "ALTER USER postgres PASSWORD 'password';"
sudo -u postgres psql -c "CREATE DATABASE arch_map;"

## run
cargo run --bin arch_map


sqlx database create --database-url postgres://postgres:password@localhost/arch_map_test
sqlx migrate run --database-url postgres://postgres:password@localhost/arch_map_test
RUST_LOG=info DATABASE_URL="postgres://postgres:password@localhost/arch_map_test" cargo test