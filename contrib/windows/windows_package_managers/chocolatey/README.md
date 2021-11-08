# how to manage hurl chocolatey package

First update *.nuspec and tools/*.ps1 files and execute local installer:

```
choco pack
choco install hurl -s .
hurl --version
echo GET google.fr | hurl --location
```

Then push package to offical chocolatey repository

```
choco apikey -k [API_KEY_HERE] -source https://push.chocolatey.org/
choco push
```

