
#!/bin/bash
sg docker -c "docker build ../ -f ./Dockerfile -t gabrik91/shm-channel" --oom-kill-disable
# --no-cache