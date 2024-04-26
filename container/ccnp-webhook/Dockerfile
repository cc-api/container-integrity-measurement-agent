# Build golang example
FROM golang:1.22.1-alpine3.19 AS golang-builder
COPY service/ccnp-webhook ccnp-webhook
RUN cd ccnp-webhook && go mod tidy && go build -o ccnp-webhook

# Copy & Install
FROM alpine:3.19
ARG USER=ccnp
ARG GROUP=ccnp
ARG UID=1000
ARG GID=1000

COPY --from=golang-builder /go/ccnp-webhook/ccnp-webhook /usr/bin/

USER $UID
CMD ["/usr/bin/ccnp-webhook"]
