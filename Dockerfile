FROM rust
EXPOSE 8080

RUN mkdir -p /usr/local/app/
WORKDIR /usr/local/app/
COPY . /usr/local/app/
RUN mkdir -p /usr/local/test/

WORKDIR /usr/local/app/scripts
RUN ./insert_data.sh

WORKDIR /usr/local/app/