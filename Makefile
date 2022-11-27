DOCS := $(PWD)/docs

book: $(DOCS)/guide/index.html

$(DOCS)/guide/index.html: $(DOCS)/src/*.md
	cd docs && mdbook build

.PHONY: book
