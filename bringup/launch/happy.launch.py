from launch import LaunchDescription
from launch_ros.actions import Node
import xacro
from pathlib import Path
from ament_index_python.packages import get_package_share_directory

PACKAGE_NAME = "happy"
PACKAGE_SHARE_DIRECTORY = Path(get_package_share_directory(PACKAGE_NAME))
URDF_FILE = "amy_marks.urdf.xacro"
URDF_DIRECTORY = "urdf"
CONTROLLERS_FILE = "happy_controllers.yaml"
CONTROLLERS_DIRECTORY = "controller"

doc = xacro.parse(open(PACKAGE_SHARE_DIRECTORY / URDF_DIRECTORY / URDF_FILE))
xacro.process_doc(doc)
robot_description = {'robot_description': doc.toxml()}
controllers_file = PACKAGE_SHARE_DIRECTORY / CONTROLLERS_DIRECTORY / CONTROLLERS_FILE

def generate_launch_description():
    return LaunchDescription([
        Node(
            package='robot_state_publisher',
            executable='robot_state_publisher',
            output='screen',
            parameters=[
                robot_description
            ]
        ),
        Node(
            package="controller_manager",
            executable="ros2_control_node",
            output='screen',
            parameters=[
                controllers_file
            ]
        )
    ])

