#! /bin/bash

cmd=$1
case $cmd in
  "b") docker-compose exec web cargo build --release;;
  "sh") docker-compose exec web /bin/sh
esac
