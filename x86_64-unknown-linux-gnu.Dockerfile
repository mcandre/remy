FROM mcandre/cartel:debian-x86_64
ENV PATH $PATH:/root/.cargo/bin
RUN apt-get update && \
    apt-get install -y curl && \
    curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    rustup update nightly && \
    rustup default nightly && \
    rustup target add i686-unknown-linux-gnu && \
    apt-get remove -y curl && \
    apt-get autoremove -y && \
    apt-get clean -y && \
    rm -rf /var/lib/apt/lists/*
