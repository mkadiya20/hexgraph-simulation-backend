FROM rust:latest as builder
WORKDIR /usr/src/server
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/server /usr/local/bin/server
CMD ["server"]

# FROM rust:latest as builder

# RUN USER=root cargo new --bin server
# WORKDIR ./server
# COPY ./Cargo.toml ./Cargo.toml
# RUN cargo build --release
# RUN rm src/*.rs

# ADD . ./

# RUN rm ./target/release/deps/server*
# RUN cargo build --release


# FROM debian:buster-slim
# ARG APP=/usr/src/app

# RUN apt-get update \
#     && apt-get install -y ca-certificates tzdata \
#     && rm -rf /var/lib/apt/lists/*

# EXPOSE 8000

# ENV TZ=Etc/UTC \
#     APP_USER=appuser

# RUN groupadd $APP_USER \
#     && useradd -g $APP_USER $APP_USER \
#     && mkdir -p ${APP}

# COPY --from=builder /server/target/release/server ${APP}/server

# RUN chown -R $APP_USER:$APP_USER ${APP}

# USER $APP_USER
# WORKDIR ${APP}

# CMD ["./server"]