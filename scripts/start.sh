#! /bin/bash

SOURCE=`dirname "$BASH_SOURCE"`

cd $SOURCE
cd ../

docker-compose up -d
./traffic_server
