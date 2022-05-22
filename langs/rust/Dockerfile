FROM rust:1.31

RUN useradd -m runner -d /runner
RUN usermod -p '!!' root

USER runner
WORKDIR /runner

ADD --chown=runner Cargo.toml .
ADD --chown=runner run.sh .

ENTRYPOINT ["sh", "./run.sh"]