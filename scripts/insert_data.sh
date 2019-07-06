#! /bin/bash

if ./download.sh; then
    cd ../traffic_records
    cargo build --release
    cd ../server
    cargo build --release
else
    echo failed
    exit 1
fi
