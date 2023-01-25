# how to manage hurl chocolatey package
Get sha256 sum of the new hurl-x.y.z-win64.zip file

```
Get-FileHash -Path <path>\hurl-<x>.<y>.<z>-win64.zip
```

Update *.nuspec and tools/*.ps1 files with new version and sum values, then execute local installer:

```
choco pack
choco install hurl -s .
hurl --version
echo "GET https://google.fr" | hurl --location
```

and finally push package to official chocolatey repository

```
choco apikey -k <choco api key> -source https://push.chocolatey.org/
choco push
```
