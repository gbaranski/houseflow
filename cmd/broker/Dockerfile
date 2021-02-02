FROM gcc:latest as builder

RUN apt update
RUN apt install -y cmake

WORKDIR /app

COPY . .


RUN gcc -o a.out main.c

CMD ["./a.out"]

