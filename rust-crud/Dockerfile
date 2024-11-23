FROM rust:1.73-buster as bulider
WORKDIR /app
# build
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
COPY . .
RUN cargo build --release
# Production
FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY --from=bulider /app/target/release/rust-crud .
CMD [ "./rust-crud" ]
