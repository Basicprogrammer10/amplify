FROM rust:latest

# Install
RUN cargo install rust-script
RUN apt-get update && apt-get install -qy curl && curl -sSL https://get.docker.com/ | sh

# Copy Files
WORKDIR /amplify
COPY data data
COPY src src
COPY web web
COPY langs/languages.json langs/languages.json
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Build *things*
WORKDIR /amplify/src/problems/text
RUN rust-script ./build_markdown.rs

# Build Server
WORKDIR /amplify
RUN cargo b --release

# Final Steps
EXPOSE 8080
ENTRYPOINT [ "/amplify/target/release/amplify" ]