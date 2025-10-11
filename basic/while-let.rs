/*
WHILE LET - Looping with Pattern Matching in Rust
================================================

WHAT IS WHILE LET?
- while let is a loop that continues while a pattern successfully matches
- Combines pattern matching with loop control
- Automatically exits when the pattern fails to match
- Perfect for iterating until a condition naturally ends
- More concise than manual loop + match combinations

REAL-WORLD ANALOGY:
- Like processing items from a delivery truck: "While there are packages to unload, keep unloading"
- Think of it as: "Keep doing this task while the pattern keeps matching"
- Like reading emails: "While I have unread messages, process the next one"
- Automatically stops when there's nothing left to process
- More elegant than checking "is there something?" before each loop iteration

SYNTAX:
while let PATTERN = EXPRESSION {
    // Code executes while pattern keeps matching
    // Loop automatically exits when pattern fails to match
}

WHEN TO USE WHILE LET:
✅ Processing items from iterators until exhausted
✅ Popping items from a stack/queue until empty
✅ Reading from channels/streams while data is available
✅ Unwrapping Option<T> values repeatedly
✅ Processing nested structures level by level
✅ Parsing sequences where end is indicated by pattern failure
✅ Consuming data structures that return Option<T>

WHILE LET vs ALTERNATIVES:
┌──────────────────┬─────────────────────────────────────┬─────────────────────────────────────┐
│     Pattern      │            WHILE LET                │          ALTERNATIVE                │
├──────────────────┼─────────────────────────────────────┼─────────────────────────────────────┤
│ Iterator         │ while let Some(x) = iter.next()     │ for x in iter (preferred)          │
│ Stack pop        │ while let Some(x) = stack.pop()     │ loop { match stack.pop() {...} }   │
│ Channel receive  │ while let Ok(x) = rx.recv()         │ loop { match rx.recv() {...} }     │
│ Nested unwrap    │ while let Some(x) = option { ... }  │ loop + if let (more verbose)       │
│ Conditional loop │ while let Some(x) = get_next() {...}│ loop { if let Some(x) = ... {...}} │
└──────────────────┴─────────────────────────────────────┴─────────────────────────────────────┘

COMPARISON: WHILE LET vs FOR vs LOOP:
┌─────────────────┬────────────────────────────┬────────────────────────────────────┐
│     Feature     │         WHILE LET          │           FOR/LOOP                 │
├─────────────────┼────────────────────────────┼────────────────────────────────────┤
│ Pattern match   │ Built-in                   │ for: No, loop: Manual with match  │
│ Auto-exit       │ When pattern fails         │ for: IntoIterator, loop: Manual   │
│ Verbosity       │ Concise                    │ for: Most concise, loop: Verbose  │
│ Use Case        │ Optional/Result sequences  │ for: Known sequences, loop: Manual│
│ Readability     │ Clear intent               │ for: Best for iteration           │
└─────────────────┴────────────────────────────┴────────────────────────────────────┘

KEY CONCEPTS:
- Pattern must implement exhaustive checking
- Loop exits on first pattern mismatch
- Variables bound in pattern are scoped to loop body
- Can use break/continue like regular loops
- More expressive than while condition + manual unwrapping

RELATED PATTERNS:
- if let - Single pattern match without looping
- for loop - Iterating over IntoIterator types
- loop + match - More explicit but verbose alternative
*/

