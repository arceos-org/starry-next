# StarryOS

[![CI](https://github.com/arceos-org/starry-next/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/arceos-org/starry-next/actions/workflows/ci.yml)

A monolithic kernel based on [ArceOS](https://github.com/arceos-org/arceos).

## Quick Start
```sh
# Build user applications
cd apps
make ARCH=x86_64 build
cd ..

# Build kernel
make ARCH=x86_64 build

# Run kernel
make ARCH=x86_64 run
```