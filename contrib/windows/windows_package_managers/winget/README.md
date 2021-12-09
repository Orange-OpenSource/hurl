Official hurl manifest url [#324](https://github.com/Orange-OpenSource/hurl/issues/324) :

- https://github.com/microsoft/winget-pkgs/tree/master/manifests/o/Orange-OpenSource/Hurl/

Update manifest command:
 
```
wingetcreate update --submit --token {personal_github_token} --urls https://github.com/Orange-OpenSource/hurl/releases/download/{version}/hurl-{version}-win64-installer.exe --version {version} Orange-OpenSource.Hurl
```
