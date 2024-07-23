#!/bin/bash

AX_ROOT=.arceos

test ! -d "$AX_ROOT" && echo "Cloning repositories ..." || true
test ! -d "$AX_ROOT" && git clone https://github.com/arceos-org/arceos -b monolithickernel-new "$AX_ROOT" || true
