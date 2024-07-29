FROM rust:1.77.1-alpine3.19 AS cima-server-builder

RUN apk update \
    && apk add --no-cache make protobuf-dev musl-dev wget openssl-dev openssl-libs-static

COPY service/cima-server /cima-server
RUN cd /cima-server && make build

# ======================================================================================================================

FROM alpine:3.19
ARG USER=cima
ARG GROUP=cima
ARG UID=1000
ARG GID=1000

RUN apk update && apk add --no-cache openssl-libs-static

RUN addgroup -S -g $GID $GROUP \
    && adduser -S -u $UID -D -G $GROUP $USER

COPY --from=cima-server-builder /cima-server/target/release/cima_server /usr/bin
COPY --from=cima-server-builder /cima-server/configs/policy.yaml /etc

USER $UID
CMD ["/usr/bin/cima_server", "-p", "/etc/policy.yaml"]
