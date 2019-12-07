#! /bin/bash

SOURCE=`dirname "$BASH_SOURCE"`

cd $SOURCE
cd ../

docker-compose down -t 1
