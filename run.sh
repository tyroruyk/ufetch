#!/bin/bash

RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

python3 src/main.py
echo ""
pr -m -t ascii/tux.txt output.txt
