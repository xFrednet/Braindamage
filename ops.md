# Addition

```brainfuck
> Skip Home
+>+++> Load 3
+>++++< Load 4

# Addition
>[-<<+>>]
<-<<  Empty Head
```

# Subtraction

```brainfuck
> Skip Home
+>++++> Load 4
+>+++< Load 3

# subtraction
>[-<<->>]
<-<<  Empty Head
```

# Mulltiplication

```brianfuck
>+>++++>+>+++< Load 4 & 3

# Multiplication
>[->+<]
<-<[->+<]
>>>
[-<<
| [-<+>>+<]>[-<+>]
| >
]
<<[-]<<
```

# Division

```
> Skip Home
+>+++++++> Load 7
+>+++< Load 3

# Division
-<[->+<]
>
[ While dividend can be decremented(divided)
  -                // Dec dividend
  >                // To divisor
  ->+<             // Dec divisor
  [>>]             // Move to empty buffer to prevent coping
  >[
    [-<+>]         // Reset divisor
    <<<+>>>>>      // Inc Result
  ]
  <<<<             // Go to: dividend
]
>[-]               // Clear divisor part
>[-]               // Clear rest part
<<<
```

## Allocation (WIP Would need 4 bytes)



>>> // Skip Home
+>++> // Load Dummy 2
+>+++< // Load 3

# Malloc
>
[- // While we need another byte
  <[<<]< // goto home
  <[<<<] // Skip Allocated Memory
  <[<<<] // Skip potentials
  +>+ // Mark as potential and claimed
  [>>>] // goto home
  >[>>] // goto head
  < // select data
]
