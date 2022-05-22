FROM python:3

RUN useradd -m runner -d /runner
RUN usermod -p '!!' root

USER runner
WORKDIR /runner

ADD --chown=runner run.sh .

ENTRYPOINT ["bash", "./run.sh"]