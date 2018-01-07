[![Build Status](https://travis-ci.org/KengoTODA/mama-ai.svg?branch=master)](https://travis-ci.org/KengoTODA/mama-ai)

## How this app works

This app uses two IFTTT applets:

1. Twitter -&gt; Webhooks, to post [@CGShanghaiAir](https://twitter.com/CGShanghaiAir)'s data to Heroku app
2. Webhooks -&gt; Email, to send email to owner

This Heroku app is based on [heroku-buildpack-rust](https://github.com/emk/heroku-buildpack-rust).
Its responsibility is parse twitter text to pick air quality data, and filter data which has better air quality. So we can receive email only when air quality is bad (equal to or more than 100).

## How to install client to Raspberry Pi 3

1. run `scp -r ./client pi@raspberrypi.local:/home/pi` to copy `client` directory to target machine
2. ssh to target machine
3. run `sudo cp client/mama-ai.service /etc/systemd/system` to register service
4. run `sudo systemctl enable mama-ai` to enable service

## Copyright

    Copyright 2017 Kengo TODA

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
