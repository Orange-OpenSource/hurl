$ErrorActionPreference = 'Stop';
$SoftwareName  = 'hurl'

$HashArguments = @{
  PackageName = $env:ChocolateyPackageName
  UnzipLocation = $(Split-Path -Parent $MyInvocation.MyCommand.Definition)
  Url64 = 'https://github.com/Orange-OpenSource/hurl/releases/download/{version}/hurl-{version}-win64.zip'
  Checksum64 = '{Checksum64}'
  ChecksumType64 = 'sha256'
}

Install-ChocolateyZipPackage @HashArguments