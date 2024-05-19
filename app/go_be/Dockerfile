FROM golang:alpine as builder

WORKDIR /go/src/app
COPY . /go/src/app
RUN go build -o bin/main ./main.go


FROM alpine:latest

RUN apk add --no-cache ca-certificates && \
    update-ca-certificates
WORKDIR /app
COPY --from=builder /go/src/app/bin/main /app/main
EXPOSE 8900

CMD ["./main"]
