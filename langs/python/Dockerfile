FROM python:3

RUN useradd -m runner -d /runner
RUN usermod -p '!!' root

USER runner
WORKDIR /runner

ENTRYPOINT ["python", "./main.py"]