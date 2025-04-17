## Update dependencies
> cargo check

## Build binary
> docker build -t rust-type-race-tui .

## Run binary
> docker run -it --rm rust-type-race-tui

## Log Handling
The easy way is just mounting local $(pwd)/logs directory for it to be directly mounted to the docker image so that the logs can be checked in real time.

> docker run -it --rm -v $(pwd)/logs:/app/logs rust-type-race-tui


### Deployed
Less interesting way is to just have a log file that you copy and rm manually to read from at a later time.

#### Run the container (it might exit quickly or you quit it)
> docker run -it --name temp-tui-log rust-type-race-tui

#### Copy the log file from the stopped container to your current directory
> docker cp temp-tui-log:/app/app.log ./logs/app.log

#### Clean up the container
> docker rm temp-tui-log

#### Now view the app.log file on your host machine
> cat app.log

