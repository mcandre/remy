FROM mcandre/cartel:linux-x86
ENV PATH $PATH:/root/.cargo/bin
RUN apt-get update && \
    apt-get install -y curl gcc && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    rustup target add i686-unknown-linux-gnu
