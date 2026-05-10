.PHONY: worker

worker: crates/worker/build/index.js

crates/worker/build/index.js: $(shell find crates/worker/src crates/shared/src -type f -name '*.rs')
	worker-build --release crates/worker
