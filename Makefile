all:
.PHONY: all

day0%/input.txt day%/input.txt: session.txt
	mkdir --parents $(@D)
	curl --cookie "session.txt" --output "$@" \
		"https://adventofcode.com/2022/day/$*/input"

day%/output1.txt: day%/input.txt day%/solve.rs
	cd $(@D); rustc solve.rs && ./solve 1 < input.txt > output1.txt
	cat "$@"

day%/output2.txt: day%/input.txt day%/solve.rs
	cd $(@D); rustc solve.rs && ./solve 2 < input.txt > output2.txt
	cat "$@"

day%/test1.txt: day%/input.txt day%/solve.rs
	cd $(@D); rustc solve.rs && ./solve 1 < test.txt > test1.txt
	cat "$@"

day%/test2.txt: day%/input.txt day%/solve.rs
	cd $(@D); rustc solve.rs && ./solve 2 < test.txt > test2.txt
	cat "$@"
