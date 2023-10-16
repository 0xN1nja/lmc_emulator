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

Input Two Numbers And Output Their Sum

```
INP
STA 99
INP
ADD 99
OUT
HLT
```

### Add Three Numbers

Input Three Numbers And Output Their Sum

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

Input Three Numbers, Output The Sum Of First Two Numbers And Then Output The Difference Of Third And First Number

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

### Infinite Loop

Input A Number And Continuously Output It

```
INPUT INP
OUT
BRA INPUT
```

### Pre-Defining Data

Load A Variable And Store It At Another Memory Address

```
LDA VARIABLE
STA 99
VARIABLE DAT 1
```

### Find Greater Number

Input Two Numbers And Output The Greater One

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
