
#!/bin/bash
sg docker -c "docker build ./ -f ./Dockerfile -t gabrik91/sub --no-cache" --oom-kill-disable