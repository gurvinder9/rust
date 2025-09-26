# Rust Architecture: From High Level to Low Level

## Overview
This document explains how Rust code transforms from source code to running binary, using `a2.rs` as a practical example.

## 1. High Level: Source Code Structure

### Your Rust Code (a2.rs)
```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let age = 15;
    if age > 20 {
        println!("What would you like to drink?");
    } else {
        println!("Sorry, you don't have enough years?");
    }
    println!("Sums is {:?}", add(1, 2));
}
```

**Key Components:**
- **Functions**: `add()` and `main()` with clear signatures
- **Variables**: `age` with type inference (`i32`)
- **Control Flow**: `if/else` branching logic
- **Macros**: `println!` for formatted output
- **Memory Safety**: Rust's ownership system prevents memory issues

## 2. Compilation Pipeline

### Stage 1: Lexing and Parsing
```
Source Code → AST (Abstract Syntax Tree)
```
- **Lexer**: Breaks source into tokens (`fn`, `add`, `(`, `a`, `:`, `i32`, etc.)
- **Parser**: Creates tree structure representing code hierarchy
- **Result**: AST representing program structure

**Example Token Stream from a2.rs:**
```
fn add ( a : i32 , b : i32 ) -> i32 { a + b }
```

### Stage 2: HIR (High-level Intermediate Representation)
```
AST → HIR
```
- **Desugaring**: Expands macros like `println!`
- **Name Resolution**: Links variable/function names to definitions
- **Type Checking**: Ensures type safety and ownership rules
- **Borrow Checking**: Validates memory safety without garbage collection

**Macro Expansion Example:**
```rust
// Before: println!("Sums is {:?}", add(1, 2));
// After: std::io::_print(core::fmt::Arguments::new_v1(...));
```

### Stage 3: MIR (Mid-level Intermediate Representation)
```
HIR → MIR
```
MIR from your a2.rs shows control flow graphs:

```mir
fn add(_1: i32, _2: i32) -> i32 {
    bb0: {
        _3 = AddWithOverflow(copy _1, copy _2);
        assert(!move (_3.1: bool), "attempt to compute `{} + {}`, which would overflow", copy _1, copy _2);
    }
    bb1: {
        _0 = move (_3.0: i32);
        return;
    }
}

fn main() -> () {
    bb0: {
        _2 = const 15_i32;                    // age = 15
        _1 = Gt(move _2, const 20_i32);      // age > 20
        switchInt(move _1) -> [0: bb3, otherwise: bb1];  // if/else branching
    }
    bb1: { /* print drink message */ }
    bb3: { /* print age message */ }
}
```

**Key Features:**
- **Basic Blocks (bb0, bb1, bb3)**: Single-entry, single-exit code segments
- **Overflow Checking**: `AddWithOverflow` automatically checks for integer overflow
- **Explicit Memory Operations**: `copy`, `move` operations made explicit
- **Control Flow**: `switchInt` for branching, `Gt` for comparisons
- **Safety Assertions**: Panic on overflow with detailed error messages

### Stage 4: LLVM IR (Low-level Intermediate Representation)
```
MIR → LLVM IR
```

**Example from a2.rs LLVM IR:**
```llvm
define internal i32 @_ZN2a23add17h... (i32 %0, i32 %1) {
start:
  %2 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %0, i32 %1)
  %3 = extractvalue { i32, i1 } %2, 1
  br i1 %3, label %panic, label %bb1

panic:
  call void @_ZN4core9panicking5panic17h...
  unreachable

bb1:
  %4 = extractvalue { i32, i1 } %2, 0
  ret i32 %4
}
```

**Key Features:**
- **Platform Independent**: Same IR works for ARM64, x86_64, etc.
- **SSA Form**: Single Static Assignment - each variable assigned exactly once
- **Intrinsics**: `@llvm.sadd.with.overflow.i32` for checked arithmetic
- **Optimization Ready**: Dead code elimination, constant propagation, inlining

### Stage 5: Assembly Generation
```
LLVM IR → Assembly Code
```

