.PHONY: worker

worker: worker/build/index.js

worker/build/index.js: $(shell find worker/src shared/src -type f -name '*.rs')
	worker-build --release worker
