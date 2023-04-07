FROM python:3.8-slim

RUN apt-get update --allow-insecure-repositories --allow-unauthenticated

RUN apt-get install -y curl git

COPY ./requirements.txt .

RUN pip install --upgrade pip
RUN pip install -r requirements.txt

RUN git clone https://github.com/eclipse-zenoh/zenoh-python -b 0.7.0-rc

CMD python3 /zenoh-python/examples/z_sub.py -m peer -k "debug/**"