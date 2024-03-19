syntax = "proto3";

package horus_bridge;

message MessageType {
  IMAGE = 0;
  LASER_SCAN = 1;
  IMU = 2;
  // ... Add other potential message types here
}

message Message {
  MessageType type = 1;
  Image image = 2;
  LaserScan laser_scan = 3;
  Imu imu = 4;
}

message Image {
  uint32 width = 1;
  uint32 height = 2;
  string encoding = 3;  
  bytes data = 4;
}

message LaserScan {
  Header header = 1;  
  float32 angle_min = 2;
  float32 angle_max = 3;
  float32 angle_increment = 4;
  float32 time_increment = 5; 
  float32 scan_time = 6;
  float32 range_min = 7;
  float32 range_max = 8;
  repeated float32 ranges = 9; 
  repeated float32 intensities = 10;
}

message Header {
  uint32 seq = 1;
  time stamp = 2; // Consider using a more descriptive timestamp type
  string frame_id = 3;
}

message Imu {
  Header header = 1;
  geometry_msgs.Quaternion orientation = 2;
  geometry_msgs.Vector3 angular_velocity = 3;
  geometry_msgs.Vector3 linear_acceleration = 4;
  float64[9] orientation_covariance = 5;
  float64[9] angular_velocity_covariance = 6;
  float64[9] linear_acceleration_covariance = 7;
}

// Messages from geometry_msgs (these may need their own file)
message Quaternion {
  float64 x = 1;
  float64 y = 2;
  float64 z = 3;
  float64 w = 4;
}

message Vector3 {
  float64 x = 1;
  float64 y = 2;
  float64 z = 3;
}
