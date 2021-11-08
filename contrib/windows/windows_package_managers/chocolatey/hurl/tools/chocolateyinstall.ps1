$ErrorActionPreference = 'Stop';
$SoftwareName  = 'hurl'

$HashArguments = @{
  PackageName = $env:ChocolateyPackageName
  UnzipLocation = $(Split-Path -Parent $MyInvocation.MyCommand.Definition)
  Url64 = 'https://github.com/Orange-OpenSource/hurl/releases/download/1.4.0/hurl-1.4.0-win64.zip'
  Checksum64 = '78255BB838095A1015679F92189074C1162EDD51B5EDC1DDA7B863BA7304C4B5'
  ChecksumType64 = 'sha256'
}

Install-ChocolateyZipPackage @HashArguments