build: dependencies
	npm run compile
	./node_modules/.bin/vsce package -o grustonnet.vsix

dependencies:
	jsonnet package.jsonnet > package.json
	npm --prefix . i
