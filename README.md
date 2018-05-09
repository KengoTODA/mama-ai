[![Build Status](https://travis-ci.org/KengoTODA/mama-ai.svg?branch=master)](https://travis-ci.org/KengoTODA/mama-ai)

This application has three parts:

1. Store Shanghai's AQI every 10 mins
2. Report bad AQI by email, and
3. Display latest AQI by web page

# Architecture
## 1. Store Shanghai's AQI every 10 mins

This is handled by the [scheduled task](https://devcenter.heroku.com/articles/scheduler) that runs `curl "http://api.waqi.info/feed/shanghai/?token=$AQI_TOKEN" | jq -r '.data.aqi' | curl -X POST --data @- https://$HEROKU_URL/aqi/`. Then invoked WebAPI will store AQI to postgres database.

To invoke `jq` command, this application requires  [heroku-buildpack-jq](https://github.com/chrismytton/heroku-buildpack-jq) buildpack.
To serve application server written in Rust, this application requires [heroku-buildpack-rust](https://github.com/emk/heroku-buildpack-rust) buildpack.

## 2. Report bad AQI by email

And if AQI is bad, this app calls an IFTTT applet, to send email to owner.
This email realizes push-notification even when user cannot use push-notification supported by iOS/Android.

## 3. Display latest AQI by web page

For admin, this application provides several pages:

1. `/` to display latest AQI

## How to install client to Raspberry Pi 3

1. run `scp -r ./client pi@raspberrypi.local:/home/pi` to copy `client` directory to target machine
2. ssh to target machine
3. run `sudo cp client/mama-ai.service /etc/systemd/system` to register service
4. run `sudo systemctl enable mama-ai` to enable service

## Copyright

    Copyright 2017-2018 Kengo TODA

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
