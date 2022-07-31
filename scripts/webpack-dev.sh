#!/usr/bin/env bash

webpack_port=$(grep WEBPACK_DEV_SERVER_PORT .env)
webpack_port_num=${webpack_port//[A-Z_=]/} 

webpack-dev-server --port $webpack_port_num --mode development
