import threading
import wiringpi

SCLK = 20
DIO = 21
RCLK = 26
LED = 12

# display 3 digits in 60 fps
DIGITS = 3
FPS = 60
DELAY = 1000 * DIGITS // FPS

DATA = {
    '0': 0b00111111,
    '1': 0b00000110,
    '2': 0b01011011,
    '3': 0b01001111,
    '4': 0b01100110,
    '5': 0b01101101,
    '6': 0b01111101,
    '7': 0b00000111,
    '8': 0b01111111,
    '9': 0b01101111,
    ' ': 0b00000000
}

"""A class which is responsible to manage user interface (4 array of 7 segment LED) via GPIO"""
class MyThread(threading.Thread):
    def __init__(self):
        super(MyThread, self).__init__()
        self.__aqi = 0
        self.__on = False
        self.__reset()

    def set(self, aqi):
        self.__aqi = aqi

    def __reset(self):
        wiringpi.wiringPiSetupGpio()
        wiringpi.pinMode(SCLK, 1)
        wiringpi.pinMode(DIO, 1)
        wiringpi.pinMode(RCLK, 1)
        wiringpi.pinMode(LED, 1)
        wiringpi.digitalWrite(LED, 0)

    """Display a character to specified position (0-indexed)"""
    def __display(self, char, position):
        wiringpi.digitalWrite(RCLK, 0)
        wiringpi.shiftOut(DIO, SCLK, 1, DATA[char])
        wiringpi.shiftOut(DIO, SCLK, 1, 0b1111 ^ (0b1 << position)) # 0 means ON, 1 means OFF
        wiringpi.digitalWrite(RCLK, 1)

    """Blink LED every seconds, if AQI is unhealthy (more than or equal to 100)"""
    def __blink(self):
        if self.__aqi >= 100:
            self.__on ^= True
        else:
            self.__on = False
        wiringpi.digitalWrite(LED, 1 - self.__on)

    def run(self):
        while True:
            self.__blink()
            for i in range(FPS):
                for x in range(DIGITS):
                    wiringpi.delay(DELAY)
                    char = ('%03u' % self.__aqi)[x]
                    self.__display(char, x + 1)
