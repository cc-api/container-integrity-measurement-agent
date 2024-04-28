#!/bin/bash

echo "Create certificate for test only"

mkdir certs
openssl genrsa -out certs/ca.key 2048

openssl req -new -x509 -days 365 -key certs/ca.key \
  -subj "/C=AU/CN=localhost"\
  -out certs/ca.crt

openssl req -newkey rsa:2048 -nodes -keyout certs/tls.key \
  -subj "/C=AU/CN=localhost" \
  -out certs/tls.csr

openssl x509 -req \
  -extfile <(printf "subjectAltName=DNS:localhost") \
  -days 365 \
  -in certs/tls.csr \
  -CA certs/ca.crt -CAkey certs/ca.key -CAcreateserial \
  -out certs/tls.crt
