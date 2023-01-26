Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop';
$SoftwareName  = 'hurl'

$HashArguments = @{
  PackageName = $env:ChocolateyPackageName
  UnzipLocation = $(Split-Path -Parent $MyInvocation.MyCommand.Definition)
  Url64 = 'https://github.com/Orange-OpenSource/hurl/releases/download/${hurl_latest_version}/hurl-${hurl_latest_version}-win64.zip'
  Checksum64 = '${hurl_latest_sha}'
  ChecksumType64 = 'sha256'
}

Install-ChocolateyZipPackage @HashArguments
