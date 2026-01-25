# Embedded Rust with ESP32

This repository contains study projects focused on embedded systems development using Rust and ESP32 (ESP-IDF).

The main goal is to explore clean software architecture for embedded systems, with emphasis on finite state machines (FSMs), task-based design, and hardware abstraction suitable for real-world driver development.

## Goals
- Learn Rust applied to real embedded systems
- Design reusable and testable FSMs
- Separate control logic from hardware access
- Implement abstract drivers using traits
- Build a foundation for Linux, SoC, and FPGA drivers

## Architecture Overview
The project is structured in layers:

- **FSM layer**: pure state logic, hardware-agnostic
- **Task layer**: execution loop / RTOS-style tasks
- **Driver layer**: hardware abstraction via traits

This separation allows the same FSM logic to be reused across different platforms and hardware implementations.

## Implemented Concepts
- Embedded Rust with ESP-IDF
- Traditional and event-driven FSMs
- Tick-based timing (no blocking delays)
- Task-oriented design
- Hardware abstraction using traits (custom HAL)
- Driver decoupled from application logic

## Project Structure
src/
├── main.rs
├── tasks/
├── fsm/
└── drivers/

## Status
Ongoing studies and architectural exploration.

## Future Work
- Additional drivers (GPIO, button, buzzer)
- Inter-task communication
- Event queues
- Memory-mapped I/O
- FPGA integration
- Linux driver foundations
