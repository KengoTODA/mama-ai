#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import os
import sys
import time
import requests

import ui

# check data every 10 minutes
SLEEP = 10 * 60
URL = 'http://api.waqi.info/feed/shanghai/us-consulate/?token=' + os.getenv('AQI_TOKEN', 'demo')

def main():
    # wait until network is ready
    time.sleep(10)

    thread = ui.MyThread()
    thread.start()

    while True:
        r = requests.get(URL)
        aqi = r.json()['data']['aqi']
        thread.set(aqi)
        time.sleep(SLEEP)

def fork():
    pid = os.fork()

    if pid > 0:
        with open('/home/pi/client/mama-ai.pid','w') as f:
            f.write(str(pid)+"\n")
        sys.exit()

    if pid == 0:
        main()

if __name__=='__main__':
    fork()
