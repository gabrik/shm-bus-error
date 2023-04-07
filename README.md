# SHM Bus Error - simple way to reproduce

This repo contains a simple way to reproduce:

```
* thread #1, name = 'runtime', stop reason = signal SIGBUS
  * frame #0: 0x00007f2bbf217c4a libc.so.6`___lldb_unnamed_symbol4288 + 778
    frame #1: 0x0000561341ed9d90 runtime`zenoh_transport::common::pipeline::TransmissionPipelineProducer::push_zenoh_message::h7c958c9f309b90eb [inlined] core::intrinsics::copy_nonoverlapping::h3a80de1563bd0d1e at intrinsics.rs:2363:9
    frame #2: 0x0000561341ed9d7c runtime`zenoh_transport::common::pipeline::TransmissionPipelineProducer::push_zenoh_message::h7c958c9f309b90eb at mod.rs:1948:18
```


## How to run

First configure your docker host to allow core dumps:


```bash
echo '/tmp/core.%p' | sudo tee /proc/sys/kernel/core_pattern
```

Then you need to build the images:

```bash
cd pub
./build-image.sh
cd ../sub
./build-image.sh
```

Finally you can run: `docker compose up`

After some time (at least on Ubuntu focal, x86 hosts), it will fail with
```
shm-bus-error-pub-1  | Bus error (core dumped)
```

## Inspect the core dump

Once the container is failed export it as an image and run a shell in it.


```bash
docker commit shm-bus-error-pub-1 pub-core-dump
docker run -it pub-core-dump bash

# X is the PID so in your case replace it with the actual name of the file
lldb /shm-bus-error/target/debug/shm-bus-error -c /tmp/core.X
```

