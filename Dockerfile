FROM shogan/rust-musl-action:1.0.2

COPY . .

RUN cargo build --target x86_64-apple-darwin --release