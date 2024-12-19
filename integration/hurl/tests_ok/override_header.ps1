Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

hurl --header 'User-Agent: different-user-agent' --header 'Accept: different-accept' --header 'Host: different-host' tests_ok/override_header.hurl
