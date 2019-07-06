#! /bin/bash

if ./download.sh; then
    cd ../traffic_records
    cargo build
    cd ../server
    cargo build
else
    echo failed
    exit 1
fi
