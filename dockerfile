FROM rust:latest AS build

WORKDIR /usr/src/bocchi

COPY . .

RUN cargo build --release

FROM ubuntu:latest

RUN apt-get update && \
    apt-get install -y \
        python3 \
        python3-pip \
        libgcc1 \
        sqlite3 \
        nginx

RUN rm -rf /var/lib/apt/lists/*
RUN pip3 install --break-system-packages yt-dlp
RUN mkdir -p /etc/nginx/html && chmod 777 /etc/nginx/html

WORKDIR /app

COPY ./.env .
COPY ./nginx/nginx.conf /etc/nginx/nginx.conf
COPY ./database ./database
COPY --from=build /usr/src/bocchi/target/release/bocchi .

RUN sqlite3 /app/database/database.db < /app/database/init.sql

CMD ["tail", "-f", "/dev/null"]
