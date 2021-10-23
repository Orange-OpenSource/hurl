FROM debian:10.10
RUN apt update && apt install -y wget curl libxml2

ARG VERSION
RUN wget https://github.com/Orange-OpenSource/hurl/releases/download/${VERSION}/hurl_${VERSION}_amd64.deb
RUN apt install ./hurl_${VERSION}_amd64.deb

