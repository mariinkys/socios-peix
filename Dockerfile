# Get started with a build env with Rust nightly
FROM rustlang/rust:nightly-alpine as builder

RUN apk update && \
    apk add --no-cache bash curl npm libc-dev binaryen openssl-dev

RUN npm install -g sass

RUN curl --proto '=https' --tlsv1.2 -LsSf https://github.com/leptos-rs/cargo-leptos/releases/latest/download/cargo-leptos-installer.sh | sh
#RUN cargo install --locked cargo-leptos@0.2.41

# Add the WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /work
COPY . .

RUN npm install

RUN cargo leptos build --release -vv

FROM rustlang/rust:nightly-alpine as runner

WORKDIR /app

COPY --from=builder /work/target/release/socios-peix /app/
COPY --from=builder /work/target/site /app/site
COPY --from=builder /work/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT=./site
ENV DATABASE_URL="sqlite:/app/db/socios.db"
ENV FROM_NAME="From Email Name"
ENV SMTP_PASSWORD="SMPTPassword"
ENV SMTP_USERNAME="myemail@gmail.com"
EXPOSE 8080

# Create a directory for the database
RUN mkdir -p /app/db

# Use VOLUME to mark the database directory as a mount point
VOLUME /app/db

CMD ["/app/socios-peix"]