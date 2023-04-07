FROM ubuntu:jammy

# first let's build the runtime, the python binding and the python api
ENV DEBIAN_FRONTEND=noninteractive


RUN apt update && apt install build-essential lldb curl -y

# install rust
RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain 1.66.0 -y

RUN mkdir shm-bus-error

# copy code to container
COPY src /shm-bus-error/src
COPY Cargo.toml /shm-bus-error/Cargo.toml
COPY rust-toolchain /shm-bus-error/rust-toolchain

# build example
RUN bash -c "source /root/.cargo/env && cd /shm-bus-error  && cargo build --all-targets"

RUN  DEBIAN_FRONTEND=noninteractive apt clean && apt autoclean

CMD bash -c "/shm-bus-error/target/debug/shm-bus-error -m peer -p debug/aggregator"