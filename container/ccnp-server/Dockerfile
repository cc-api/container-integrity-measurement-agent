FROM rust:1.77.1-alpine3.19 AS ccnp-server-builder

RUN apk update \
    && apk add --no-cache make protobuf-dev musl-dev wget openssl-dev openssl-libs-static

COPY service/ccnp-server /ccnp-server
RUN cd /ccnp-server && make build

# ======================================================================================================================

FROM alpine:3.19
ARG USER=ccnp
ARG GROUP=ccnp
ARG UID=1000
ARG GID=1000

RUN apk update && apk add --no-cache openssl-libs-static

RUN addgroup -S -g $GID $GROUP \
    && adduser -S -u $UID -D -G $GROUP $USER

COPY --from=ccnp-server-builder /ccnp-server/target/release/ccnp_server /usr/bin
COPY --from=ccnp-server-builder /ccnp-server/configs/policy.yaml /etc

USER $UID
CMD ["/usr/bin/ccnp_server", "-p", "/etc/policy.yaml"]
