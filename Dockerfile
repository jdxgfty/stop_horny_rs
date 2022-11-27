FROM alpine:latest
RUN mkdir /app
COPY ./app /app
WORKDIR /app
RUN apk add gcompat
ENTRYPOINT ["./stop_horny"]
