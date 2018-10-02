on run {input, parameters}
	do shell script "/Users/j/www/rust/rustbar/target/release/rustbar -t " & input

	return input
end run