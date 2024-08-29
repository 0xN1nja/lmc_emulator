# LMC Emulator

LMC Emulator written in Rust

This emulator implements the instructions provided by [Peter Higginson's LMC](https://peterhigginson.co.uk/LMC) and York University's [LMC Instruction Set](https://www.yorku.ca/sychen/research/LMC/LMCInstructionSummary.html).

## Instructions set

| Name            | Instruction | Code | Description                                                                           |
| --------------- | ----------- | ---- | ------------------------------------------------------------------------------------- |
| Add             | `ADD XY`    | 1XY  | Adds the content of memory address XY to the accumulator                              |
| Subtract        | `SUB XY`    | 2XY  | Subtracts the content of memory address XY from the accumulator                       |
| Store           | `STA XY`    | 3XY  | Stores the accumulator's content in memory address XY                                 |
| Load            | `LDA XY`    | 5XY  | Loads the content of memory address of XY into the accumulator                        |
| Branch          | `BRA XY`    | 6XY  | Branches to memory address XY unconditionally                                         |
| Branch Zero     | `BRZ XY`    | 7XY  | Branches to memory address XY if accumulator is zero                                  |
| Branch Positive | `BRP XY`    | 8XY  | Branches to memory address XY if accumulator is positive                              |
| Input           | `INP`       | 901  | Takes user input and stores it in the accumulator                                     |
| Output          | `OUT`       | 902  | Writes the content of the accumulator to shell                                        |
| Halt            | `HLT`       | 0    | Halts the execution of the program                                                    |
| Data            | `DAT`       | -    | Defines a data location or initializes memory (can include an optional initial value) |

## Usage

Initialize

```bash
cargo build
```

Compile a source file

```bash
./target/debug/lmc_emulator --build <filename>
```

Run the generated binary

```bash
./target/debug/lmc_emulator --run <filename>
```

## Examples

### Add two numbers

```
INP
STA 99
INP
ADD 99
OUT
HLT
```

### Add three numbers

```
INP
STA 99
INP
ADD 99
STA 99
INP
ADD 99
OUT
HLT
```

### Add/subtract

```
INP
STA FIRST
INP
ADD FIRST
OUT
INP
SUB FIRST
OUT
HLT
FIRST DAT
```

### Multiply two numbers

```
INP
STA NUM1
INP
STA NUM2
LOOP LDA TOTAL
ADD NUM1
STA TOTAL
LDA NUM2
SUB ONE
STA NUM2
BRP LOOP
LDA TOTAL
SUB NUM1
STA TOTAL
OUT
HLT
NUM1 DAT
NUM2 DAT
ONE DAT 1
TOTAL DAT 0
```

### Infinite loop

```
INPUT INP
OUT
BRA INPUT
```

### Pre-defining data

```
LDA VARIABLE
STA 99
VARIABLE DAT 1
```

### Find greater number

```
INP
STA FIRST
INP
STA SECOND
SUB FIRST
BRP HIGHER
LDA FIRST
BRA DONE
HIGHER LDA SECOND
DONE OUT
HLT
FIRST DAT
SECOND DAT
```

### Triangular numbers

```
INP
STA INPUT
LOOP LDA NUMBER
ADD COUNTER
OUT
STA NUMBER
LDA COUNTER
ADD ONE
STA COUNTER
LDA INPUT
SUB COUNTER
BRP LOOP
HLT
COUNTER DAT 1
NUMBER DAT 0
ONE DAT 1
INPUT DAT
```

### Count down

```
INP
LOOP OUT
STA COUNT
SUB ONE
STA COUNT
BRP LOOP
HLT
ONE DAT 1
COUNT DAT
```

### Fibonacci series

```
INP
STA N
START LDA N
BRZ FINISH
LDA A
OUT
LDA B
ADD A
STA 99
LDA B
STA A
LDA 99
STA B
LDA N
SUB Y
STA N
BRA START
FINISH LDA N
HLT
N DAT
A DAT 0
B DAT 1
Y DAT 1
```
