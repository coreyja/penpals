FROM rust:1.81.0

RUN rustup target add wasm32-unknown-unknown

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall trunk -y

RUN wget https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
RUN chmod +x tailwindcss-linux-x64
RUN mv tailwindcss-linux-x64 /bin/tailwindcss
RUN alias tailwindcss=/bin/tailwindcss

WORKDIR /app

COPY . .

RUN cd frontend && trunk build --release
RUN cd server && cargo build --release

EXPOSE 3000

CMD ["target/release/server"]
