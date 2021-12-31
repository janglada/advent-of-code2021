#!/usr/bin/env bash

if [ -z "$1" ]
  then
    echo "No argument supplied"
fi


cargo generate --git https://github.com/janglada/cargo-generate-aoc-day.git --name day$1
