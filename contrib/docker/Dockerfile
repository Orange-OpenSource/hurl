FROM alpine:3.23 AS builder
ARG TARGETPLATFORM
WORKDIR /tmp/hurl-docker
COPY . /tmp/hurl-docker
RUN apk add --no-cache bash git && \
    bash -c "bin/install_prerequisites_alpine.sh" && \
    bash -c "bin/install_rust.sh" && \
    bash -c "./bin/release/release.sh"

FROM alpine:3.23 AS runner
ARG docker_build_date
ARG docker_build_tag
LABEL "org.opencontainers.image.source"="https://github.com/Orange-OpenSource/hurl" \
      "org.opencontainers.image.description"="Hurl is a command line tool that runs HTTP requests defined in a simple plain text format" \
      "org.opencontainers.image.licenses"="Apache-2.0" \
      "com.orange.hurl.created"="${docker_build_date}" \
      "com.orange.hurl.authors"="Fabrice REIX, Jean-Christophe AMIEL, Filipe PINTO Orange-OpenSource" \
      "com.orange.hurl.url"="https://hurl.dev" \
      "com.orange.hurl.documentation"="https://hurl.dev" \
      "com.orange.hurl.source"="https://github.com/Orange-OpenSource/hurl" \
      "com.orange.hurl.version"="${docker_build_tag}" \
      "com.orange.hurl.vendor"="Orange-OpenSource" \
      "com.orange.hurl.licenses"="Apache-2.0" \
      "com.orange.hurl.title"="Hurl" \
      "com.orange.hurl.description"="Hurl is a command line tool that runs HTTP requests defined in a simple plain text format" \
      "com.orange.hurl.base.name"="alpine:3.23"
COPY --from=builder /tmp/hurl-docker/target/release/hurl /tmp/hurl-docker/target/release/hurlfmt /usr/bin/
COPY --from=builder \
    /usr/lib/libcurl.so.* \
    /usr/lib/libcares.so.* \
    /usr/lib/libpsl.so.* \
    /usr/lib/libzstd.so.* \
    /usr/lib/libunistring.so.* \
    /usr/lib/libnghttp2.so.* \
    /usr/lib/libbrotli*.so.* \
    /usr/lib/libxml2.so.* \
    /usr/lib/libgcc_s.so.* \
    /usr/lib/liblzma.so.* \
    /usr/lib/libidn2.so.* \
    /usr/lib/libnghttp3.so.* \
    /usr/lib/
ENTRYPOINT ["/usr/bin/hurl"]
