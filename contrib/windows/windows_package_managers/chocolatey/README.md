# how to manage hurl chocolatey package

Get latest released version

```
$hurl_latest_version=((Invoke-WebRequest -UseBasicParsing https://api.github.com/repos/Orange-OpenSource/hurl/releases/latest).content | ConvertFrom-Json | Select -exp tag_name)
echo ${hurl_latest_version}
```

Get latest released hurl-x.y.z-win64.zip file's sha256 sum

```
Invoke-WebRequest -UseBasicParsing https://github.com/Orange-OpenSource/hurl/releases/download/${hurl_latest_version}/hurl-${hurl_latest_version}-win64.zip -OutFile C:\Windows\Temp\hurl-latest-win64.zip
$hurl_latest_sha=(Get-FileHash C:\Windows\Temp\hurl-latest-win64.zip).Hash
echo ${hurl_latest_sha}
```

Update choco package files with the latest released version and sha256 sum

```
(Get-Content -Path hurl.nuspec) | foreach{$_.replace('${hurl_latest_version}',${hurl_latest_version})} | Set-Content hurl.nuspec
(Get-Content -Path tools\chocolateyinstall.ps1) | foreach{$_.replace('${hurl_latest_version}',${hurl_latest_version})} | Set-Content tools\chocolateyinstall.ps1
(Get-Content -Path tools\chocolateyinstall.ps1) | foreach{$_.replace('${hurl_latest_sha}',${hurl_latest_sha})} | Set-Content tools\chocolateyinstall.ps1
```

Execute local installer:

```
choco pack
choco install hurl -s .
hurl --version
echo "GET https://google.fr" | hurl --location
```

And finally push package to official chocolatey repository

```
choco apikey -k <choco api key> --source https://push.chocolatey.org/
choco push --source https://push.chocolatey.org/
```
