FROM jimmycuadra/rust

RUN mkdir /app
WORKDIR /app
COPY ./ /app

EXPOSE 3000

RUN cargo build --release

CMD ./target/release/solami
