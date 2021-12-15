FROM alpine:3.15 AS builder
WORKDIR /tmp
ARG hurl_latest_version
RUN apk add git jq curl cargo gcc libffi-dev libxml2-dev libxml2-utils openssl-dev
RUN git clone --quiet --depth 1 --branch ${hurl_latest_version} https://github.com/Orange-OpenSource/hurl.git
WORKDIR /tmp/hurl
RUN cargo build --release --verbose --bin hurl

FROM alpine:3.15 AS runner
ARG docker_build_date
ARG hurl_latest_version
LABEL "com.orange.hurl.created"="${docker_build_date}"
LABEL "com.orange.hurl.authors"="Fabrice REIX, Jean Christophe AMIEL, Orange-OpenSource"
LABEL "com.orange.hurl.url"="https://hurl.dev"
LABEL "com.orange.hurl.documentation"="https://hurl.dev"
LABEL "com.orange.hurl.source"="https://github.com/Orange-OpenSource/hurl"
LABEL "com.orange.hurl.version"=${hurl_latest_version}
LABEL "com.orange.hurl.vendor"="Orange-OpenSource"
LABEL "com.orange.hurl.licenses"="Apache-2.0"
LABEL "com.orange.hurl.title"="Hurl"
LABEL "com.orange.hurl.description"="Hurl is a command line tool that runs HTTP requests defined in a simple plain text format"
LABEL "com.orange.hurl.base.name"="alpine:3.15"
COPY --from=builder /tmp/hurl/target/release/hurl /usr/bin/
COPY --from=builder /usr/lib/libxml2.so.2 /usr/lib/
COPY --from=builder /usr/lib/libgcc_s.so.1 /usr/lib/
COPY --from=builder /usr/lib/liblzma.so.5 /usr/lib/
ENTRYPOINT ["/usr/bin/hurl"]
