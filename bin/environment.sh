#!/bin/bash
set -Eeuo pipefail

date

echo "# os"
uname -a

echo "# user"
whoami

echo "# python3"
if command -V python3 ; then
    which python3
    python3 -V 
else
    echo "No python3 installed"
fi

echo "# pip"
if command -V pip ; then
    which pip
    pip --version
else
    echo "No pip installed"
fi

echo "# curl"
if command -V curl ; then
    which curl || true
    curl --version || true
else
    echo "No curl installed"
fi


echo  "# rust"
if command -V rustc ; then
    which rustc 
    rustc --version
    which cargo
    cargo --version
else
    echo "No rust installed"
fi

