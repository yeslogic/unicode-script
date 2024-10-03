UCD:=16.0.0

tables:
	yeslogic-ucd-generate script --rust-enum --name Script ../ucd-generate/ucd-$(UCD) > src/tables.rs
	cargo fmt


.PHONY: tables

