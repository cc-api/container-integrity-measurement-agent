FROM rust:1.74.1-alpine3.17 AS ccnp-server-builder

RUN apk update \
    && apk add --no-cache make protobuf-dev musl-dev wget openssl-dev openssl-libs-static

COPY service/ccnp-server /ccnp-server
RUN cd /ccnp-server && make build

# add rediness and liveness probe command
WORKDIR /usr/bin
RUN GRPC_HEALTH_PROBE_VERSION=v0.4.22 && \
    wget -qO grpc-health-probe https://github.com/grpc-ecosystem/grpc-health-probe/releases/download/${GRPC_HEALTH_PROBE_VERSION}/grpc_health_probe-linux-amd64 && \
    chmod +x grpc-health-probe

# ======================================================================================================================

From alpine:3.17
ARG USER=ccnp
ARG GROUP=ccnp
ARG UID=1000
ARG GID=1000

RUN apk update && apk add --no-cache openssl-libs-static

RUN addgroup -S -g $GID $GROUP \
    && adduser -S -u $UID -D -G $GROUP $USER

COPY --from=ccnp-server-builder /ccnp-server/target/release/ccnp_server /usr/bin
COPY --from=ccnp-server-builder /ccnp-server/configs/policy.yaml /etc
COPY --from=ccnp-server-builder /usr/bin/grpc-health-probe /usr/bin

USER $UID
CMD ["/usr/bin/ccnp_server", "-p", "/etc/policy.yaml"]
