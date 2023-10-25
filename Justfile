set dotenv-load

docker_compose_bin := "docker compose -f compose.dev.yaml"
sqlx_bin := "~/.cargo/bin/sqlx"
cargo_bin := "cargo"

start:
    {{docker_compose_bin}} up -d --wait

stop:
    {{docker_compose_bin}} down

run:
    {{cargo_bin}} run
