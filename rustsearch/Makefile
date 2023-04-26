size ?= "100KB"
file ?= "data/WestburyLab.wikicorp.201004_$(size).txt"
ss ?= "third"
output ?= "output.txt"

run: 
	cargo run -- $(file)

file:
	cargo run -- $(ss) $(file) > $(output)