#!/bin/bash

if [[ ! -f $(which psql) ]]; then
  echo "Unable to locate psql. Please ensure you have postgres installed."
  exit 0
fi

psql -U $(whoami) -d template1 <<SQL
CREATE USER caelid;
CREATE DATABASE caelid_development WITH owner = 'caelid';
SQL

conn_uri="postgresql://caelid@localhost:5432/caelid_development"

echo $conn_uri
