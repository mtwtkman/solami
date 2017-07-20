#! /bin/bash

cmd=$1
case $cmd in
  "up") docker-compose up -d;;
  "re") docker-compose stop web && docker-compose up -d web;;
  "br") docker-compose exec web cargo build --release;;
  "b")  docker-compose exec web cargo build;;
  "sh") docker-compose exec web /bin/sh;;
  *) docker-compose ${cmd};;
esac
