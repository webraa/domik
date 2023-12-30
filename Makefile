binname="webapp_template"

help:
	@cat Makefile

edit:
	@nvim ./src/example_view.rs
edit.app:
	@nvim ./src/root_app.rs
edit.main:
	@nvim ./src/main.rs

all: release trunk.release savetogit
trunk.release:
	@trunk build --release
serve:
	@trunk serve

run: release size
	@./target/release/$(binname)
release:
	@cargo rustc --release -- -C prefer-dynamic
test:
	@cargo test

size:
	@ls -lAh ./target/release/$(binname)
path:
	export LD_LIBRARY_PATH='/home/configurator/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib'


to.release:
	@git checkout release
to.dev:
	@git checkout dev
merge.dev:
	@git merge dev

pull:
	@git pull

savetogit: git.pushall
git.pushall: git.commitall
	@git push
git.commitall: git.addall
	@if [ -n "$(shell git status -s)" ] ; then git commit -m 'saving'; else echo '--- nothing to commit'; fi
git.addall:
	@git add .

clean:
	@cargo clean
	@trunk clean
