Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# In CI, --help is wrapped on a 100 columns wide terminal.
# We don't want to test color in help for the moment.$env:NO_COLOR = '1'
hurl --help
