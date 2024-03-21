build_plugins:
	cd plugins && cargo build --target wasm32-unknown-unknown

run_server:
	cd server && cargo run

build_web:
	cd web && npm run build

run_web:
	cd web && npm run dev

run: build_web run_server