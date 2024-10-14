cmake_minimum_required(VERSION 3.0.2)
project(horus_bridge)

find_package(catkin REQUIRED COMPONENTS
  rospy
  # Add other dependencies here
)

catkin_package(
  # INCLUDE_DIRS include
  # LIBRARIES horus_bridge
  # CATKIN_DEPENDS rospy
  # DEPENDS system_lib
)

include_directories(
  ${catkin_INCLUDE_DIRS}
)

# Uncomment and modify these if you have C++ code
# add_executable(${PROJECT_NAME}_node src/placeholder_source.cpp)
# target_link_libraries(${PROJECT_NAME}_node ${catkin_LIBRARIES})

# Install launch files
install(DIRECTORY launch
  DESTINATION ${CATKIN_PACKAGE_SHARE_DESTINATION}
)

# Install Python scripts
catkin_install_python(PROGRAMS
  scripts/placeholder_script.py
  DESTINATION ${CATKIN_PACKAGE_BIN_DESTINATION}
)
