# Description of Hardware Setup

## Bill-Of-Material:

 * LED
 * Button
 * Resistor 670 Ohm
 * Potentiometer 
 * female-female-cable (DebugProbe for 3 DebugPins)
 * female-male-cable is (for DebugProbe Left 1, Left 2, Right2)

# Board Wiring

For the DebugPico the Pins are numbered by the pico board.
For left side, right side the Pins according to the bread board numeration.

## Left Side

| Source | Destination |
|-------:|-------------|
|      + | Right 2     |
|      1 | DebucPico 7 |
|      2 | DebucPico 6 |
|      3 | - (left)    |
|     19 | Button 1    |
|      + | Button 2    |
|     20 | Resistor 1  |
|     25 | Resistor 2  |
|     25 | LED Anode   |
|      - | LED Kathode |

## Right Side

| Source | Destination  |
|-------:|--------------|
|      2 | DebugPico 39 |
|      2 | + (left)     |
|     26 | + (left)     |
|     28 | 10           |
|     30 | - (left)     |
|     26 | Poti 1       |
|     28 | Poti 2       |
|     30 | Poti 3       |


## DebugPin (from left to right)

| Source | Destination |
|-------:|-------------|
|      1 | DebucPico 4 |
|      2 | DebucPico 3 |
|      3 | DebucPico 5 |
