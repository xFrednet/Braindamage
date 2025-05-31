# Memory

A document used for prototyping memory layout for braindamage.

*Franklin Pezzuti Dyer* has written an excellent blog post about how memory management can be done in BF: https://franklin.dyer.me/post/209

## Memory Chunks

```
+---+---+---+---+
| 1 | 2 | 3 | 4 |
+---+---+---+---+
```

1. **Jump limiter**: Usually 1, it can be set to 0 to *jump* between memory location. `[>]`
3. **In Use**: A bit flag indicating if this cell is in use or not. (0: Empty, 1: filled)
2. **Scratch Cell**: A cell used for temp values. Each operation might use this cell differently.
4. **Data Cell**: The data stored in the cell.
