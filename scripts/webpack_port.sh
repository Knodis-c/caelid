#!/usr/bin/env bash

info=$(grep WEBPACK_DEV_SERVER .env)

printf ${info//[A-Z_=]/} 
