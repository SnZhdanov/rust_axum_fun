# FROM rust:1.75.0 as build
# RUN USER=root cargo new --bin rust_up_in_here
# WORKDIR /rust_up_in_here

# # copy over your manifests
# COPY ./Cargo.lock ./Cargo.lock
# COPY ./Cargo.toml ./Cargo.toml

# # this build step will cache your dependencies
# RUN cargo build --release
# RUN rm src/*.rs

# COPY ./src ./src

# # build for release
# RUN rm ./target/release/deps/rust_up_in_here*
# RUN cargo build --release

# COPY --from=build /rust_up_in_here/target/release/rust_up_in_here .

# CMD ["./rust_up_in_here"]


FROM rust:latest

WORKDIR /
COPY . .

RUN cargo build

CMD ["cargo", "run"]



# FROM rust:latest

# WORKDIR /

# COPY ./Cargo.lock ./Cargo.lock
# COPY ./Cargo.toml ./Cargo.toml

# # this build step will cache your dependencies
# RUN cargo build --release
# RUN rm src/*.rs

# RUN rm ./target/release/deps/rust_up_in_here*
# RUN cargo build --release

# COPY --from=build /rust_up_in_here/target/release/rust_up_in_here .

# CMD ["./rust_up_in_here"]

