#!/usr/bin/env bash

# Bash
mkdir -p ~/.local/share/bash-completion/completions
touch ~/.local/share/bash-completion/completions/cog.bash-completion

cog generate-completions bash >~/.local/share/bash-completion/completions/cog.bash-completion

cog install-hook --all -o
