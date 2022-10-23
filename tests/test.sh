#!/bin/bash

BASEDIR=$(dirname "$0")

echo "Running Tests"

# csharp
cd ${BASEDIR}/cs/error
otter

cd ${BASEDIR}/cs/ok
otter

# golang
cd ${BASEDIR}/go/error
otter

cd ${BASEDIR}/go/ok
otter

exit 0
