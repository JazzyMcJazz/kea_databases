# Define the name of the builder stage
FROM rust:1.73 as builder

# Create a new empty shell project
RUN USER=root cargo new --bin rpg
WORKDIR /rpg

# Create directory for migration workspace
RUN mkdir /migration

# Copy our manifests into the project directory
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./migration ./migration

# Build our dependencies, this will be cached because we didn't
# copy our source code yet
RUN cargo build --release
RUN rm src/*.rs

# Now, copy our source code
COPY ./src ./src

# Build our application
RUN rm ./target/release/deps/rpg*
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `jazzshare_backend`.
FROM debian:bookworm-slim
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        libssl-dev \
        tzdata \
        libsqlite3-dev \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

ARG APP=/usr/local/bin
RUN mkdir -p ${APP}//public

COPY --from=builder /rpg/target/release/rpg ${APP}/rpg
COPY ./.env.docker /.env
COPY ./templates ${APP}/templates

WORKDIR ${APP}

CMD ["rpg"]