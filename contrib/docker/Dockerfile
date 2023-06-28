FROM alpine:3.17 AS builder
WORKDIR /tmp/hurl
COPY . /tmp/hurl
# hadolint ignore=DL3018
RUN apk add --no-cache bash git && \
    bash -c bin/install_prerequisites_alpine.sh && \
    bash -c bin/install_rust.sh && \ 
    bash -c ./bin/release/release.sh

FROM alpine:3.17 AS runner
ARG docker_build_date
ARG hurl_branch
LABEL "org.opencontainers.image.source"="https://github.com/Orange-OpenSource/hurl"
LABEL "org.opencontainers.image.description"="Hurl is a command line tool that runs HTTP requests defined in a simple plain text format"
LABEL "org.opencontainers.image.licenses"="Apache-2.0"
LABEL "com.orange.hurl.created"="${docker_build_date}"
LABEL "com.orange.hurl.authors"="Fabrice REIX, Jean Christophe AMIEL, Orange-OpenSource"
LABEL "com.orange.hurl.url"="https://hurl.dev"
LABEL "com.orange.hurl.documentation"="https://hurl.dev"
LABEL "com.orange.hurl.source"="https://github.com/Orange-OpenSource/hurl"
LABEL "com.orange.hurl.version"=${hurl_branch}
LABEL "com.orange.hurl.vendor"="Orange-OpenSource"
LABEL "com.orange.hurl.licenses"="Apache-2.0"
LABEL "com.orange.hurl.title"="Hurl"
LABEL "com.orange.hurl.description"="Hurl is a command line tool that runs HTTP requests defined in a simple plain text format"
LABEL "com.orange.hurl.base.name"="alpine:3.17"
COPY --from=builder /tmp/hurl/target/release/hurl /usr/bin/
COPY --from=builder /tmp/hurl/target/release/hurlfmt /usr/bin/
COPY --from=builder /usr/lib/libcurl.so.4 /usr/lib/
COPY --from=builder /usr/lib/libnghttp2.so.14 /usr/lib/
COPY --from=builder /usr/lib/libbrotli*.so.1 /usr/lib/
COPY --from=builder /usr/lib/libxml2.so.2 /usr/lib/
COPY --from=builder /usr/lib/libgcc_s.so.1 /usr/lib/
COPY --from=builder /usr/lib/liblzma.so.5 /usr/lib/
ENTRYPOINT ["/usr/bin/hurl"]
