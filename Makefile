build: 
	cargo build --target=arm-unknown-linux-musleabi
deploy:
	cargo build --target=arm-unknown-linux-musleabi 
	scp ./target/arm-unknown-linux-musleabi/debug/troyka_hat pi@raspberrypi.local:~/