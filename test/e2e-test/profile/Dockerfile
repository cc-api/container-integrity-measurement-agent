FROM python:3.12.1-alpine3.19

ARG USER=ccnp
ARG UID=1000
ARG GID=1000
ARG GROUP=ccnp

WORKDIR /run/confidential-cloud-native-primitives
COPY  ./ ./
RUN addgroup -S -g $GID $GROUP && adduser -S -u $UID -D -G $GROUP $USER
RUN chown -R  $USER:$GROUP  ./

RUN apk update && apk add bash vim
<<<<<<< HEAD
RUN pip install -r sdk/python3/requirements.txt && pip install pytest pytdxattest
=======
RUN pip install -r sdk/python3/requirements.txt && pip install pytest
>>>>>>> ddd915b (update e2e test)

USER $UID

COPY --from=golang:1.20-alpine /usr/local/go/ /usr/local/go/
ENV PATH="/usr/local/go/bin:${PATH}"
ENV GOROOT="/usr/local/go"
ENV GOPATH="/home/ccnp/gopath"

RUN pip install  ./sdk/python3/
RUN cd ./test/e2e-test/go-test && go mod tidy && cd ../../..

ENTRYPOINT ["tail", "-f", "/dev/null"]
