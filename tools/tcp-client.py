import socket
import datetime
from time import sleep

s = socket.socket()
s.connect(("127.0.0.1", 12345))

data_list = []
with open('1920_1080.jpeg', 'rb') as f:
    data = f.read()
    data_list.append(data)

with open('a.jpg', 'rb') as f:
    data = f.read()
    data_list.append(data)

# data = bytearray(b'X' * 1920 * 1080 * 3)
# data = bytearray(b'X' * 840 * 600 * 3)
# data = bytearray(b'X' * 3)


for i in range(6000):
    print(datetime.datetime.now())
    data = data_list[i%10 < 5]
    size = len(data).to_bytes(4, 'big')
    s.send(size)
    s.send(data)
    sleep(1/60)


# import cv2
# import numpy as np
# import io
# from PIL import Image

# video = cv2.VideoCapture("BubbleSort.mp4")

# i = 0
# data_list = []
# while video.isOpened():

#     ret, frame = video.read()
#     frame = cv2.cvtColor(frame, cv2.COLOR_BGR2RGB)
#     image = Image.fromarray(frame)
#     png = io.BytesIO()
#     image.save(png, format='png')
#     b_frame = png.getvalue()
#     data_list.append(b_frame)
#     i += 1
#     if i >= 60 * 10:
#         break

# video.release()

# for i in range(10):
#     for data in data_list:
#         size = len(data).to_bytes(4, 'big')
#         s.send(size)
#         s.send(data)
#         sleep(1/60)

