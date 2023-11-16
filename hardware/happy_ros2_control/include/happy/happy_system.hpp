#ifndef HAPPY_SYSTEM_HPP_
#define HAPPY_SYSTEM_HPP_

#define HAPPY_HARDWARE_PUBLIC __attribute__((visibility("default")))
#define HAPPY_HARDWARE_LOCAL __attribute__((visibility("hidden")))

#include "hardware_interface/system_interface.hpp"
#include "hardware_interface/handle.hpp"
#include "hardware_interface/hardware_info.hpp"
#include "hardware_interface/system_interface.hpp"
#include "hardware_interface/types/hardware_interface_return_values.hpp"
#include "hardware_interface/types/hardware_interface_type_values.hpp"
#include "rclcpp/macros.hpp"

namespace happy {
    class HappySystemHardware : public hardware_interface::SystemInterface
    {
    private:
        // Store the command for the robot
        std::vector<double> hw_commands_;
        std::vector<double> hw_positions_;
        std::vector<double> hw_velocities_;
        // Store the wheeled robot position
        double base_x_, base_y_, base_theta_;
    public:
        RCLCPP_SHARED_PTR_DEFINITIONS(HappySystemHardware);

        HAPPY_HARDWARE_PUBLIC
        hardware_interface::CallbackReturn on_init(
        const hardware_interface::HardwareInfo & info) override;

        HAPPY_HARDWARE_PUBLIC
        std::vector<hardware_interface::StateInterface> export_state_interfaces() override;

        HAPPY_HARDWARE_PUBLIC
        std::vector<hardware_interface::CommandInterface> export_command_interfaces() override;

        HAPPY_HARDWARE_PUBLIC
        hardware_interface::CallbackReturn on_activate(
            const rclcpp_lifecycle::State & previous_state) override;

        HAPPY_HARDWARE_PUBLIC
        hardware_interface::CallbackReturn on_deactivate(
            const rclcpp_lifecycle::State & previous_state) override;

        HAPPY_HARDWARE_PUBLIC
        hardware_interface::return_type read(
            const rclcpp::Time & time, const rclcpp::Duration & period) override;

        HAPPY_HARDWARE_PUBLIC
        hardware_interface::return_type write(
            const rclcpp::Time & time, const rclcpp::Duration & period) override;
    };
    
}

#endif // HAPPY_SYSTEM_HPP_