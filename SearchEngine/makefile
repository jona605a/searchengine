IDX ?= 5
SIZE ?= "100KB"
FILE ?= "Data/WestburyLab.wikicorp.201004_$(SIZE).txt"

run:
	java --enable-preview -cp bin Index$(IDX) $(FILE)

testCollisions:
	java --enable-preview -cp bin Index$(IDX) Data/WestburyLab.wikicorp.201004_100KB.txt "Test"
	java --enable-preview -cp bin Index$(IDX) Data/WestburyLab.wikicorp.201004_5MB.txt "Test"

testU:
	java --enable-preview -cp bin Tester $(FILE) 4 5

tester:
	java --enable-preview -cp bin Tester $(FILE) $(i) $(j)
