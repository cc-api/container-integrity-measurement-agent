# Build golang example
FROM golang:1.22.1-alpine3.19 AS golang-builder
COPY service/cima-webhook cima-webhook
RUN cd cima-webhook && go mod tidy && go build -o cima-webhook

# Copy & Install
FROM alpine:3.19
ARG USER=cima
ARG GROUP=cima
ARG UID=1000
ARG GID=1000

COPY --from=golang-builder /go/cima-webhook/cima-webhook /usr/bin/

USER $UID
CMD ["/usr/bin/cima-webhook"]
