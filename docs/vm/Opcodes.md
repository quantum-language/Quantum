<div align="center">

# QVM Opcodes

This document describes the opcodes of the Quantum Virtual Machine (QVM).

![GitHub Version](https://img.shields.io/badge/version-0.0.1-inforamtional?style=for-the-badge&color=blueviolet&label=Current%20Version)

</div>

> **Note**
> This document is not final and is subject to change.

## Opcodes

The following opcodes are supported by the QVM:

### Stack

| Opcode | Description | Since | Implemented? |
| --- | --- | --- | --- |
| `LOAD` | Load a constant from the local variable table | 0.0.1 | No |
| `STORE` | Store a constant in the local variable table | 0.0.1 | No |
| `POP` | Pop a constant from the stack | 0.0.1 | No |
| `PEEK` | Peek at the top of the stack | 0.0.1 | No |
| `PEEK2` | Peek at the second item on the stack | 0.0.1 | No |
| `PUSH` | Push a constant onto the stack | 0.0.1 | No |

### Registers

| Opcode | Description | Since | Implemented? |
| --- | --- | --- | --- |
| `REGLOAD` | Load a constant from a register | 0.0.1 | No |
| `REGSTORE` | Store a constant in a register | 0.0.1 | No |
| `MOVE` | Move a constant from one register to another | 0.0.1 | No |

### Arithmetic

| Opcode | Description | Since | Implemented? |
| --- | --- | --- | --- |
| `ADD` | Add two constants | 0.0.1 | No |
| `SUB` | Subtract two constants | 0.0.1 | No |
| `MUL` | Multiply two constants | 0.0.1 | No |
| `DIV` | Divide two constants | 0.0.1 | No |
| `MOD` | Mod two constants | 0.0.1 | No |
| `NEG` | Negate an constants | 0.0.1 | No |
| `INC` | Increment an constants | 0.0.1 | No |
| `DEC` | Decrement an constants | 0.0.1 | No |

### Bitwise

| Opcode | Description | Since | Implemented? |
| --- | --- | --- | --- |
| `AND` | Bitwise AND two constants | 0.0.1 | No |
| `OR` | Bitwise OR two constants | 0.0.1 | No |
| `XOR` | Bitwise XOR two constants | 0.0.1 | No |
| `NOT` | Bitwise NOT two constants | 0.0.1 | No |
| `SHL` | Shift a constant left | 0.0.1 | No |
| `SHR` | Shift a constant right | 0.0.1 | No |
| `USHR` | Shift a constant right, zero fill | 0.0.1 | No |

### Comparison

| Opcode | Description | Since | Implemented? |
| --- | --- | --- | --- |
| `CMP` | Compare two constants | 0.0.1 | No |

### Control Flow

| Opcode | Description | Since | Implemented? |
| --- | --- | --- | --- |
| `GOTO` | Unconditionally jump to a label | 0.0.1 | No |
| `IFEQ` | Jump to a label if the top of the stack is equal to zero | 0.0.1 | No |
| `IFNE` | Jump to a label if the top of the stack is not equal to zero | 0.0.1 | No |
| `IFLE` | Jump to a label if the top of the stack is less than or equal to zero | 0.0.1 | No |
| `IFLT` | Jump to a label if the top of the stack is less than zero | 0.0.1 | No |
| `IFGE` | Jump to a label if the top of the stack is greater than or equal to zero | 0.0.1 | No |
| `IFGT` | Jump to a label if the top of the stack is greater than zero | 0.0.1 | No |
| `IFNULL` | Jump to a label if the top of the stack is null | 0.0.1 | No |
| `IFNONNULL` | Jump to a label if the top of the stack is not null | 0.0.1 | No |

### Object Creation

| Opcode | Description | Since | Implemented? |
| --- | --- | --- | --- |
| `NEW` | Create a new object | 0.0.1 | No |
| `NEWARRAY` | Create a new array | 0.0.1 | No |
| `ANEWARRAY` | Create a new array of references | 0.0.1 | No |
| `MULTIANEWARRAY` | Create a new multi-dimensional array | 0.0.1 | No |

### Object Access

| Opcode | Description | Since | Implemented? |
| --- | --- | --- | --- |
| `GETFIELD` | Get a field from an object | 0.0.1 | No |
| `PUTFIELD` | Set a field on an object | 0.0.1 | No |
| `GETSTATIC` | Get a static field | 0.0.1 | No |
| `PUTSTATIC` | Set a static field | 0.0.1 | No |
| `ARRAYLENGTH` | Get the length of an array | 0.0.1 | No |

### Object Manipulation

| Opcode | Description | Since | Implemented? |
| --- | --- | --- | --- |
| `CHECKCAST` | Check if an object is of a certain type | 0.0.1 | No |
| `INSTANCEOF` | Check if an object is of a certain type | 0.0.1 | No |
| `MONITORENTER` | Enter a monitor for an object | 0.0.1 | No |
| `MONITOREXIT` | Exit a monitor for an object | 0.0.1 | No |

### Method Invocation

| Opcode | Description | Since | Implemented? |
| --- | --- | --- | --- |
| `INVOKEVIRTUAL` | Invoke a virtual method | 0.0.1 | No |
| `INVOKESPECIAL` | Invoke a special method | 0.0.1 | No |
| `INVOKESTATIC` | Invoke a static method | 0.0.1 | No |

### Exception Handling

| Opcode | Description | Since | Implemented? |
| --- | --- | --- | --- |
| `THROW` | Throw an exception | 0.0.1 | No |
| `TRYCATCH` | Try to catch an exception | 0.0.1 | No |

### Miscellaneous

| Opcode | Description | Since | Implemented? |
| --- | --- | --- | --- |
| `NOP` | No operation | 0.0.1 | No |
| `BREAKPOINT` | Breakpoint | 0.0.1 | No |
| `RETURN` | Return a constant | 0.0.1 | No |