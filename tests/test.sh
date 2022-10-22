#!/bin/ash

# csharp
(cd ./cs/error && otter && exit 1)
(cd ./cs/ok && otter || exit 1)

# golang
(cd ./go/error && otter && exit 1)
(cd ./go/ok && otter || exit 1)

exit 0
