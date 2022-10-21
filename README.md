# otter

Otter is a aggregation of multiple dependency scanners that automatically find vulnerabilities in software dependencies.

## Languages

| Language   | Tool        | Files                                     | Supported |
| ---------- | ----------- | ----------------------------------------- | --------- |
| C#         | dotnet      | package-lock.json                         | ❌         |
| Go         | nancy       | go.mod go.sum                             | ❌         |
| JavaScript | npm/yarn    | package-lock.json yarn.lock               | ❌         |
| Python     | pip-audit   | requirements.txt Pipfile.lock poetry.lock | ❌         |
| Rust       | cargo audit | Cargo.lock                                | ❌         |