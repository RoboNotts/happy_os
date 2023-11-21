#ifndef MOTOR_INTERFACE_H_
#define MOTOR_INTERFACE_H_

#include <stdint.h>

const char * LEFT_SERVO_NAME = "amy_485_port_left";
const char * RIGHT_SERVO_NAME = "amy_485_port_right";
const uint8_t DEFAULT_DEVICE_ADDRESS = 0x1;


typedef struct motor_controller motor_controller_t;

extern motor_controller_t *
motor_controller_new(const char *port_path, const uint8_t device_address);

extern void
motor_controller_free(motor_controller_t *);

extern void
motor_controller_enable_modbus(motor_controller_t *);

extern void
motor_controller_set_motor_enabled(motor_controller_t *);

extern void
motor_controller_set_position_feedforward(motor_controller_t *, uint16_t);

extern void
motor_controller_set_position_gain(motor_controller_t *, uint16_t);

extern void
motor_controller_set_motor_disabled(motor_controller_t *);

extern int32_t
motor_controller_get_position(motor_controller_t *);

extern float
motor_controller_get_velocity(motor_controller_t *);

extern float
motor_controller_set_velocity(motor_controller_t *, float);

#endif // MOTOR_INTERFACE_H_