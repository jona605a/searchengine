IDX ?= 4
SIZE ?= "5MB"
FILE ?= "Data/WestburyLab.wikicorp.201004_$(SIZE).txt"

run:
	java Index$(IDX).java $(FILE)

