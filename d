#! /bin/bash

cmd=$1
case $cmd in
  "up") docker-compose up -d;;
  "re") docker-compose stop web && docker-compose up -d web;;
  "br") docker-compose exec web cargo build --release;;
  "b")  docker-compose exec web cargo build;;
  "bi")  docker-compose run web cargo build;;
  "sh") docker-compose exec web /bin/sh;;
  "pg") docker-compose exec db psql -U postgres;;
  *) docker-compose ${cmd};;
esac
