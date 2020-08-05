.DEFAULT_GOAL := run

client_build:
	ssvmup build

client:
	node app/app.js

client_clean:
	ssvmup clean

server_build:
	cargo build

server:
	cargo run -- --server

server_clean:
	cargo clean

run: server client

build: server_build client_build

clean: server_clean client_clean
