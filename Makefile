



clean:
	find . -maxdepth 1 -type d \( -path . -o -path './.*' \) -prune -o -exec sh -c 'cd {} && cargo clean' \;



