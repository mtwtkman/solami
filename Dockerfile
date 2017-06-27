FROM jimmycuadra/rust

RUN mkdir /app
WORKDIR /app
COPY ./ /app

EXPOSE 3000
