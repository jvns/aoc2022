all:
.PHONY: all

day0%/input.txt day%/input.txt: session.txt
	mkdir --parents $(@D)
	curl --cookie "session.txt" --output "$@" \
		"https://adventofcode.com/2022/day/$*/input"

day%/output.txt: day%/input.txt day%/solve.py
	cd $(@D); python3 solve.py
	cat "$@"
