#!/bin/bash

docker kill frontend alice-relay bob-relay charlie-relay collator 
sleep 1
docker rm frontend alice-relay bob-relay charlie-relay collator
