#!/bin/bash

docker run -v $(pwd):/root/tsttr -w /root/tsttr rust cargo build --target=x86_64-pc-windows-gnu