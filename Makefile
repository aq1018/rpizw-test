bootstrap:
	docker build -t rpi0w/builder:latest .

build: bootstrap
	cross build --target=arm-unknown-linux-gnueabihf

clean:
	cross clean

deploy: build
	scp target/arm-unknown-linux-gnueabihf/debug/servo pi@pi0w:

run: deploy
	ssh pi@pi0w ./servo