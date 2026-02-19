Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

# Default: color
hurl tests_pty/color/color.hurl

# No color
$env:NO_COLOR = '1'
hurl tests_pty/color/color.hurl
$env:NO_COLOR = $null

# Color
$env:HURL_COLOR = '1'
hurl tests_pty/color/color.hurl
$env:HURL_COLOR = $null

# No color
$env:HURL_COLOR = '0'
hurl tests_pty/color/color.hurl
$env:HURL_COLOR = $null

# No color
$env:HURL_NO_COLOR = '1'
hurl tests_pty/color/color.hurl
$env:HURL_NO_COLOR = $null

# Color
$env:HURL_NO_COLOR = '0'
hurl tests_pty/color/color.hurl
$env:HURL_NO_COLOR = $null

# No color
hurl --no-color tests_pty/color/color.hurl

# Color
hurl --color tests_pty/color/color.hurl

# Cli flags priority is greater than env vars
# No color
$env:HURL_NO_COLOR = '0'
hurl --no-color tests_pty/color/color.hurl
$env:HURL_NO_COLOR = $null
