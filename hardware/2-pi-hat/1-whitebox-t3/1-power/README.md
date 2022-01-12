# POWER - Whitebox T3

## Power Consumption

To estimate the total power consumption of your setup, add the consumption of each of your EZO Circuits

On the Whitebox T3, the EZO devices are powered either by 3.3V or 5V. The voltage depends on which port is used. See Pinout for all port information.

| EZO Device       | Voltage |  Max   | Standby | Sleep  |
| ---------------- |:-------:|:------:|:-------:|:------:|
| EZO pH           |   5V    | 18.3mA | 16.0mA  | 1.16mA |
| EZO pH           |  3.3V   | 14.5mA | 13.8mA  | 0.99mA |
| EZO DO           |   5V    | 13.5mA | 13.1mA  | 0.66mA |
| EZO DO           |  3.3V   | 12.1mA | 12.0mA  | 0.3mA  |
| EZO Conductivity |   5V    | 50.0mA | 18.2mA  | 0.7mA  |
| EZO Conductivity |  3.3V   | 35.0mA | 16.9mA  | 0.4mA  |
| EZO RTD          |   5V    | 16.0mA | 15.4mA  | 0.4mA  |
| EZO RTD          |  3.3V   | 14.3mA | 13.8mA  | 0.09mA |

## Estimate power consumption

Let's assume we have an EZO pH circuit, an EZO Conductivity circuit, an EZO RTD circuit and two EZO PMP dosing pumps. We can estimate the total power consumption like this:

| Device           | Ch# | Voltage |  Max Power  |   Standby   |    Sleep    |
| ---------------- | --- |:-------:|:-----------:|:-----------:|:-----------:|
| Whitebox T3      |     |   5V    |    70mA     |    70mA     |    70mA     |
| EZO pH           | 1   |   5V    |   18.3mA    |   16.0mA    |   1.16mA    |
| EZO Conductivity | 2   |   5V    |   50.0mA    |   18.2mA    |    0.7mA    |
| EZO RTD          | 3   |  3.3V   |   14.3mA    |   15.4mA    |    0.4mA    |
| EZO PMP          | 4   |  3.3V   |   12.5mA    |   12.4mA    |   0.13mA    |
| EZO PMP          | 5   |  3.3V   |   12.5mA    |   12.4mA    |   0.13mA    |
| **Total**        |     |         | **177.6mA** | **144.4mA** | **72.52mA** |

> This is an estimation. Numbers in reality will vary.

## Select a suitable power supply

To make the Whitebox T3 and the EZO devices work well, they must be properly powered. A junky or under-powered power supply will ruin your day.

> We recommend to power your Pi-T3 stack using a dedicated power supply (not from a computer USB port)

To find a suitable power supply for your Pi-T3 stack, you need to add all other devices in the setup:
* Raspberry Pi power consumption
* Raspberry Pi 3 B+ under full load: 980mA
* [Official Raspberry Pi Documentation: Power Suppl](https://www.raspberrypi.com/documentation/computers/raspberry-pi.html#power-supply)
* [Raspberry Pi Power Consumption Benchmarks] (https://www.pidramble.com/wiki/benchmarks/power-consumption)
* Other boards / HATs
* Peripherals - Displays, SSDs, etc.

Using the numbers from our example estimation above, peak can be 177mA + 980mA = 1157mA.
* As a safety margin, we recommend to double the estimate mA to get to the required power supply wattage 1157mA *2 = 2314mA = 2.314A
* In this example, a standard 5V 2.5A DC power supply is suitable. Buy it from a well known supplier - and your day is saved