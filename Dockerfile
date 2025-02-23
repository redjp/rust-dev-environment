FROM rust:latest
WORKDIR /usr/src/myapp
RUN rustc --version && cargo --version
RUN apt-get update && apt-get install -y \
    build-essential \
    && rm -rf /var/lib/apt/lists/*
CMD ["bash"]
