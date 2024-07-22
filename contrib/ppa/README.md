# WIP !!

## Run ubuntu container
```
docker run -it --rm ubuntu:22.04 bash
```

## Install prerequisites
```
apt update
DEBIAN_FRONTEND=noninteractive \
    apt install -y git vim curl \
                   rustc cargo \
                   curl libcurl4-openssl-dev libxml2-utils libxml2-dev libssl-dev \
                   build-essential devscripts debhelper dh-cargo
```

## Clone hurl 4.3.0
```
git clone --depth 1 https://github.com/Orange-OpenSource/hurl.git --branch 4.3.0 /tmp/ppa/hurl-4.3.0
cd /tmp/ppa/hurl-4.3.0
```

## Create debian dir tree
```
mkdir debian debian/source
```

## Create debian/source/format file
```
echo "3.0 (native)" > debian/source/format
```

## Create debian/compat file
```
echo "10" > debian/compat
```

## Create debian/copyright file
```
cat << EOF > debian/copyright
Format: https://www.debian.org/doc/packaging-manuals/copyright-format/1.0/
Upstream-Name: hurl
Upstream-Contact: https://github.com/Orange-OpenSource/hurl/issues
Source: https://github.com/Orange-OpenSource/hurl

Files: *
Copyright: 2024 Jean Christophe AMIEL, Fabrice REIX, Filipe PINTO
License: Apache-2.0

License: Apache-2.0
 License detail can be found at "https://github.com/Orange-OpenSource/hurl/blob/master/LICENSE".

EOF
```

## Create debian/control file
```
cat << EOF > debian/control
Source: hurl
Section: utils
Priority: optional
Maintainer: lepapareil <filipe.pinto@orange.com>
Build-Depends:  debhelper (>= 10), dh-cargo, rustc, cargo, libcurl4-openssl-dev, libxml2-utils, curl, libxml2-dev, libssl-dev
Standards-Version: 4.6.0
Homepage: https://hurl.dev
Rules-Requires-Root: no

Package: hurl
Architecture: any
Depends: \${shlibs:Depends}, \${misc:Depends}
Description: Hurl is a command line tool that runs HTTP requests defined in a simple plain text format.
 It can chain requests, capture values and evaluate queries on headers and body response. Hurl is very versatile: it can be used for both fetching data and testing HTTP sessions.
 Hurl makes it easy to work with HTML content, REST / SOAP / GraphQL APIs, or any other XML / JSON based APIs.
EOF
```

## Create debian/changelog
```
cat << EOF > debian/changelog
hurl (4.3.0) UNRELEASED; urgency=medium

  * Initial Release.

 -- lepapareil <filipe.pinto@orange.com>  Fri, 17 May 2024 13:30:36 +0200
EOF
```

## create debian/rules file
cat << EOF > debian/rules
#!/usr/bin/make -f

%:
	dh \$@ --buildsystem=cargo

override_dh_auto_build:
	cargo build --release

EOF
chmod u+x debian/rules
```

## Create deb package source
```
debuild -S -us -uc
```

## List deb package source files
```
cd ..
ls -l hurl_*
```
