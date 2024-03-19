import rospy
from sensor_msgs.msg import Image, LaserScan, Imu  # Assuming ROS 1
import socket
import horus_bridge # Import your generated Protobuf module

# Replace with your actual ROS subscriber logic
def handle_message(data):
  try:
    message = horus_bridge.Message.decode(data)

    if message.type == horus_bridge.MessageType.IMAGE:
      image_msg = Image()
      image_msg.data = message.image.data
      image_msg.width = message.image.width
      image_msg.height = message.image.height
      image_msg.encoding = message.image.encoding
      # Add header if needed
      # ... Publish to image topic

    elif message.type == horus_bridge.MessageType.LASER_SCAN: 
      # ... Create a LaserScan msg and publish...
      pass

    elif message.type == horus_bridge.MessageType.IMU:
      # ... Create an Imu msg and publish...
      pass

  except Exception as e:
      rospy.logerr("Error decoding message: {}".format(e))

HOST = '127.0.0.1'  # Address of the Rust server
PORT = 8080     

with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((HOST, PORT))

    while not rospy.is_shutdown():
        data = s.recv(1024) 
        if data:
            handle_message(data)
def main():
  rospy.init_node('horus_bridge')
