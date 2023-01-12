<div align="center">

# VM Opcodes

This document describes the opcodes of the Quantum Virtual Machine (QVM).

</div>

> **Note**
> This document is not final and is subject to change.

## Opcodes

The following opcodes are supported by the QVM:

### Stack

| Opcode | Description | Stack | Notes |
| --- | --- | --- | --- |
| `ILOAD` | Load an integer from the local variable table | | |
| `FLOAD` | Load a float from the local variable table | | |
| `LLOAD` | Load a long from the local variable table | | |
| `DLOAD` | Load a double from the local variable table | | |
| `ALOAD` | Load a reference from the local variable table | | |
| `ISTORE` | Store an integer in the local variable table | | |
| `FSTORE` | Store a float in the local variable table | | |
| `LSTORE` | Store a long in the local variable table | | |
| `DSTORE` | Store a double in the local variable table | | |
| `ASTORE` | Store a reference in the local variable table | | |
| `IPOP` | Pop an integer from the stack | | |
| `FPOP` | Pop a float from the stack | | |
| `LPOP` | Pop a long from the stack | | |
| `DPOP` | Pop a double from the stack | | |
| `APOP` | Pop a reference from the stack | | |
| `IPEEK` | Peek at the top of the stack | | |
| `FPEEK` | Peek at the top of the stack | | |
| `LPEEK` | Peek at the top of the stack | | |
| `DPEEK` | Peek at the top of the stack | | |
| `APEEK` | Peek at the top of the stack | | |
| `IPEEK2` | Peek at the second item on the stack | | |
| `FPEEK2` | Peek at the second item on the stack | | |
| `LPEEK2` | Peek at the second item on the stack | | |
| `DPEEK2` | Peek at the second item on the stack | | |
| `APEEK2` | Peek at the second item on the stack | | |
| `IPUSH` | Push an integer onto the stack | | |
| `FPUSH` | Push a float onto the stack | | |
| `LPUSH` | Push a long onto the stack | | |
| `DPUSH` | Push a double onto the stack | | |
| `APUSH` | Push a reference onto the stack | | |

### Arithmetic

| Opcode | Description | Stack | Notes |
| --- | --- | --- | --- |
| `IADD` | Add two integers | | |
| `FADD` | Add two floats | | |



## Prefixes

The Quantum Virtual Machine's opcodes are prefixed using a type (like the JVM does). The following prefixes are supported:

* `I` - Integer
* `F` - Float
* `L` - Long
* `D` - Double
* `A` - Reference
* `B` - Byte
* `C` - Character
* `S` - Short
* `Z` - Boolean