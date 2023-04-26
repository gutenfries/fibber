# build the application, using the official rust docker image
FROM rust:1.67 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .


# run the application, using a minimal debian image
FROM debian:bullseye-slim
# copy executable
COPY --from=builder /usr/local/cargo/bin/fibber /usr/local/bin/fibber
# give executable +x permissions
RUN chmod +x /usr/local/bin/fibber
# entrypoint for image
ENTRYPOINT ["/usr/local/bin/fibber"]
