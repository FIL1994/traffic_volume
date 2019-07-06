FROM rust
EXPOSE 8080

RUN mkdir -p /usr/local/app/
WORKDIR /usr/local/app/
COPY . /usr/local/app/
RUN rm -f did_insert.txt

WORKDIR /usr/local/app/scripts
ADD https://raw.githubusercontent.com/vishnubob/wait-for-it/master/wait-for-it.sh wait-for-it.sh
RUN ./insert_data.sh

WORKDIR /usr/local/app/
