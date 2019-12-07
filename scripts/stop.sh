#! /bin/bash

SOURCE=`dirname "$BASH_SOURCE"`

cd $SOURCE
cd ../

docker-compose stop -t 1
