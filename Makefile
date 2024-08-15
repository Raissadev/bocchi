SHELL=/bin/zsh

docker.build:
	@docker build -t raissageek/bocchi .

docker.run:
	@docker run -it --rm raissageek/bocchi bash

run:
	@cargo run

build:
	@cargo build --release