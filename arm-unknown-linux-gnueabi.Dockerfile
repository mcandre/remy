FROM mcandre/cartel:debian-other
ENV PATH $PATH:/root/.cargo/bin
RUN apt-get update && \
    apt-get install -y curl gcc && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    rustup target add arm-unknown-linux-gnueabi && \
    apt-get remove -y curl && \
    apt-get autoremove -y && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/*
