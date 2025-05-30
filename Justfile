set unstable  # enable unstable features if needed
default:
    @echo "Available command"
    @echo "  just build               # Compile entire Rust workspace"
    @echo "  just run-<crate>         # run a specific crate binary"
    @echo "  just db                  # connect to CockroachDB"
    @echo "  just schema check        # Validate SQLx macros with DATABASE_URL_COCKROACH"
    @echo "  just help                # show this message"
# Justfile

# Runs only on the host
dev-host:
    ./dev.sh

# Runs only inside the container
dev:
    cargo build

