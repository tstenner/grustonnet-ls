# Development


## Setup

The easiest and best way to create a development environment is to just use Nix flakes. You'll just need Nix and direnv.

Run `direnv allow` and all required dependencies in the correct version will be installed automatically (for this project only).

For all other setups ensure that you have go, rust (1.88+), and jsonnet (0.21+) installed

## Committing

This repository enforces two rules for commits:

1. Your commits have to be signed. Either using SSH or GPG. There is absolutely no reason to not sign your commits, so I'll just force you :)
It is trivial to use it

Git:
```bash
git config --global gpg.format ssh
git config --global user.signingkey ~/.ssh/id_ed25519.pub
git config --global commit.gpgsign true
```
jj:
```bash
jj config set --user signing.behavior "own"
jj config set --user signing.backend "ssh"
jj config set --user signing.key "~/.ssh/id_ed25519.pub"
```

2. All commit messages must be [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/). This makes generating changelogs easy and also improves the general readability of commit messages


## Testing

To test the current dev version you have a few options

### Using stdio

If you just need to test simple things you can just convince you editor to start the freshly compiled binary. Either by specifying an absolute path in the settings of your editor, override your current stable version, or by manipulating the `PATH` variable. To do that just run `PATH=$(pwd)/target/debug:$PATH <your editor>`

Using this method you'll only have limited debugging capabilities


### Using sockets

If you need better debugging support, you should use sockets to communicate with the language server. For `neovim` you'd override the `cmd` in your lsp config
```lua
cmd = vim.lsp.rpc.connect("127.0.0.1", 4874)
```

Then run the binary in e.g. `gdb` with `gdb --args ./target/debug/grustonnet-ls --port 4874`


### Using lsp devtools

You can also use the [lsp devtools](https://lsp-devtools.readthedocs.io/en/latest/). This is basically a mitm proxy for the lsp

