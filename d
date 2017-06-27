#! /bin/bash

cmd=$1
case $cmd in
  "up") docker-compose up -d;;
  "re") docker-compose stop web && docker-compose up -d web;;
  "b") docker-compose exec web cargo build --release;;
  "sh") docker-compose exec web /bin/sh;;
  *) docker-compose ${cmd};;
esac
