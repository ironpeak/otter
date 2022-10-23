#!/bin/ash

BASEDIR=$(dirname "$0")

echo "Running Tests"

# csharp
(cd ${BASEDIR}/cs/error && otter) && exit 1
(cd ${BASEDIR}/cs/ok && otter) || exit 1

# golang
(cd ${BASEDIR}/go/error && otter) && exit 1
(cd ${BASEDIR}/go/ok && otter) || exit 1

# javascript
(cd ${BASEDIR}/js/error && otter) && exit 1
(cd ${BASEDIR}/js/ok && otter) || exit 1

# python
(cd ${BASEDIR}/py/error && otter) && exit 1
(cd ${BASEDIR}/py/ok && otter) || exit 1

# rust
(cd ${BASEDIR}/rs/error && otter) && exit 1
(cd ${BASEDIR}/rs/ok && otter) || exit 1

# all
(cd ${BASEDIR} && otter) && exit 1

exit 0
