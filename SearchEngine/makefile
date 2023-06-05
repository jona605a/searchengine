IDX ?= 5
SIZE ?= "100KB"
FILE ?= "Data/WestburyLab.wikicorp.201004_$(SIZE).txt"
i ?= 4
j ?= 5

run: compile
	java --enable-preview -cp bin Index$(IDX) $(FILE)

compile:
	javac src/*.java -d bin

testCollisions:
	java --enable-preview -cp bin Index$(IDX) Data/WestburyLab.wikicorp.201004_100KB.txt "Test"
	java --enable-preview -cp bin Index$(IDX) Data/WestburyLab.wikicorp.201004_5MB.txt "Test"

testcorrect:
	java --enable-preview -cp bin Tester $(FILE) $(i) $(j) correctness

testmem:
	java --enable-preview -cp bin Tester $(FILE) $(i) $(j) memory

testtimeIDX:
	java --enable-preview -cp bin Tester $(FILE) $(i) $(j) timeIDX

testtimeFile:
	java --enable-preview -cp bin Tester $(FILE) $(i) $(j) timeFILE

