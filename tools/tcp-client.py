import socket
import datetime
from time import sleep

s = socket.socket()
s.connect(("127.0.0.1", 12345))


# data = bytearray(b'X' * 1920 * 1080)
data = bytearray(b'X' * 840 * 600 * 3)
size = len(data).to_bytes(4, 'big')


for i in range(600):
    print(datetime.datetime.now())
    data[0] = i%100
    s.send(size)
    s.send(data)
    sleep(1/16)