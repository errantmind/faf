#!/bin/bash

~/apps/wrk/wrk -H "Host: 127.0.0.1:8089" -H "Accept: text/plain,text/html;q=0.9,application/xhtml+xml;q=0.9,application/xml;q=0.8,*/*;q=0.7" -H "Connection: keep-alive" --latency -d 120 -c 256 --timeout 5 -t "4" "http://127.0.0.1:8089/plaintext"
