run:
	cargo run

image-build:
	docker build . -t mtinside/prom-counter:latest

image-push:
	docker push mtinside/prom-counter
