FROM rust:1.84

# Update cargo
RUN rustup update

# work dir
WORKDIR /usr/src/myapp

# Copying code
COPY . .

# Compile project
RUN cargo build --release

# port exposed
EXPOSE 80

# Executing
CMD ["./target/release/web_actix"]

