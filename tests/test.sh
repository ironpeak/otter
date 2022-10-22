#!/bin/ash

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# csharp
(cd ${SCRIPT_DIR}/cs/error && otter && exit 1)
(cd ${SCRIPT_DIR}/cs/ok && otter || exit 1)

# golang
(cd ${SCRIPT_DIR}/go/error && otter && exit 1)
(cd ${SCRIPT_DIR}/go/ok && otter || exit 1)

exit 0
