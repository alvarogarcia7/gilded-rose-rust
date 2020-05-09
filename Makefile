.PHONY: docker-build
docker-build:
	docker build -t my-rust-app .
	docker run -it -v ${PWD}:/usr/src/myapp --rm --name my-running-app my-rust-app 


