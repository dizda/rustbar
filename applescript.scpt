on run {input, parameters}
	do shell script "curl --request POST --url http://127.0.0.1:3000/touchbar/ --header 'content-type: application/json' --data '{\"price\":\"" & input & "\"}'"

	return input
end run