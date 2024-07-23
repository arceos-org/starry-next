#/bin/bash

if [ "$#" -ne 1 ]; then
    echo "Usage: $0 <AX_ROOT>"
    exit 1
fi

AX_ROOT=$1

mkdir -p .cargo
sed -e "s|%AX_ROOT%|$AX_ROOT|g" scripts/config.toml.temp > .cargo/config.toml

echo "Set AX_ROOT (ArceOS directory) to $AX_ROOT"
