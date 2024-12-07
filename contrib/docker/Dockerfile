FROM alpine:3.21 AS builder
ARG TARGETPLATFORM
WORKDIR /tmp/hurl-docker
COPY . /tmp/hurl-docker
RUN apk add --no-cache bash git && \
    if [ "${TARGETPLATFORM}" = "linux/arm64" ] ; then \
        apk add --no-cache -y g++-aarch64-linux-gnu libc6-dev-arm64-cross ; \
        source bin/export_cross_compile_env.sh ; \
    fi ; \
    bash -c "bin/install_prerequisites_alpine.sh" && \
    bash -c "bin/install_rust.sh" && \
    bash -c "./bin/release/release.sh"

FROM alpine:3.21 AS runner
ARG docker_build_date
ARG docker_build_tag
LABEL "org.opencontainers.image.source"="https://github.com/Orange-OpenSource/hurl"
LABEL "org.opencontainers.image.description"="Hurl is a command line tool that runs HTTP requests defined in a simple plain text format"
LABEL "org.opencontainers.image.licenses"="Apache-2.0"
LABEL "com.orange.hurl.created"="${docker_build_date}"
LABEL "com.orange.hurl.authors"="Fabrice REIX, Jean Christophe AMIEL, Orange-OpenSource"
LABEL "com.orange.hurl.url"="https://hurl.dev"
LABEL "com.orange.hurl.documentation"="https://hurl.dev"
LABEL "com.orange.hurl.source"="https://github.com/Orange-OpenSource/hurl"
LABEL "com.orange.hurl.version"=${docker_build_tag}
LABEL "com.orange.hurl.vendor"="Orange-OpenSource"
LABEL "com.orange.hurl.licenses"="Apache-2.0"
LABEL "com.orange.hurl.title"="Hurl"
LABEL "com.orange.hurl.description"="Hurl is a command line tool that runs HTTP requests defined in a simple plain text format"
LABEL "com.orange.hurl.base.name"="alpine:3.21"
COPY --from=builder /tmp/hurl-docker/target/release/hurl /usr/bin/
COPY --from=builder /tmp/hurl-docker/target/release/hurlfmt /usr/bin/
COPY --from=builder /usr/lib/libcurl.so.* /usr/lib/
COPY --from=builder /usr/lib/libcares.so.* /usr/lib/
COPY --from=builder /usr/lib/libpsl.so.* /usr/lib/
COPY --from=builder /usr/lib/libzstd.so.* /usr/lib/
COPY --from=builder /usr/lib/libunistring.so.* /usr/lib/
COPY --from=builder /usr/lib/libnghttp2.so.* /usr/lib/
COPY --from=builder /usr/lib/libbrotli*.so.* /usr/lib/
COPY --from=builder /usr/lib/libxml2.so.* /usr/lib/
COPY --from=builder /usr/lib/libgcc_s.so.* /usr/lib/
COPY --from=builder /usr/lib/liblzma.so.* /usr/lib/
COPY --from=builder /usr/lib/libidn2.so.* /usr/lib/
ENTRYPOINT ["/usr/bin/hurl"]

