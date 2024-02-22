FROM rust:latest AS build

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli --locked

WORKDIR /usr/src/app
COPY . .

RUN trunk build --release

#####

FROM nginx

WORKDIR /usr/share/nginx/html

COPY --from=build /usr/src/app/dist .

CMD ["nginx", "-g", "daemon off;"]