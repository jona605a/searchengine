IDX ?= 5
SIZE ?= "5MB"
FILE ?= "Data/WestburyLab.wikicorp.201004_$(SIZE).txt"

run:
	java Index$(IDX).java $(FILE)

test:
	java Index$(IDX).java Data/WestburyLab.wikicorp.201004_100KB.txt "Test"
	java Index$(IDX).java Data/WestburyLab.wikicorp.201004_5MB.txt "Test"
