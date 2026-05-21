# keep-running

[![Crates.io](https://img.shields.io/crates/v/keep-running.svg)](https://crates.io/crates/keep-running)
[![CI](https://github.com/jimmyhmiller/keep-running/actions/workflows/ci.yml/badge.svg)](https://github.com/jimmyhmiller/keep-running/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

A terminal session manager. Like `dtach`, but with names you can remember and a CLI that doesn't fight you.

Start something, walk away, come back, it's still running.

```sh
keep-running run -- npm run dev
# detach with Ctrl+a d, close your terminal, go to lunch...
keep-running                       # later: list what's running
keep-running fuzzy-penguin         # attach by name
```

## Install

From crates.io:

```sh
cargo install keep-running
```

From source:

```sh
git clone https://github.com/jimmyhmiller/keep-running
cd keep-running
cargo install --path .
```

Unix only (Linux, macOS). Uses Unix domain sockets.

## Use

```sh
keep-running run -- npm run dev              # start a session running a command
keep-running run --name dev -- npm run dev   # ...with a name you pick
keep-running shell                           # start one with your default shell
keep-running list                            # see what's running (alias: ls)
keep-running attach fuzzy-penguin            # attach to a session
keep-running kill fuzzy-penguin              # stop one
```

Shortcuts at the top level:

```sh
keep-running                # bare: list sessions
keep-running fuzzy-penguin  # bare name: attach (prefix match works, e.g. `fuzzy`)
```

Inside an attached session:

```
Ctrl+a d        detach (leave it running)
Ctrl+a k        kill it
Ctrl+a Ctrl+a   send a literal Ctrl+a
```

If you don't name a session, it gets a random one like `fuzzy-penguin` or `quiet-otter`. Any unique prefix attaches it.

## Shell completions

```sh
keep-running completions bash > /usr/local/etc/bash_completion.d/keep-running
keep-running completions zsh  > "${fpath[1]}/_keep-running"
keep-running completions fish > ~/.config/fish/completions/keep-running.fish
```

Also supports `elvish` and `powershell`.

## Where state lives

- **Session metadata** (`<name>.json`): `$XDG_CONFIG_HOME/keep-running/sessions/` (typically `~/.config/keep-running/sessions/` on Linux, `~/Library/Application Support/keep-running/sessions/` on macOS).
- **Sockets**: `$XDG_RUNTIME_DIR/keep-running/`, or `/tmp/keep-running-<uid>/` if unset.

Override either with `KEEP_RUNNING_SESSION_DIR` / `KEEP_RUNNING_SOCKET_DIR`.

If a daemon crashes, leftover socket files are cleaned up on the next `run`/`start` for that name; stale `.json` files clear when `list` notices the pid is gone.

## Compared to…

- **`dtach`** — same idea, but sessions have names instead of socket paths you have to remember, plus `list`/`kill` built in.
- **`tmux`/`screen`** — those are multiplexers. `keep-running` does one thing: keep a process running detached. No windows, panes, or config language.

## License

MIT. See [LICENSE](LICENSE).
