# otter

Otter is a aggregation of multiple dependency scanners that automatically find vulnerabilities in software dependencies.

## Tools

| Language   | Tool                                                                          |
| ---------- | ----------------------------------------------------------------------------- |
| C#         | [dotnet](https://github.com/dotnet/sdk)                                       |
| Go         | [govulncheck](https://github.com/golang/vuln)                                 |
| JavaScript | [npm](https://github.com/npm/cli) and [yarn](https://github.com/yarnpkg/yarn) |
| Python     | [pip-audit](https://github.com/pypa/pip-audit)                                |
| Rust       | [cargo-audit](https://github.com/RustSec/rustsec/tree/main/cargo-audit)       |

## Configurations

| Env            | Description                                            | Default |
| -------------- | ------------------------------------------------------ | ------- |
| OTTER_INCLUDES | Comma seperated regex patterns to match files against. | `.`     |
| OTTER_EXCLUDES | Comma seperated regex patterns to match files against. | None    |

### C#

| Env            | Description                                            | Default                |
| -------------- | ------------------------------------------------------ | ---------------------- |
| OTTER_CS_FILES | Comma seperated regex patterns to match files against. | `/packages.lock.json$` |
| OTTER_CS_FLAGS | Flags that are appended at the end of the command.     | `--include-transitive` |

### Golang

| Env            | Description                                            | Default             |
| -------------- | ------------------------------------------------------ | ------------------- |
| OTTER_GO_FILES | Comma seperated regex patterns to match files against. | `/go.mod$,/go.sum$` |
| OTTER_GO_FLAGS | Flags that are appended at the end of the command.     | None                |

### JavaScript

| Env            | Description                                            | Default                           |
| -------------- | ------------------------------------------------------ | --------------------------------- |
| OTTER_JS_FILES | Comma seperated regex patterns to match files against. | `/package-lock.json$,/yarn.lock$` |
| OTTER_JS_FLAGS | Flags that are appended at the end of the command.     | `--frozen-lockfile --production`  |

### Python

| Env            | Description                                            | Default              |
| -------------- | ------------------------------------------------------ | -------------------- |
| OTTER_PY_FILES | Comma seperated regex patterns to match files against. | `/requirements.txt$` |
| OTTER_PY_FLAGS | Flags that are appended at the end of the command.     | None                 |

### Rust

| Env            | Description                                            | Default        |
| -------------- | ------------------------------------------------------ | -------------- |
| OTTER_RS_FILES | Comma seperated regex patterns to match files against. | `/Cargo.lock$` |
| OTTER_RS_FLAGS | Flags that are appended at the end of the command.     | None           |
