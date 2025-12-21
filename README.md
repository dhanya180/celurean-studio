# Celurean Studio

## Installation Instructions

### Pre-Requisites
- [CockroachDB](https://www.cockroachlabs.com/)
- [Redis](https://redis.io/)
- [Cargo](https://doc.rust-lang.org/cargo/)
- [Meson](https://mesonbuild.com/)

### Clone the Repo

```bash
git clone https://github.com/dhanya180/celurean-studio
cd celurean-studio
```

### Setup Cockroach DB
```bash
# start the process
 cockroach start-single-node \
  --insecure \
  --listen-addr=localhost:26257 \
  --http-addr=localhost:8081

# in separate terminal
cockroach sql --insecure --host=localhost
```

### Setup Black Channel
```bash
meson setup builddir
meson compile -C builddir
```

### Setup Crimson Heart
```bash
cd crimson_heart
cargo run 
```
