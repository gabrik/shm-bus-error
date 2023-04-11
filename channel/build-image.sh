#!/bin/bash
sg docker -c "docker build ../ -f ./Dockerfile -t gabrik91/shm-channel --no-cache" --oom-kill-disable