FROM golang:alpine as build

RUN echo $COMPOSE_PROJECT_NAME

ARG MAIN_PATH

RUN if [[ -z "$MAIN_PATH" ]] ; then exit 1 ; fi

# Set necessary environmet variables needed for our image
ENV GO111MODULE=on \
    CGO_ENABLED=0 \
    GOOS=linux \
    GOARCH=amd64

# Move to working directory /build
WORKDIR /build

# Copy and download dependency using go mod
COPY go.mod .
COPY go.sum .
RUN go mod download

# Copy the code into the container
COPY . .

# Build the application
RUN go build -o /bin/app $MAIN_PATH

FROM scratch
COPY --from=build /bin/app  /
ENTRYPOINT ["/app"]
