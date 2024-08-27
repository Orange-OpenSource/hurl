#!/bin/bash 
set -Eeuo pipefail

bash completions/hurl.bash
bash completions/hurlfmt.bash

fish completions/hurl.fish
fish completions/hurlfmt.fish

pwsh completions/_hurl.ps1
pwsh completions/_hurlfmt.ps1

