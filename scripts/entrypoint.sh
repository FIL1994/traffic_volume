#! /bin/bash
echo "starting"
cd /usr/local/app
if ./scripts/download.sh; then
    # wait for mongodb to start 
    let "TIMEOUT = 1 * 30"
    ./scripts/wait-for-it.sh mongo:27017 -t $TIMEOUT

    cd traffic_records
    cargo run --release
    cd ../server
    cargo run --release
else
    echo failed
    exit 1
fi
