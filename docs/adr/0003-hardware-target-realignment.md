# ADR 0003: Hardware Target Realignment (STM32F4)

## Status
Accepted

## Context
The `spacecan-firmware` crate was initially configured to target the **STM32G071** microcontroller using `stm32g0xx-hal`. Furthermore, the project relies on the `bxcan` crate for managing CAN communications at the hardware level. However, a fundamental hardware mismatch was identified: the STM32G071 does not possess a native CAN controller. While some later G0 variants (like the G0B1) support CAN, they use the newer FDCAN peripheral, which is fundamentally incompatible with the `bxcan` abstraction layer.

## Decision
We will realign the firmware hardware target to the **STM32F4** series (specifically Cortex-M4F `thumbv7em-none-eabihf` target via `stm32f4xx-hal`).

## Consequences
- **Positive**: STM32F4 microcontrollers (such as the STM32F407) contain the classic `bxCAN` peripheral natively, perfectly aligning with the `bxcan` dependency.
- **Positive**: Provides a significantly higher clock speed (168 MHz) and larger memory footprint, accommodating future RTOS and advanced PUS service integrations.
- **Negative**: Requires modifying the CI pipeline target triples and redefining the linker script memory map (`memory.x`).
