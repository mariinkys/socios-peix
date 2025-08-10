FROM rustlang/rust:nightly-bookworm as builder

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && cp cargo-binstall /usr/local/cargo/bin \
    && rm cargo-binstall-x86_64-unknown-linux-musl.tgz

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends clang libssl-dev pkg-config npm binaryen \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

RUN cargo binstall cargo-leptos -y

RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

RUN npm install -g sass \
    && npm install

RUN RUSTFLAGS="--cfg erase_components" cargo leptos build --release -vv

FROM debian:bookworm-slim as runner
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/socios-peix /app/

COPY --from=builder /app/target/site /app/site

COPY --from=builder /app/Cargo.toml /app/

ENV RUST_LOG="info"
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="./site"
ENV DATABASE_URL="sqlite:/app/db/socios.db"
ENV FROM_NAME="From Email Name"
ENV SMTP_PASSWORD="SMPTPassword"
ENV SMTP_USERNAME="myemail@gmail.com"

EXPOSE 8080

RUN mkdir -p /app/db
VOLUME /app/db

CMD ["/app/socios-peix"]
