#! /bin/bash

echo "starting"

if test ! -f "did_insert.txt"
then
    echo "inserting..."
    # wait for mongodb to start 
    ./scripts/wait-for-it.sh mongo:27017 -t 30
    cd /usr/local/app/traffic_records
    cargo run --release
    touch ../did_insert.txt
fi

cd /usr/local/app/server
cargo run --release
