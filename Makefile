run-train:
	docker build -t linear .
	docker run --rm -v $(PWD):/usr/src/app linear

run-predict:
	docker build -t linear .
	docker run --rm -v $(PWD):/usr/src/app linear cargo run --bin predict
