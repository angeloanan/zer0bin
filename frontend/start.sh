#!/bin/bash
set -e

cd /app
./import-meta-env -x .env.example

cd /app/dist
nginx -g "daemon off;"