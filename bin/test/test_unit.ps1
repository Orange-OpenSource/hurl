Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'

write-host -foregroundcolor Cyan "----- unit tests  -----"
cargo test --release --tests
if ($LASTEXITCODE) { Throw }
