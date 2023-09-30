from launch import LaunchDescription
from launch.actions import DeclareLaunchArgument
from launch.conditions import IfCondition
from launch.substitutions import Command, FindExecutable, LaunchConfiguration, PathJoinSubstitution

from launch_ros.actions import Node
from launch_ros.substitutions import FindPackageShare

PACKAGE_NAME = "happy"
PACKAGE_SHARE_DIRECTORY = FindPackageShare(PACKAGE_NAME)
URDF_FILE = "amy_marks.urdf.xacro"
URDF_DIRECTORY = "urdf"
RVIS_DIRECORY = "rviz"
RVIZ_FILE = "happy.rviz"

def generate_launch_description():
    # Declare arguments
    declared_arguments = [
        DeclareLaunchArgument(
            "description_file",
            default_value=URDF_FILE,
            description="URDF/XACRO description file with the robot.",
        ),
        DeclareLaunchArgument(
            "rviz_file",
            default_value=RVIZ_FILE,
            description="URDF/XACRO description file with the robot.",
        ),
        DeclareLaunchArgument(
            "gui",
            default_value="true",
            description="Start Rviz2 and Joint State Publisher gui automatically \
        with this launch file.",
        ),
        DeclareLaunchArgument(
            "prefix",
            default_value='""',
            description="Prefix of the joint names, useful for \
        multi-robot setup. If changed than also joint names in the controllers' configuration \
        have to be updated.",
        )
    ]

    # Initialize Arguments
    description_file = LaunchConfiguration("description_file")
    rviz_file = LaunchConfiguration("rviz_file")
    gui = LaunchConfiguration("gui")
    prefix = LaunchConfiguration("prefix")

    # Get URDF via xacro
    robot_description_content = Command(
        [
            PathJoinSubstitution([FindExecutable(name="xacro")]),
            " ",
            PathJoinSubstitution(
                [PACKAGE_SHARE_DIRECTORY, URDF_DIRECTORY, description_file]
            ),
            " ",
            "prefix:=",
            prefix,
        ]
    )
    robot_description = {"robot_description": robot_description_content}

    
    rviz_config_file = PathJoinSubstitution(
        [PACKAGE_SHARE_DIRECTORY, RVIS_DIRECORY, rviz_file]
    )

    joint_state_publisher_node = Node(
        package="joint_state_publisher_gui",
        executable="joint_state_publisher_gui",
        condition=IfCondition(gui),
    )
    robot_state_publisher_node = Node(
        package="robot_state_publisher",
        executable="robot_state_publisher",
        output="both",
        parameters=[robot_description],
    )
    rviz_node = Node(
        package="rviz2",
        executable="rviz2",
        name="rviz2",
        output="log",
        arguments=["-d", rviz_config_file],
        condition=IfCondition(gui),
    )

    nodes = [
        joint_state_publisher_node,
        robot_state_publisher_node,
        rviz_node,
    ]

    return LaunchDescription(declared_arguments + nodes)
