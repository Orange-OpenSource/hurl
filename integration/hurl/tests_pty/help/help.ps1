Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# In pty tests, --help is wrapped on a 100 columns wide terminal.
export NO_COLOR=1
hurl --help
