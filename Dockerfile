FROM rustlang/rust:nightly-bookworm as builder

RUN wget https://github.com/cargo-bins/cargo-binstall/releases/latest/download/cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && tar -xvf cargo-binstall-x86_64-unknown-linux-musl.tgz \
    && cp cargo-binstall /usr/local/cargo/bin \
    && rm cargo-binstall-x86_64-unknown-linux-musl.tgz

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends clang libssl-dev pkg-config npm binaryen \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Download latest Binaryen release from GitHub and install it
RUN BINARYEN_VERSION=$(curl -s https://api.github.com/repos/WebAssembly/binaryen/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/') \
    && wget https://github.com/WebAssembly/binaryen/releases/download/${BINARYEN_VERSION}/binaryen-${BINARYEN_VERSION}-x86_64-linux.tar.gz \
    && tar -xzf binaryen-${BINARYEN_VERSION}-x86_64-linux.tar.gz \
    && cp -r binaryen-${BINARYEN_VERSION}/bin/* /usr/local/bin/ \
    && rm -rf binaryen-${BINARYEN_VERSION} binaryen-${BINARYEN_VERSION}-x86_64-linux.tar.gz

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
# Please change the secret key don't leave the default one
ENV SECRET_KEY="da5ff4ed8bd415a6a99a313d54de591503b762ed33a10c98b3d8d09293ed0adaabd70e9da8721e3c4b4e1a0ec842790e159e5cd14485811945030c58c5de2467"

EXPOSE 8080

RUN mkdir -p /app/db
VOLUME /app/db

CMD ["/app/socios-peix"]
