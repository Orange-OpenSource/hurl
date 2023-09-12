Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop';
$SoftwareName  = 'hurl'

$HashArguments = @{
  PackageName = $env:ChocolateyPackageName
  UnzipLocation = $(Split-Path -Parent $MyInvocation.MyCommand.Definition)
  Url64 = 'https://github.com/Orange-OpenSource/hurl/releases/download/${hurl_latest_version}/hurl-${hurl_latest_version}-x86_64-pc-windows-msvc.zip'
  Checksum64 = '${hurl_latest_sha}'
  ChecksumType64 = 'sha256'
}

Install-ChocolateyZipPackage @HashArguments
