#!/usr/bin/env bash

# Bash
cog generate-completions bash >~/.local/share/bash-completion/completions/cog.bash-completion

cog install-hook --all
