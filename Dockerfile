# FROM rust:alpine as Builder

# WORKDIR /

# COPY . .

# RUN apk add musl-dev
# RUN cargo build --release

# FROM alpine

# COPY --from=builder /build/target/release/restaurant_app /usr/bin/restaurant_app

# EXPOSE 8080
# EXPOSE 27017

# ENTRYPOINT "/usr/bin/restaurant_app"
