Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# We check the help without any color
$env:NO_COLOR = '1'

# In pty tests, --help is wrapped on a 100 columns wide terminal.
hurl --help
