webGen:
	wasm-pack build -t web
webStart:
	cd pkg/ && python -m SimpleHTTPServer
webStart2:
	cd pkg/ && python server.py