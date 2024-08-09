#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -ex
./init.coffee

source ./srv_host.sh

./hostNew.coffee $SRV_HOST
cd ../cron
./banTld.coffee