fn main() {
    println!("🔄 === WHILE LET PATTERN MATCHING MASTERCLASS ===");

    // ========================================================================
    // 1. BASIC WHILE LET WITH ITERATORS
    // ========================================================================
    println!("\n📦 === BASIC WHILE LET WITH ITERATORS ===");

    let numbers = vec![1, 2, 3, 4, 5];
    let mut iter = numbers.into_iter();

    println!("Processing numbers with while let:");
    while let Some(num) = iter.next() {
        println!("  Processing: {}", num);
        println!("    Doubled: {}", num * 2);
    }
    println!("  All numbers processed!");

    // Compare with traditional for loop (usually preferred for simple iteration)
    println!("Same with for loop (more idiomatic):");
    let numbers = vec![1, 2, 3, 4, 5];
    for num in numbers {
        println!("  Processing: {}", num);
        println!("    Doubled: {}", num * 2);
    }

    // When while let is better: with manual control
    println!("While let with conditional processing:");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut iter = numbers.into_iter();

    while let Some(num) = iter.next() {
        println!("  Got: {}", num);
        if num == 5 {
            println!("    Found 5! Processing next 2 numbers specially:");

            // Manually control iterator within the loop
            if let Some(next1) = iter.next() {
                println!("      Special processing: {}", next1);
            }
            if let Some(next2) = iter.next() {
                println!("      Special processing: {}", next2);
            }
        }
    }

    // ========================================================================
    // 2. WHILE LET WITH STACK OPERATIONS (POP)
    // ========================================================================
    println!("\n📚 === WHILE LET WITH STACK OPERATIONS ===");

    // Stack simulation - LIFO (Last In, First Out)
    let mut stack = vec!["bottom", "middle-1", "middle-2", "top"];

    println!("Stack contents (bottom to top): {:?}", stack);
    println!("Popping items with while let:");

    while let Some(item) = stack.pop() {
        println!("  Popped: '{}'", item);
        println!("    Remaining items: {}", stack.len());
    }
    println!("  Stack is empty!");

    // Real-world example: Undo stack
    #[derive(Debug, Clone)]
    struct Action {
        description: String,
        timestamp: u64,
    }

    let mut undo_stack = vec![
        Action {
            description: "Created file".to_string(),
            timestamp: 1001,
        },
        Action {
            description: "Edited line 5".to_string(),
            timestamp: 1002,
        },
        Action {
            description: "Added function".to_string(),
            timestamp: 1003,
        },
        Action {
            description: "Saved file".to_string(),
            timestamp: 1004,
        },
    ];

    println!("Undo system - reverting last 2 actions:");
    let mut undo_count = 0;
    while let Some(action) = undo_stack.pop() {
        println!(
            "  Undoing: {} (timestamp: {})",
            action.description, action.timestamp
        );
        undo_count += 1;

        if undo_count >= 2 {
            println!("  Undo limit reached!");
            break;
        }
    }
    println!("  Remaining actions: {:?}", undo_stack);

    // ========================================================================
    // 3. WHILE LET WITH CUSTOM OPTION-RETURNING FUNCTIONS
    // ========================================================================
    println!("\n🎲 === WHILE LET WITH CUSTOM OPTION FUNCTIONS ===");

    // Simulating a data source that eventually runs out
    struct DataSource {
        data: Vec<String>,
        index: usize,
    }

    impl DataSource {
        fn new(data: Vec<String>) -> Self {
            DataSource { data, index: 0 }
        }

        fn fetch_next(&mut self) -> Option<String> {
            if self.index < self.data.len() {
                let item = self.data[self.index].clone();
                self.index += 1;
                Some(item)
            } else {
                None
            }
        }
    }

    let mut source = DataSource::new(vec![
        "Record 1".to_string(),
        "Record 2".to_string(),
        "Record 3".to_string(),
        "Record 4".to_string(),
    ]);

    println!("Fetching records from data source:");
    let mut record_count = 0;
    while let Some(record) = source.fetch_next() {
        record_count += 1;
        println!("  Record {}: {}", record_count, record);
    }
    println!("  All records fetched! Total: {}", record_count);

    // ========================================================================
    // 4. WHILE LET WITH NESTED OPTION STRUCTURES
    // ========================================================================
    println!("\n🪆 === WHILE LET WITH NESTED STRUCTURES ===");

    #[derive(Debug)]
    struct Node {
        value: i32,
        next: Option<Box<Node>>,
    }

    // Create a linked list: 1 -> 2 -> 3 -> 4
    let list = Node {
        value: 1,
        next: Some(Box::new(Node {
            value: 2,
            next: Some(Box::new(Node {
                value: 3,
                next: Some(Box::new(Node {
                    value: 4,
                    next: None,
                })),
            })),
        })),
    };

    println!("Traversing linked list:");
    let mut current = Some(&list);
    let mut position = 1;

    while let Some(node) = current {
        println!("  Position {}: value = {}", position, node.value);
        current = node.next.as_ref().map(|boxed| boxed.as_ref());
        position += 1;
    }

    // ========================================================================
    // 5. WHILE LET WITH RESULT TYPE
    // ========================================================================
    println!("\n✅ === WHILE LET WITH RESULT TYPE ===");

    // Simulating operations that can fail
    struct TaskQueue {
        tasks: Vec<Result<String, String>>,
        index: usize,
    }

    impl TaskQueue {
        fn new(tasks: Vec<Result<String, String>>) -> Self {
            TaskQueue { tasks, index: 0 }
        }

        fn get_next(&mut self) -> Option<Result<String, String>> {
            if self.index < self.tasks.len() {
                let task = self.tasks[self.index].clone();
                self.index += 1;
                Some(task)
            } else {
                None
            }
        }
    }

    let mut queue = TaskQueue::new(vec![
        Ok("Task 1: Success".to_string()),
        Ok("Task 2: Success".to_string()),
        Err("Task 3: Failed - Network error".to_string()),
        Ok("Task 4: Success".to_string()),
        Err("Task 5: Failed - Timeout".to_string()),
    ]);

    println!("Processing task queue:");
    let mut success_count = 0;
    let mut failure_count = 0;

    while let Some(task_result) = queue.get_next() {
        match task_result {
            Ok(task) => {
                success_count += 1;
                println!("  ✅ {}", task);
            }
            Err(error) => {
                failure_count += 1;
                println!("  ❌ {}", error);
            }
        }
    }

    println!(
        "  Summary: {} succeeded, {} failed",
        success_count, failure_count
    );

    // ========================================================================
    // 6. WHILE LET WITH BREAK AND CONTINUE
    // ========================================================================
    println!("\n🔀 === WHILE LET WITH BREAK AND CONTINUE ===");

    let mut data = vec![1, 2, 3, -1, 5, 6, -2, 8, 9, 10];

    println!("Processing until negative number:");
    while let Some(num) = data.pop() {
        if num < 0 {
            println!("  Found negative number: {}", num);
            println!("  Stopping processing!");
            break;
        }
        println!("  Processing: {}", num);
    }
    println!("  Remaining items: {:?}", data);

    // Continue example
    let data = vec![Some(1), None, Some(3), Some(4), None, Some(6)];
    let mut iter = data.into_iter();

    println!("Processing with skip on None:");
    while let Some(item) = iter.next() {
        if item.is_none() {
            println!("  Skipping None value");
            continue;
        }

        if let Some(value) = item {
            println!("  Processing value: {}", value);
        }
    }

    // ========================================================================
    // 7. WHILE LET WITH ENUMS AND PATTERN MATCHING
    // ========================================================================
    println!("\n🎭 === WHILE LET WITH ENUMS ===");

    #[derive(Debug, Clone)]
    enum Token {
        Number(i32),
        Plus,
        Minus,
        Multiply,
        Divide,
        EndOfExpression,
    }

    struct TokenStream {
        tokens: Vec<Token>,
        index: usize,
    }

    impl TokenStream {
        fn new(tokens: Vec<Token>) -> Self {
            TokenStream { tokens, index: 0 }
        }

        fn next_token(&mut self) -> Option<Token> {
            if self.index < self.tokens.len() {
                let token = self.tokens[self.index].clone();
                self.index += 1;
                Some(token)
            } else {
                None
            }
        }
    }

    let mut stream = TokenStream::new(vec![
        Token::Number(5),
        Token::Plus,
        Token::Number(3),
        Token::Multiply,
        Token::Number(2),
        Token::EndOfExpression,
    ]);

    println!("Parsing token stream:");
    let mut token_count = 0;

    while let Some(token) = stream.next_token() {
        token_count += 1;

        match token {
            Token::Number(n) => println!("  Token {}: Number({})", token_count, n),
            Token::Plus => println!("  Token {}: Plus", token_count),
            Token::Minus => println!("  Token {}: Minus", token_count),
            Token::Multiply => println!("  Token {}: Multiply", token_count),
            Token::Divide => println!("  Token {}: Divide", token_count),
            Token::EndOfExpression => {
                println!("  Token {}: EndOfExpression", token_count);
                println!("  Stopping parsing!");
                break;
            }
        }
    }

    // ========================================================================
    // 8. REAL-WORLD EXAMPLES
    // ========================================================================
    println!("\n🌍 === REAL-WORLD EXAMPLES ===");

    // Example 1: Command processor
    #[derive(Debug, Clone)]
    enum Command {
        Start(String),
        Stop(String),
        Restart(String),
        Status(String),
        Exit,
    }

    let mut commands = vec![
        Command::Start("web-server".to_string()),
        Command::Status("web-server".to_string()),
        Command::Stop("database".to_string()),
        Command::Restart("cache".to_string()),
        Command::Status("cache".to_string()),
        Command::Exit,
    ];

    println!("Command processor:");
    while let Some(command) = commands.pop() {
        match command {
            Command::Start(service) => {
                println!("  🚀 Starting service: {}", service);
            }
            Command::Stop(service) => {
                println!("  🛑 Stopping service: {}", service);
            }
            Command::Restart(service) => {
                println!("  🔄 Restarting service: {}", service);
            }
            Command::Status(service) => {
                println!("  ℹ️  Status check for: {}", service);
            }
            Command::Exit => {
                println!("  🚪 Exit command received");
                println!("  Gracefully shutting down...");
                break;
            }
        }
    }

    // Example 2: Event processing with filtering
    #[derive(Debug, Clone)]
    enum Event {
        UserLogin {
            user_id: u32,
            timestamp: u64,
        },
        UserLogout {
            user_id: u32,
            timestamp: u64,
        },
        Purchase {
            user_id: u32,
            amount: f64,
            timestamp: u64,
        },
        Error {
            message: String,
            timestamp: u64,
        },
    }

    let mut events = vec![
        Event::UserLogin {
            user_id: 1,
            timestamp: 1000,
        },
        Event::Purchase {
            user_id: 1,
            amount: 29.99,
            timestamp: 1001,
        },
        Event::Error {
            message: "Connection timeout".to_string(),
            timestamp: 1002,
        },
        Event::Purchase {
            user_id: 2,
            amount: 49.99,
            timestamp: 1003,
        },
        Event::UserLogout {
            user_id: 1,
            timestamp: 1004,
        },
        Event::Error {
            message: "Database unreachable".to_string(),
            timestamp: 1005,
        },
    ];

    println!("Processing events (most recent first):");
    let mut error_count = 0;
    let max_errors = 2;

    while let Some(event) = events.pop() {
        match event {
            Event::UserLogin { user_id, timestamp } => {
                println!("  👤 User {} logged in at {}", user_id, timestamp);
            }
            Event::UserLogout { user_id, timestamp } => {
                println!("  👋 User {} logged out at {}", user_id, timestamp);
            }
            Event::Purchase {
                user_id,
                amount,
                timestamp,
            } => {
                println!(
                    "  💰 User {} purchased ${:.2} at {}",
                    user_id, amount, timestamp
                );
            }
            Event::Error { message, timestamp } => {
                error_count += 1;
                println!("  ⚠️  Error at {}: {}", timestamp, message);

                if error_count >= max_errors {
                    println!("  🚨 Too many errors! Stopping event processing.");
                    println!("  Remaining events: {}", events.len());
                    break;
                }
            }
        }
    }

    // Example 3: Batch processing with size limits
    #[derive(Debug)]
    struct DataBatch {
        items: Vec<String>,
        size_bytes: usize,
    }

    impl DataBatch {
        fn new() -> Self {
            DataBatch {
                items: Vec::new(),
                size_bytes: 0,
            }
        }

        fn add(&mut self, item: String) {
            self.size_bytes += item.len();
            self.items.push(item);
        }

        fn is_full(&self, max_size: usize) -> bool {
            self.size_bytes >= max_size
        }
    }

    let data_items = vec![
        "item1".to_string(),
        "item2_longer".to_string(),
        "item3".to_string(),
        "item4_very_long_string".to_string(),
        "item5".to_string(),
        "item6_medium".to_string(),
        "item7".to_string(),
    ];

    let mut items_iter = data_items.into_iter();
    let max_batch_size = 30; // bytes
    let mut batch_number = 0;

    println!("Batch processing with size limits:");

    while let Some(first_item) = items_iter.next() {
        batch_number += 1;
        let mut batch = DataBatch::new();
        batch.add(first_item);

        println!("  📦 Batch {}:", batch_number);

        // Fill batch until size limit
        while let Some(item) = items_iter.next() {
            if batch.is_full(max_batch_size) {
                println!("    Size limit reached at {} bytes", batch.size_bytes);
                // Put the item back by processing it in next batch
                // In real code, you might use peekable() iterator
                println!("    Items in batch: {:?}", batch.items);

                // Start new batch with this item
                batch_number += 1;
                batch = DataBatch::new();
                batch.add(item);
                println!("  📦 Batch {}:", batch_number);
            } else {
                batch.add(item);
            }
        }

        println!("    Final size: {} bytes", batch.size_bytes);
        println!("    Items in batch: {:?}", batch.items);
        break; // Exit outer loop after processing all items
    }

    // ========================================================================
    // 9. COMPARISON: DIFFERENT LOOP APPROACHES
    // ========================================================================
    println!("\n⚖️  === COMPARISON: DIFFERENT LOOP APPROACHES ===");

    println!("Processing same data with different approaches:");

    // Approach 1: while let
    println!("1. While let:");
    let mut stack = vec![1, 2, 3, 4, 5];
    while let Some(item) = stack.pop() {
        println!("   {}", item);
    }

    // Approach 2: loop with match
    println!("2. Loop with match:");
    let mut stack = vec![1, 2, 3, 4, 5];
    loop {
        match stack.pop() {
            Some(item) => println!("   {}", item),
            None => break,
        }
    }

    // Approach 3: loop with if let
    println!("3. Loop with if let:");
    let mut stack = vec![1, 2, 3, 4, 5];
    loop {
        if let Some(item) = stack.pop() {
            println!("   {}", item);
        } else {
            break;
        }
    }

    // Approach 4: for loop (when applicable)
    println!("4. For loop (reversed):");
    let stack = vec![1, 2, 3, 4, 5];
    for item in stack.iter().rev() {
        println!("   {}", item);
    }

    println!("Analysis:");
    println!("  ✅ while let: Most concise for Option-returning operations");
    println!("  ⚙️  loop + match: More explicit but verbose");
    println!("  🔄 loop + if let: Readable but unnecessary nesting");
    println!("  🎯 for loop: Best when you have an iterator and don't need to mutate");

    // ========================================================================
    // 10. ADVANCED PATTERNS AND BEST PRACTICES
    // ========================================================================
    println!("\n🎓 === ADVANCED PATTERNS AND BEST PRACTICES ===");

    // Pattern 1: Peekable iterator with while let
    println!("Pattern 1: Peekable iterator:");
    let numbers = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
    let mut iter = numbers.iter().peekable();

    while let Some(&num) = iter.next() {
        let mut count = 1;

        // Peek ahead to count consecutive equal numbers
        while let Some(&&next_num) = iter.peek() {
            if next_num == num {
                count += 1;
                iter.next();
            } else {
                break;
            }
        }

        println!("  Number {} appears {} time(s)", num, count);
    }

    // Pattern 2: Multiple while let in sequence
    println!("Pattern 2: Sequential processing:");
    let mut high_priority = vec!["urgent1", "urgent2"];
    let mut normal_priority = vec!["normal1", "normal2", "normal3"];

    println!("  Processing high priority:");
    while let Some(task) = high_priority.pop() {
        println!("    ⚡ {}", task);
    }

    println!("  Processing normal priority:");
    while let Some(task) = normal_priority.pop() {
        println!("    📋 {}", task);
    }

    // Pattern 3: while let with mutable state
    println!("Pattern 3: Stateful processing:");
    let data = vec![10, 20, 30, 40, 50];
    let mut iter = data.into_iter();
    let mut running_sum = 0;
    let threshold = 60;

    while let Some(num) = iter.next() {
        running_sum += num;
        println!("  Added {}, running sum: {}", num, running_sum);

        if running_sum >= threshold {
            println!("  Threshold {} reached!", threshold);
            break;
        }
    }

    println!("\n📋 === WHILE LET SUMMARY ===");
    println!("✅ while let PATTERN = EXPRESSION - loop while pattern matches");
    println!("✅ Perfect for processing sequences that return Option<T>");
    println!("✅ Automatically exits when pattern fails to match");
    println!("✅ More concise than loop + match for simple cases");
    println!("✅ Works with iterators, stacks, queues, and custom types");
    println!("✅ Supports break and continue for flow control");
    println!("✅ Ideal for: stack.pop(), iterator.next(), channel.recv()");
    println!("🎯 Choose while let for clean, pattern-based looping!");
    println!("💡 Prefer 'for' loop when you don't need pattern matching!");
}
