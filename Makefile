# gem install bundler jekyll
dev:
	bundle exec jekyll serve

fmt:
	cargo fmt

check: fmt
	cargo check

PATTERN?="update_db"
test: check
	cargo test ${PATTERN}

