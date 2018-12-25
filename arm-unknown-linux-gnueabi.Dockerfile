FROM mcandre/cartel:debian-other
ENV PATH $PATH:/root/.cargo/bin
RUN apt-get update && \
    apt-get install -y curl && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    rustup update nightly && \
    rustup target add arm-unknown-linux-gnueabi && \
    apt-get remove -y curl && \
    apt-get autoremove -y && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/*
