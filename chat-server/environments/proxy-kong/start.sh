#!/usr/bin/env bash

var="$1"
def="$2"

envsubst < /kong.template.yml > /kong.yml
/docker-entrypoint.sh "$var" "$def"
