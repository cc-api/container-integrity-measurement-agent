
# Build python example dependencies
FROM python:3.12.3-alpine3.19 AS python-builder

RUN apk update && apk add git
RUN python3 -m pip install --upgrade build
RUN git clone https://github.com/cc-api/cc-trusted-api && \
    cd cc-trusted-api/common/python && \
    python3 -m build

COPY sdk/python3 cima-sdk
RUN cd cima-sdk && python3 -m build

# Build golang example
FROM golang:1.22.1-alpine3.19 AS golang-builder
RUN apk update && apk add git
COPY sdk/golang cima-sdk
RUN cd cima-sdk/example && go mod tidy && go build -o go-sdk-example

# Build rust example
FROM rust:1.77.1-alpine3.19 AS rust-builder
RUN apk update && apk add --no-cache make protobuf-dev musl-dev wget
COPY sdk/rust cima-sdk
COPY service/cima-server/proto/cima-server.proto cima-sdk/cima/proto/cima-server.proto
RUN cd cima-sdk/example && cargo build --release

# Copy/Install
FROM python:3.12.3-alpine3.19
ARG USER=cima
ARG UID=1000
ARG GID=1000
ARG GROUP=cima

WORKDIR /run/cima
RUN addgroup -S -g $GID $GROUP && adduser -S -u $UID -D -G $GROUP $USER
RUN chown $USER:$GROUP /run/cima

COPY test ./
COPY sdk/python3/example/py_sdk_example.py ./
COPY --from=python-builder cc-trusted-api/common/python/dist/cctrusted_base*.whl ./
COPY --from=python-builder cima-sdk/dist/cima*.whl ./

COPY --from=golang-builder /go/cima-sdk/example/go-sdk-example ./
COPY --from=rust-builder cima-sdk/example/target/release/rust-sdk-example ./

RUN apk update && apk add bash vim tar wget libexpat=2.6.2-r0
RUN pip install ./cctrusted_base*.whl ./cima*.whl pytest && rm -f *.whl

USER ${USER}
ENTRYPOINT ["tail", "-f", "/dev/null"]
