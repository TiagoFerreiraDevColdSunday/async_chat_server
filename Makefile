install_run_windows:
	$(MAKE) install_cargo_windows
	$(MAKE) build
	cargo run --bin client

build:
	cargo fmt
	cargo build

install_cargo_unix:
	curl https://sh.rustup.rs -sSf | sh

install_cargo_windows:
    powershell -Command "Invoke-WebRequest -Uri https://win.rustup.rs -OutFile rustup-init.exe; Start-Process -FilePath ./rustup-init.exe -Wait; Remove-Item ./rustup-init.exe"

run_server:
	cargo run --bin server

run_client:
	cargo run --bin client