# CLI AI assistant

## Installation

```
cargo install hilfe
```

## Configuration

You need to edit your `hilfe.toml`. Run `hilfe` and it will guide you.

## Usage

```bash
$ hilfe reboot into firmware # ask it to do anything
systemctl reboot --firmware-setup

$ hilfe --qa When was tmux created
2007

$ hilfe --alias 
Save this to your zsh config:
alias '??'='source /home/sagartiwari/.config/hilfe.zsh'

# Now ?? will generate the command line and paste it to your prompt for immediate usage
```

![hilfe demo](https://raw.githubusercontent.com/amokfa/hilfe/main/resources/demo.png)

