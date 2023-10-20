# LMC Emulator

LMC Emulator Written In Rust

This Emulator Implements The Instructions Provided By [Peter Higginson's LMC](https://peterhigginson.co.uk/LMC) And York University's [LMC Instruction Set](https://www.yorku.ca/sychen/research/LMC/LMCInstructionSummary.html).

## Instructions Table

| Name            | Instruction | Code | Description                                                                           |
| --------------- | ----------- | ---- | ------------------------------------------------------------------------------------- |
| Add             | `ADD XY`    | 1XY  | Adds Value Of Memory Address XY To The Accumulator                                    |
| Subtract        | `SUB XY`    | 2XY  | Subtracts The Value Of Memory Address XY From The Accumulator                         |
| Store           | `STA XY`    | 3XY  | Stores The Accumulator's Value In Memory Address XY                                   |
| Load            | `LDA XY`    | 5XY  | Loads The Memory Address Of XY Into The Accumulator                                   |
| Branch          | `BRA XY`    | 6XY  | Branches To Memory Address XY Unconditionally                                         |
| Branch Zero     | `BRZ XY`    | 7XY  | Branches To Memory Address XY If Accumulator Is Zero                                  |
| Branch Positive | `BRP XY`    | 8XY  | Branches To Memory Address XY If Accumulator Is Positive                              |
| Input           | `INP`       | 901  | Takes User Input And Stores It In Accumulator                                         |
| Output          | `OUT`       | 902  | Writes Value Of Accumulator To Shell                                                  |
| Halt            | `HLT`       | 0    | Halts Execution Of The Program                                                        |
| Data            | `DAT`       | -    | Defines A Data Location Or Initializes Memory (Can Include An Optional Initial Value) |

## Usage

Initialize

```bash
$ cargo build
```

Compile A Source File

```bash
$ ./target/debug/lmc_emulator --build <filename>
```

Run The Generated Binary

```bash
$ ./target/debug/lmc_emulator --run <filename>
```

## Examples

### Add Two Numbers

```
INP
STA 99
INP
ADD 99
OUT
HLT
```

### Add Three Numbers

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

### Add/Subtract

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

### Multiply Two Numbers

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

### Infinite Loop

```
INPUT INP
OUT
BRA INPUT
```

### Pre-Defining Data

```
LDA VARIABLE
STA 99
VARIABLE DAT 1
```

### Find Greater Number

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

### Triangular Numbers

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

### Count Down

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

### Fibonacci Series

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
