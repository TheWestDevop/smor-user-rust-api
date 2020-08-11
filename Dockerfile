FROM rustlang/rust:nightly as builder

WORKDIR /smor_user

COPY .env .env
COPY Rocket.toml Rocket.toml
COPY diesel.toml diesel.toml

COPY . .

RUN cargo build --release

# RUN cargo install --path .

FROM debian:buster-slim

RUN mkdir smor_user

WORKDIR /smor_user

# install libpq
RUN apt-get update; \
    apt-get install -y --no-install-recommends libpq-dev; \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /smor_user/target/release/admin_service ./

COPY --from=builder /smor_user/Rocket.toml .
COPY --from=builder /smor_user/diesel.toml .
COPY --from=builder /smor_user/.env .


EXPOSE 8000

ENTRYPOINT [ "/smor_user/admin_service" ]