import rospy
from sensor_msgs.msg import Image  # Assuming ROS 1

# Replace with your actual ROS subscriber logic
def handle_image(msg):
  # Process the image data (msg.data)
  print("Image received:", msg.width, msg.height, msg.encoding)

def main():
  rospy.init_node('horus_bridge')
