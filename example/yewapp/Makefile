default:
	make build
	make run
build:
	wasm-pack build --target web --out-name wasm --out-dir ./static
run:
	miniserve ./static --index index.html
