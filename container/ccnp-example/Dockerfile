FROM python:3.12.1-alpine3.19 AS python-builder

RUN apk update && apk add git
RUN python3 -m pip install --upgrade build
RUN git clone https://github.com/cc-api/cc-trusted-api && \
    cd cc-trusted-api/common/python && \
    python3 -m build

COPY sdk/python3 ccnp-sdk
RUN cd ccnp-sdk && python3 -m build

# ======================================================================================================================

FROM python:3.12.1-alpine3.19

WORKDIR /run/ccnp

COPY container/ccnp-example/ccnp_example.py ./
COPY --from=python-builder cc-trusted-api/common/python/dist/cctrusted_base*.whl ./
COPY --from=python-builder ccnp-sdk/dist/ccnp*.whl ./

RUN apk update && apk add bash vim
RUN pip install ./cctrusted_base*.whl ./ccnp*.whl && rm -f *.whl

ENTRYPOINT ["tail", "-f", "/dev/null"]
