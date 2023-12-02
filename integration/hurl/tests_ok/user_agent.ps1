Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
hurl tests_ok/user_agent.hurl --user-agent "Mozilla/5.0 A"
