#### Simple pub sub micro service event communication

`web-service` is the publisher and opens an http endpoint `/publish` that we can send messages like below

These messages are sent to redis `my-event` channel.

`subscriber-service` is the subscriber and listens to these events and prints messages on the console.

To run you have to do

```
docker-compose up
```

and then

```
curl -X POST http://localhost:8080/publish \
     -H "Content-Type: application/json" \
     -d '{"message": "Fooooo Bar"}'
```
