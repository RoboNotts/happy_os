from launch import LaunchDescription
from launch_ros.actions import Node
from launch_ros.substitutions import FindPackageShare
import xacro
from pathlib import Path

PACKAGE_NAME = "happy"
PACKAGE_SHARE_DIRECTORY = Path(FindPackageShare(PACKAGE_NAME))
URDF_FILE = "amy_marks.urdf.xacro"
URDF_DIRECTORY = "urdf"

doc = xacro.parse(open(PACKAGE_SHARE_DIRECTORY / URDF_DIRECTORY / URDF_FILE))
xacro.process_doc(doc)
robot_description = {'robot_description': doc.toxml()}

def generate_launch_description():
  return LaunchDescription([
    Node(
      package='robot_state_publisher',
      executable='robot_state_publisher',
      output='screen',
      parameters=[robot_description]
    )
  ])