**ARM64 Assembly from a2.rs:**
```assembly
__ZN2a23add17h...:                    // add function
    sub sp, sp, #32                   // Allocate stack space
    stp x29, x30, [sp, #16]          // Save frame pointer and link register
    add x29, sp, #16                  // Set up frame pointer

    adds w0, w0, w1                   // Add with overflow check
    b.vs LBB0_1                       // Branch if overflow

    ldp x29, x30, [sp, #16]          // Restore registers
    add sp, sp, #32                   // Deallocate stack
    ret                               // Return to caller

LBB0_1:                              // Overflow panic
    bl  _ZN4core9panicking5panic17h... // Call panic handler
```

**Key Features:**
- **Platform Specific**: ARM64 instructions (`adds`, `b.vs`, `stp`)
- **Register Allocation**: Optimized use of CPU registers
- **Stack Management**: Precise stack frame setup/teardown
- **Overflow Detection**: Hardware-level overflow checking

### Stage 6: Machine Code
```
Assembly → Binary Executable
```
- **Assembler**: Converts assembly to object files (.o)
- **Linker**: Combines object files with system libraries
- **Symbol Resolution**: Links function calls to actual addresses
- **Final Binary**: Executable machine code with entry point

## 3. Memory Management Architecture

### Stack vs Heap in a2.rs
```rust
fn main() {
    let age = 15;           // Stack: i32 value (4 bytes)
    // Stack frame layout:
    // [age: i32] [locals for println] [return address]
}

fn add(a: i32, b: i32) -> i32 {
    // New stack frame:
    // [a: i32] [b: i32] [return value] [parent frame pointer]
    a + b  // Result computed on stack
}
```

**Memory Layout Analysis:**
- **Stack Growth**: Each function call pushes new frame
- **Automatic Cleanup**: Stack unwinding when functions return
- **No Heap Usage**: Simple values like `i32` live entirely on stack
- **Predictable Performance**: No dynamic allocation overhead

### Ownership System Deep Dive

#### Rule 1: Single Ownership
```rust
let value = String::from("hello");  // value owns the string
let moved = value;                  // Ownership transferred to 'moved'
// println!("{}", value);           // ERROR: value no longer valid
```

#### Rule 2: Automatic Drop
```rust
{
    let data = vec![1, 2, 3];      // data owns heap allocation
}   // Drop::drop() called automatically, heap memory freed
```

#### Rule 3: Borrowing
```rust
fn process_data(data: &Vec<i32>) { }    // Immutable borrow
fn modify_data(data: &mut Vec<i32>) { } // Mutable borrow

// Borrow checker ensures:
// - No use after free
// - No data races
// - No dangling pointers
```

### Compile-Time vs Runtime
```rust
// Compile time: Borrow checker validates these rules
fn safe_function() {
    let mut data = vec![1, 2, 3];
    let reference = &mut data;
    // let another = &data;  // ERROR: Cannot borrow as immutable while mutable borrow exists
}

// Runtime: Zero overhead - no garbage collection needed
// Memory freed immediately when owner goes out of scope
```

## 4. Runtime Architecture

### Zero-Cost Abstractions
- **Compile Time**: Complex ownership checking
- **Runtime**: No overhead - performs like C/C++

### Memory Safety Without GC
- **Compile-time checks** prevent memory leaks, double-free, use-after-free
- **No runtime garbage collection** pause

### Concurrency Model
```rust
use std::thread;
use crossbeam_channel::unbounded;

// Safe concurrent programming without data races
```

## 5. Binary Structure

### Executable Layout
```
┌─────────────────┐
│   Text Segment  │ ← Your compiled functions (add, main)
├─────────────────┤
│   Data Segment  │ ← Global/static variables
├─────────────────┤
│    BSS Segment  │ ← Uninitialized data
├─────────────────┤
│      Heap       │ ← Dynamic allocations (Vec, Box, etc.)
├─────────────────┤
│      Stack      │ ← Local variables, function calls
└─────────────────┘
```

### How a2.rs Executes
1. **Program Load**: OS loads binary into memory
2. **Stack Setup**: Creates stack frame for `main()`
3. **Variable Storage**: `age = 15` stored on stack
4. **Condition Check**: `age > 20` evaluation
5. **Function Call**: `add(1, 2)` creates new stack frame
6. **Return**: Result stored and printed
7. **Cleanup**: Stack frames automatically unwound

