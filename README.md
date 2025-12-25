# Celurean-Studio

## Crimson Heart
- Very Fast, Asynchronous Control Server

## Installation Instructions

### Pre-Requisites
- [CockroachDB](https://www.cockroachlabs.com/)
- [Redis](https://redis.io/)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Meson](https://mesonbuild.com/)
- [Grafana](https://grafana.com/)

### Clone the Repo

```bash
git clone https://github.com/dhanya180/celurean-studio
cd celurean-studio
```

**Six Terminal Sessions are Required**

### Cockroach DB
- Start
```bash
 cockroach start-single-node \
  --insecure \
  --listen-addr=localhost:26257 \
  --http-addr=localhost:8081
```
- For SQL Queries
```bash
cockroach sql --insecure --host=localhost

# use for deleting table 
TRUNCATE table_name
```

### Redis Server
- Start
```bash
redis-server
```
- Monitor
```bash
redis-cli

# for viewing HSET keys
KEYS *

# for flushing 
FLUSHDB
```

### Loki
- Start
```bash
loki -config.file=./config/loki.yml
```

- Monitor
```bash
grafana-server

# use grafana server to check loki logs
```

### Spin up Crimson Heart (Server)
- Make sure `.env` file exists & is filled with valid information. 
```bash
cd crimson_heart
cargo run 
```
#### [Benchmarking](./bench/Bench.md)

### Setup Black Channel
```bash
meson setup builddir
meson compile -C builddir
```