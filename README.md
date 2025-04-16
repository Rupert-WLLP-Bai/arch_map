# Archmap

## Prerequisites

```bash
# Update package lists
sudo apt update

# Install dependencies
sudo apt install pkg-config libssl-dev postgresql postgresql-contrib

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install SQLx CLI
cargo install sqlx-cli
```

## Database Setup

```bash
# Set PostgreSQL password
sudo -u postgres psql -c "ALTER USER postgres PASSWORD 'password';"

# Create main database
sudo -u postgres psql -c "CREATE DATABASE arch_map;"
```

## Development

### Run Application

```bash
cargo run --bin arch_map
```

### Testing

```bash
# Create test database
sqlx database create --database-url postgres://postgres:password@localhost/arch_map_test

# Run migrations
sqlx migrate run --database-url postgres://postgres:password@localhost/arch_map_test

# Run tests
RUST_LOG=info DATABASE_URL="postgres://postgres:password@localhost/arch_map_test" cargo test
```