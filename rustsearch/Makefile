size ?= "100KB"
file ?= "data/WestburyLab.wikicorp.201004_$(size).txt"
ss ?= "third"
output ?= "output.txt"

run: 
	cargo run -- $(file) "9_0"

file:
	cargo run -- $(ss) $(file) > $(output)