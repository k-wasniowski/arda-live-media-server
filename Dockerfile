FROM lukemathwalker/cargo-chef:latest-rust-1.66.1 as chef

WORKDIR /home/app

RUN apt update && apt install lld clang -y

FROM chef as planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder

COPY --from=planner /home/app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release

FROM  debian:bullseye-slim AS runtime

WORKDIR /home/app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /home/app/target/release/arda-live-media-server arda-live-media-server
COPY configuration /home/app/configuration

ENV APP_ENVIRONMENT production
ENTRYPOINT ["/home/app/arda-live-media-server"]