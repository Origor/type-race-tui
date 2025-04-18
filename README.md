## Update dependencies
> cargo check

## Handle docker service
Just to check up on the application
> systemctl status docker

If any changes to settings are made
> systemctl restart docker

## Build binary
> docker build -t rust-type-race-tui .

## Run binary
> docker run -it --rm -v $(pwd)/logs:/app/logs rust-type-race-tui

### Log Handling
The easy way is just mounting local $(pwd)/logs directory for it to be directly mounted to the docker image so that the logs can be checked in real time.

#### Clean up the container
> docker rm temp-tui-log

#### Now view the app.log file on your host machine
> cat app.log

