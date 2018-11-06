FROM mcandre/cartel:linux-arm
ENV PATH $PATH:/root/.cargo/bin
RUN apt-get update && \
    apt-get install -y curl gcc && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    rustup target add arm-unknown-linux-gnueabi
