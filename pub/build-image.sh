
#!/bin/bash
sg docker -c "docker build ../ -f ./Dockerfile -t gabrik91/shm-pub --no-cache" --oom-kill-disable