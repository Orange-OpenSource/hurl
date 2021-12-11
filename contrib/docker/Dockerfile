FROM alpine:3.15 AS builder
WORKDIR /tmp
RUN apk add git jq curl cargo gcc libffi-dev libxml2-dev libxml2-utils openssl-dev
RUN git clone --quiet --depth 1 --branch $(curl --silent "https://api.github.com/repos/Orange-OpenSource/hurl/releases/latest" | jq -r .tag_name) https://github.com/Orange-OpenSource/hurl.git
WORKDIR /tmp/hurl
RUN cargo build --release --verbose --bin hurl

FROM alpine:3.15 AS runner
COPY --from=builder /tmp/hurl/target/release/hurl /usr/sbin/
COPY --from=builder /usr/lib/libxml2.so.2 /usr/lib/
COPY --from=builder /usr/lib/libgcc_s.so.1 /usr/lib/
COPY --from=builder /usr/lib/liblzma.so.5 /usr/lib/
ENTRYPOINT ["/usr/sbin/hurl"]
