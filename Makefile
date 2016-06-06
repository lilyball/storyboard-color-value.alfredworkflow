NAME = storyboard-color-value

all: target/release/$(NAME)
.PHONY: all

target/release/$(NAME):
	cargo build --release
.PHONY: target/release/$(NAME)

install: target/release/$(NAME)
	./alfred-install-workflow/install-workflow.sh target/release/$(NAME)
.PHONY: install

update-plist:
	./alfred-install-workflow/install-workflow.sh --update-plist
.PHONY: update-plist

clean:
	cargo clean
.PHONY: clean

