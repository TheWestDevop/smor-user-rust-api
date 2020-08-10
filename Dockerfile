FROM rustlang/rust:nightly as builder

WORKDIR /smor_user

COPY .env .env
COPY Rocket.toml Rocket.toml
COPY diesel.toml diesel.toml
COPY secret.key secret.key
COPY . .

# RUN rustup default nightly

# RUN rustup override set nightly
# ENV DATABASE_URL = "postgres://xwfvbwhp:qRldz9EOmZFy-1BgUx9hWOu_GXFhoag8@ruby.db.elephantsql.com:5432/xwfvbwhp"

RUN cargo build --release

# RUN cargo install --path .

FROM debian:buster-slim

RUN mkdir smor_user

WORKDIR /smor_user

# ENV DATABASE_URL = postgres://xwfvbwhp:qRldz9EOmZFy-1BgUx9hWOu_GXFhoag8@ruby.db.elephantsql.com:5432/xwfvbwhp

# install libpq
RUN apt-get update; \
    apt-get install -y --no-install-recommends libpq-dev; \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /smor_user/target/release/admin_service ./

COPY --from=builder /smor_user/Rocket.toml .
COPY --from=builder /smor_user/diesel.toml .
COPY --from=builder /smor_user/secret.key .
COPY --from=builder /smor_user/.env .


EXPOSE 8001

ENTRYPOINT [ "/smor_user/admin_service" ]