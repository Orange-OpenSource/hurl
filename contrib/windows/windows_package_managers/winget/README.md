Official hurl manifest URL:

- https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Orange-OpenSource/Hurl/

Update manifest command:
 
```
$hurl_latest_version=((Invoke-WebRequest -UseBasicParsing https://api.github.com/repos/Orange-OpenSource/hurl/releases/latest).content | ConvertFrom-Json | Select -exp tag_name)
echo ${hurl_latest_version}
wingetcreate update --submit --token <personal_github_token> --urls https://github.com/Orange-OpenSource/hurl/releases/download/${hurl_latest_version}/hurl-${hurl_latest_version}-win64-installer.exe --version ${hurl_latest_version} Orange-OpenSource.Hurl
```
