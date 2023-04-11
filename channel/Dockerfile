FROM ubuntu:jammy

# first let's build the runtime, the python binding and the python api
ENV DEBIAN_FRONTEND=noninteractive


RUN apt update && apt install build-essential lldb curl git -y

# install rust
RUN curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain nighlty -y

RUN git clone https://github.com/gabrik/shm-bus-error

# build example
RUN bash -c "source /root/.cargo/env && cd /shm-bus-error  && cargo build --all-targets"

RUN  DEBIAN_FRONTEND=noninteractive apt clean && apt autoclean

CMD bash -c "/shm-bus-error/target/debug/shm-channel -n $ELEMS -s $SIZE"