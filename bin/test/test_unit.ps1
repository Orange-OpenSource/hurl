Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

powershell write-host -foregroundcolor Cyan "----- unit tests  -----"

# run test units
cargo test --release --features strict --tests
if ($LASTEXITCODE) { Throw }
