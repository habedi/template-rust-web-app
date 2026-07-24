# Build from the workspace root: docker build -f backend/Dockerfile .
FROM rust:1.97-alpine AS build

RUN apk add --no-cache musl-dev
WORKDIR /app

# Copy the manifests first so the dependency layer is cached independently of
# the source code.
COPY Cargo.toml Cargo.lock ./
COPY backend/Cargo.toml ./backend/
RUN mkdir -p backend/src && echo "fn main() {}" > backend/src/main.rs
RUN cargo build --release --package backend
RUN rm -rf backend/src

COPY backend/migrations ./backend/migrations
COPY backend/src ./backend/src
# Touch the entry point so cargo does not reuse the placeholder build.
RUN touch backend/src/main.rs && cargo build --release --package backend

FROM alpine:3.24
RUN adduser -D -u 10001 app
COPY --from=build /app/target/release/backend /usr/local/bin/backend
USER app
EXPOSE 8080
ENTRYPOINT [ "backend" ]
