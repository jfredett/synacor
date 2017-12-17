# ASSEMBLY LANGUAGE FOR SYNACOR MACHINE

## ADDRESSES

Addresses are distinguisted from literals with a preceding `@`

## LITERALS

Literals are any unadorned number from 0 to 32768

## REGISTERS

Registers are written as `R0`, `R1`, etc up through `R7`

## INSTRUCTIONS

Supported instructions are:

* HALT
* SET
* PUSH
* POP
* EQ
* GT
* JMP
* JT
* JF
* ADD
* MULT
* MOD
* AND
* OR
* NOT
* RMEM
* WMEM
* CALL
* RET
* OUT
* IN
* NOOP

They are usually written in all caps, but that is not necessary.

## DIRECTIVES

    $START @ADDR

Set the start point of the program in memory (given by `@ADDR`, an address).
The first instruction will be written to this position.

    LABEL: <INSTRUCTION>

Labels the instruction with the given label. There is no current functionality
to jump to labels. Labels are not embedded in any way in the final binary
format, this is just convenience.

## EXAMPLE

Here is a program which computes the factorial of the value on the top of the
stack.


    $START 1000
       0: POP R0
       1: SET R1 1
       2: SET R2 1
       3: GT R7 R1 R0
       4:  JT R7 1006
       5:  JMP 1009
     6-T: ADD R1 R1 1
     7-T: MULT R2 R2 R1
     8-T: JMP 1002
     9-F: PUSH R2
    10-F: SET R0 0
    11-F: SET R1 0
    12-F: SET R2 0
    13-F: SET R7 0
      14: HALT


The labels to the side are purely aesthetic at this point.



