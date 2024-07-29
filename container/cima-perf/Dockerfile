FROM python:alpine AS python-builder

RUN apk update && apk add git
RUN python3 -m pip install --upgrade build
RUN git clone https://github.com/cc-api/evidence-api && \
    cd evidence-api/common/python && \
    python3 -m build

COPY sdk/python3 cima-sdk
RUN cd cima-sdk && python3 -m build

# ======================================================================================================================

FROM python:alpine

WORKDIR /run/cima

COPY container/cima-perf/cima_perf.py ./
COPY --from=python-builder evidence-api/common/python/dist/evidence_api*.whl ./
COPY --from=python-builder cima-sdk/dist/cima*.whl ./

RUN apk update && apk add bash vim
RUN python3 -m pip install pytest
RUN pip install ./evidence_api*.whl ./cima*.whl && rm -f *.whl

ENTRYPOINT ["tail", "-f", "/dev/null"]
