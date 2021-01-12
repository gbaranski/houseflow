# Please keep up to date with the new-version of Golang docker for builder
FROM golang:latest

WORKDIR /app 

COPY . . 

RUN go get -u github.com/cosmtrek/air

CMD ["air", "-c", "/app/cmd/state_manager/.air.toml"]