## 6. Debugging and Inspection

### Compilation Artifacts
```bash
# Generate all intermediate representations
cargo rustc --bin a2 -- --emit=asm,llvm-ir,mir

# Generated files:
# a2-{hash}.mir  ← Mid-level IR
# a2-{hash}.ll   ← LLVM IR
# a2-{hash}.s    ← Assembly
```

### Memory Layout Inspection
```bash
# View symbols in binary
nm target/debug/a2

# View assembly with debugging info
objdump -d target/debug/a2
```

## 7. Performance Characteristics

### Compile Time vs Runtime
- **Heavy compile-time analysis** ensures safety
- **Zero-cost abstractions** at runtime
- **Aggressive optimizations** possible due to ownership guarantees

### Compared to Other Languages
- **Memory Safety**: Like Java/C# but without GC overhead
- **Performance**: Like C/C++ but with safety guarantees
- **Concurrency**: Built-in safety prevents data races

## 8. Practical Exploration Commands

### Inspect Compilation Stages
```bash
# From activities/ directory:

# 1. Compile with all intermediate outputs
cargo rustc --bin a2 -- --emit=asm,llvm-ir,mir

# 2. View MIR (control flow graphs)
cat target/debug/deps/a2-*.mir

# 3. View LLVM IR (optimization-ready)
cat target/debug/deps/a2-*.ll | head -50

# 4. View Assembly (platform-specific)
cat target/debug/deps/a2-*.s | head -50

# 5. Examine binary symbols
nm target/debug/a2

# 6. Disassemble compiled binary
objdump -d target/debug/a2 | head -30
```

### Debug Build vs Release Build
```bash
# Debug build (your current a2.rs)
cargo run --bin a2
# - Contains overflow checks
# - Includes debug symbols
# - No optimizations

# Release build
cargo run --bin a2 --release
# - Aggressive optimizations
# - Overflow checks may be optimized away
# - Smaller binary size
```

### Memory Usage Analysis
```bash
# Check binary size
ls -la target/debug/a2
ls -la target/release/a2

# Runtime memory usage
time cargo run --bin a2
```

## 9. Understanding Your a2.rs Execution

### Step-by-Step Execution Flow

1. **Binary Load**: OS loads executable into memory at specific address
2. **Runtime Setup**: Rust runtime initializes, sets up stack
3. **main() Entry**:
   - Stack frame created
   - `age = 15` stored as 4-byte i32 on stack
4. **Condition Evaluation**:
   - `age > 20` → `15 > 20` → `false`
   - Branch prediction: CPU likely predicts false path
5. **String Output**:
   - `println!` macro expanded to format + write syscalls
   - String literals stored in binary's data segment
6. **Function Call**:
   - `add(1, 2)` pushes new stack frame
   - Parameters `a=1, b=2` copied to new frame
   - Overflow check: `1 + 2` cannot overflow i32
   - Result `3` returned to caller's stack
7. **Final Output**: Result formatted and printed
8. **Cleanup**: Stack frames unwound, program exits with status 0

### Binary Layout in Memory
```
Address Space Layout for a2:
┌─────────────────┐ ← 0x7fff...
│     Stack       │   [main frame: age=15]
│       ↓         │   [add frame: a=1, b=2, result=3]
├─────────────────┤ ← heap grows up
│      Heap       │   (unused in a2.rs)
│       ↑         │
├─────────────────┤ ← 0x100...
│   Data Segment  │   "Sorry, you don't..." strings
├─────────────────┤
│   Text Segment  │   add() machine code
│                 │   main() machine code
└─────────────────┘ ← 0x100000000 (program base)
```

## Key Takeaways for Beginners

1. **Rust compiles to native machine code** - no interpreter or VM
2. **Ownership system eliminates entire classes of bugs** at compile time
3. **Multiple compilation stages** each add safety and optimization
4. **Zero runtime overhead** for memory safety features
5. **Deterministic performance** - no unexpected GC pauses
6. **Your simple a2.rs becomes sophisticated machine code** with automatic safety checks
