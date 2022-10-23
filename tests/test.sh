#!/bin/bash

BASEDIR=$(dirname "$0")

echo "Running Tests"

# csharp
(cd ${BASEDIR}/cs/error && otter) && exit 1
(cd ${BASEDIR}/cs/ok && otter) || exit 1

# golang
(cd ${BASEDIR}/go/error && otter) && exit 1
(cd ${BASEDIR}/go/ok && otter) || exit 1

exit 0
