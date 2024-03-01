build_plugins:
	cd plugins && cargo build --target wasm32-unknown-unknown

run_server:
	cd server && cargo run

run_web:
	cd web && npm run dev