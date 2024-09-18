#!/usr/bin/env bash

set -e

pushd "$(git rev-parse --show-toplevel)"
  docker build . -t penpals -f Dockerfile

  docker run -i  --init -p 3000:3000 penpals
popd
