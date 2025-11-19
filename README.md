# **G**o **Rust** Js**onnet** **L**anguage **S**erver
This is a jsonnet language server using the `go-jsonnet` implementation to generate the AST and evaluate jsonnet code

## Install

### Getting the binary
#### Using Nix (preferred way)
Either use this flake as an input or run
```bash
nix shell git+ssh://git@gitlab.ppidev.net/ct/std/grustonnet-ls.git

```
#### Using Cargo
Install go, rust (1.88+), and jsonnet (0.21+) and then run
```bash
cargo install --path ./crates/grustonnet-bin
```


#### Using Gitlab (not on darwin)
Until there is a proper release you can go into the latest pipeline and download the binary from `build:linux` or `build:windows`

Since cross compiling for OSX requires the Apple SDK, there are currently no pipeline builds for OSX

The SDK can't (legally) be used on non Apple hardware. Therefore we would need a Gitlab runner using Apple hardware.

### Editors

For all editors: Download the [latest release](https://gitlab.ppidev.net/ct/std/apps/grustonnet-ls/-/releases/permalink/latest) and put the binary in your path (or configure your plugin to directly point to the binary)

#### Neovim

Add this file to your `[after/]lsp` folder. Put a `grustonnet.json` next to it with the configuration

```lua
local function getJson(filename)
	-- https://neovim.io/doc/user/lua.html#lua-script-location
	local current_file = debug.getinfo(1, "S").source:sub(2)
	local current_dir = vim.fn.fnamemodify(current_file, ":h")

	local file_content = vim.fn.readfile(current_dir .. "/" .. filename)
	return vim.json.decode(table.concat(file_content, "\n"))
end

local grustonnet_settings = getJson("grustonnet.json");
return {
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
config = {}

[[language]]
name = "jsonnet"
language-servers = ["grustonnet-ls"]
```

However, you won't have completion for you config. One workaround is to just use jsonnet for your config and import a json:

```jsonnet
local config = {
  'language-server': {
    'grustonnet-ls': {
      command: 'grustonnet-ls',
      config: import './grustonnet.json',
    },
  },
  language: [
    {
      name: 'jsonnet',
      'language-servers': ['grustonnet-ls'],
    },
  ],

};

std.manifestTomlEx(config, ' ')
```

Compile it with `jsonnet -S config.jsonnet > ~/.config/helix/languages.toml`

### VSCodium

Download [the plugin](https://gitlab.ppidev.net/ct/std/apps/grustonnet-ls/-/releases/permalink/latest/downloads/grustonnet.vsix) and install it.
The plugin includes basic syntax highlighting (you should still user proper highlighting with TreeSitter) and a (live) preview function.

### Intellij

Install [lsp4ij](https://plugins.jetbrains.com/plugin/23257-lsp4ij), download the [lsp4ij template](https://gitlab.ppidev.net/ct/std/apps/grustonnet-ls/-/releases/permalink/latest/downloads/lsp4ij.tar.gz) and extract it. In Intellij add a new language server and select `Import template`.

Since Intellij does not offer support for proper syntax highlighting, the language server has a compatibility mode that bridges TreeSitter nodes to LSP semantic tokens. This is not perfect and you'll probably need to modify some colorscheme settings.


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
        * [x] Make the Object preview pretty
    * [-] Complete "everything"
        * [ ] Find the remaining cases where completion does not work/tries to complete the wrong node
    * [x] Stdlib
    * [-] Advanced Stdlib completion
        * [x] extVar completion
        * [ ] Return values
        * [ ] Function parameters e.g. std.map
    * [ ] Complete Loops
        * [ ] Use `std.map` for loops
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
    * [x] Name after long objects
* [x] Goto definition
    * [x] Goto file from import string
    * Can goto everything we can complete
* [x] Find reference
    * [ ] Import strings
    * Can find references for all identifiers we can goto
* [x] Rename
    * [ ] Rename imports if file is renamed
    * [ ] Rename file if import is renamed
    * Can rename all identifiers we can find the reference of
* [ ] Signature Help
* [x] Docsonnet support
    * [x] How to handle the license issues? Docsonnet does not have an open source license
        * Just evaluate it
    * [ ] Handle the stdlib the same as docsonnet?
* [-] AST repair
* [x] Commands
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
* Windows Specific Issues
    * On Windows the process will comsumes more memory than on other systems. To prevent this set the environment variable `GODEBUG` to `invalidptr=0,cgocheck=0`

## Jsonnet Quirks
* `tailstrict`
    * not part of the spec apart from the reserved keyword
    * no documentation at all
    * in `foo(myArg()) tailstrict` forces myArg to be evaluated before the body, even if it is unused
