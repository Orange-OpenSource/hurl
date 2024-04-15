Set-StrictMode -Version latest
$ErrorActionPreference = 'Stop'
$file = New-Object System.IO.FileStream build\post_large.bin, Create, ReadWrite
$file.SetLength(15728640)
$file.Close()

hurl --verbose --file-root build/ tests_ok/post_large.hurl
