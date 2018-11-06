FROM mcandre/cartel:linux-x86_64
ENV PATH $PATH:/root/.cargo/bin
RUN apt-get update && \
    apt-get install -y curl gcc && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y
