#!/bin/bash

echo "using password: "
echo $PASSWD

/target/debug/lit-recovery --folder=/data/db --password=$PASSWD

/bin/bash