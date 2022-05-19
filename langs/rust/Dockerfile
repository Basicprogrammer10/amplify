FROM debian:buster-slim

RUN apt-get update
RUN apt-get -y install curl
RUN apt-get clean
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal --default-toolchain stable
RUN ls /root/.cargo/

RUN useradd -m runner -d /runner
RUN chown -R runner /root
RUN usermod -p '!!' root

USER runner
RUN /root/.cargo/bin/cargo --version
WORKDIR /runner
ADD --chown=runner Cargo.toml .
ADD --chown=runner run.sh .

ENTRYPOINT ["sh", "./run.sh"]
