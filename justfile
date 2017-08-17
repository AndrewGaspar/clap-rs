@update-contributors:
	githubcontrib --help 2>/dev/null || echo 'githubcontrib not found, see https://github.com/mgechev/github-contributors-list' && false
	echo 'Removing old CONTRIBUTORS.md'
	mv CONTRIBUTORS.md CONTRIBUTORS.md.bak
	echo 'Downloading a list of new contributors'
	echo "the following is a list of contributors:" > CONTRIBUTORS.md
	echo "" >> CONTRIBUTORS.md
	echo "" >> CONTRIBUTORS.md
	githubcontrib --owner kbknapp --repo clap-rs --sha master --cols 6 --format md --showlogin true --sortBy contributions --sortOrder desc >> CONTRIBUTORS.md
	echo "" >> CONTRIBUTORS.md
	echo "" >> CONTRIBUTORS.md
	echo "This list was generated by [mgechev/github-contributors-list](https://github.com/mgechev/github-contributors-list)" >> CONTRIBUTORS.md
	rm CONTRIBUTORS.md.bak

run-test TEST:
	just run-tests --test {{TEST}}

only-run-test TEST NAME:
	cargo test --test {{TEST}} {{NAME}}

run-tests:
	cargo test --features "yaml unstable"

debug TEST:
	just run-test {{TEST}} --features "debug"

only-debug TEST NAME:
	# @TODO-v3-beta: Add yaml feature back
	cargo test --test {{TEST}} --features "debug unstable" {{NAME}}

bench: set-nightly
	cargo bench 
	just remove-nightly

set-nightly:
	rustc -V | grep 'nightly' || rustup override add nightly

remove-nightly:
	rustc -V | grep 'stable' || rustup override remove

lint: set-nightly
	cargo build --features lints
	just remove-nightly

clean:
	cargo clean
	find . -type f -name "*.orig" -exec rm {} \;
	find . -type f -name "*.bk" -exec rm {} \;
	find . -type f -name ".*~" -exec rm {} \;

count-errors:
    cargo build 2>&1 | grep -e 'error[:\[]' | wc -l

find-errors:
    cargo build 2>&1 | grep -e '--> [^:]*' -o | sort | uniq -c
