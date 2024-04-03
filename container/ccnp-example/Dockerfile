
# Build python example dependencies
FROM python:3.12.1-alpine3.19 AS python-builder

RUN apk update && apk add git
RUN python3 -m pip install --upgrade build
RUN git clone https://github.com/cc-api/cc-trusted-api && \
    cd cc-trusted-api/common/python && \
    python3 -m build

COPY sdk/python3 ccnp-sdk
RUN cd ccnp-sdk && python3 -m build

# Build golang example
FROM golang:1.22.1-alpine3.19 AS golang-builder
COPY sdk/golang ccnp-sdk
RUN cd ccnp-sdk/example && go mod tidy && go build -o go-sdk-example

# Build rust example
FROM rust:1.77.1-alpine3.19 AS rust-builder
RUN apk update && apk add --no-cache make protobuf-dev musl-dev wget
COPY sdk/rust ccnp-sdk
COPY service/ccnp-server/proto/ccnp-server.proto ccnp-sdk/ccnp/proto/ccnp-server.proto
RUN cd ccnp-sdk/example && cargo build --release

# Copy/Install
FROM python:3.12.1-alpine3.19

WORKDIR /run/ccnp

COPY test ./
COPY sdk/python3/example/py_sdk_example.py ./
COPY --from=python-builder cc-trusted-api/common/python/dist/cctrusted_base*.whl ./
COPY --from=python-builder ccnp-sdk/dist/ccnp*.whl ./

COPY --from=golang-builder /go/ccnp-sdk/example/go-sdk-example ./
COPY --from=rust-builder ccnp-sdk/example/target/release/rust-sdk-example ./

RUN apk update && apk add bash vim tar wget
RUN pip install ./cctrusted_base*.whl ./ccnp*.whl pytest && rm -f *.whl

ENTRYPOINT ["tail", "-f", "/dev/null"]
