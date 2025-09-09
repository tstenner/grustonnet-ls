# **G**o **R**ust Js**onnet** **L**anguage **S**erver
This is a jsonnet language server using the `go-jsonnet` implementation to generate the AST and evaluate jsonnet code

## Install

### Getting the binary
#### Using Cargo
```bash
cargo install --path .
```

#### Using Nix
Either use this flake as an input or run
```bash
nix shell git+ssh://git@gitlab.ppidev.net/ct/std/grustonnet-ls.git
```

#### Using Gitlab (not on darwin)
Until there is a proper release you con go into the latest pipeline and download the binary from `build:linux` or `build:windows`

NOTE: Currently the windows version from the CI is lacking `std` completion support

### Editors

#### Neovim

Add this file to your `[after/]lsp` folder. Put a `grustonnet.json` next to it with the configuration

```lua
local function getJson(filename)
	local foundFile = vim.fs.find(filename, { path = vim.loop.cwd(), upward = true, type = "file" })[1]
	if foundFile == nil then
		return nil
	end
	local f = io.open(foundFile, "r")
	if f == nil then
		return nil
	end
	local data = f:read("*all")
	return vim.json.decode(data)
end

local grustonnet_settings = getJson("./grustonnet.json");
return {
	--cmd = vim.lsp.rpc.connect("127.0.0.1", 4874),
	cmd = { "grustonnet-ls" },
	filetypes = { 'jsonnet', 'libsonnet' },
	root_markers = { 'jsonnetfile.json', '.git' },
	settings = grustonnet_settings,
}

```

### (Evil-)Helix

Add this to your `languages.toml`
```toml
[language-server.grustonnet-ls]
command = "grustonnet-ls"

[[language]]
name = "jsonnet"
language-servers = ["grustonnet-ls"]
```

### VCcodium

TODO: For whatever reason a separate plugin is required

### Intellij

Install `lsp4all` and add the binary

TODO: finish this documentation and how to get syntax highlights


## Options
To generate a schema with all the supported options run
```bash
grustonnet-ls --export-config-schema
```

## Roadmap

* [-] Completion
    * [x] Global completion
    * [x] Index completion
    * [x] Value preview
        * [ ] Make the Object preview pretty
    * [-] Complete "everything"
        * [ ] Find the remaining cases where completion does not work/tries to complete the wrong node
    * [x] Stdlib
    * [-] Advanced Stdlib completion
        * [x] extVar completion
        * [ ] Return values
        * [ ] Function parameters e.g. std.map
    * [ ] Complete Loops
    * [x] All jsonnet imports
        * [ ] Properly handle completion if "/" is already in the string
    * [x] self
    * [x] super
        * [ ]  Fix super not working if it never had an index
    * [-] Keyword completion
        * [ ] Only complete if valid
        * [ ] Add missing keywords
    * [-] Conditionals
        * [ ] Actually evaluate the condition
    * [x] Default parameters
    * [x] Builder pattern
        * [ ] Check extremely complex patterns
    * [x] Array access
    * [ ] Unused function arguments
* [x] Semantic tokens
* [-] Inlay Hints
    * [x] Function parameters
        * [ ] Only update if needed
    * [ ] Indices
* [x] Goto definition
    * [ ] Goto file from import string
    * Can go everything we can complete
* [x] Find reference
    * Can find references for everything we can goto
* [x] Rename
    * Can rename everything we can find the reference
* [ ] Signature Help
* [ ] Docsonnet support
    * [ ] How to handle the license issues? Docsonnet does not have an open source license
    * [ ] Handle the stdlib the same as docsonnet?
* [-] AST repair
* [ ] Commands
    * [x] Evaluate file
* [ ] Missing LSP features
    * [ ] Code actions
    * [ ] Code Lense?
    * [ ] Hover
    * [ ] Document highlight
    * [ ] Document/Workspace symbols
    * [ ] Folding
    * [ ] Call hierachie
    * [ ] File operation support for automatic refactoring (like renaming imports)
* [ ] Improve performance
    * [ ] Test rust2go mem
    * [ ] More multithreading
* [ ] More tests
    * [ ] Fix ignored tests
* [ ] (Major) Code cleanup
    * Once the prototyping phase is over

## Known Issues

* (Go)-Jsonnet bugs
    * If you import `foo.libsonnet` and there is also a `foo.libsonnet` in the current working directory, evaluating the snippet will result in a diagnostic error
        * To reproduce `cat mydir/bar.jsonnet | jsonnet --jpath mydir -`
    * If there is a circular dependency go-jsonnet emits a strange error
