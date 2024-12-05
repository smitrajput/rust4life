# rust4life
Personal notes on my journey to mastering Rust on Solana.

## Daily Notes

### 2nd April, 2024
1. Real TPS only 898/s lol
2. Looks too verbose
3. Sol playground’s simple contract interaction failed, test SOL amount really beta (only 5 SOL in 24hrs, one deployment costs 4.5 SOL).
4. PBFT: https://pmg.csail.mit.edu/papers/osdi99.pdf 
5. Think of solana as an OS: accounts are files in linux
6. Solana’s executable accounts CAN initiate txns
7. Private keys for solana accounts?

### Dec 4
1. default types: i32, f64
2. usize is big enough to store ANY pointer / offset in a data structure
3. assert!(0.1+0.2==0.3); fails coz 0.1, 0.2’s default type is f64 so 0.1 = 0.100000000000002 etc, but (0.1_f32 + 0.2+f32 == 0.3) passes
4. for i in -3..2 { sum += i }     => sum -5
5. char size = 4 bytes and can hold ANY unicode char
6. bool size = 1 byte
7. fn not returning any value returns ‘unit’ type implicitly: (). Its size is 0 bytes
8. let x = 5u32; let z = { x = 2 * x; } => z = () (unit type, since x = 2 * x is a statement, hence returns nothing)
9. let z = { 2 * x } => z = 10_u32 (since 2 * x is an expression, so returns its value)
10. REMEMBER, ‘;’ makes an expression, a statement. Expressions might also end in ‘;’, making them a statement: 
    1. fn main(x: i32, y: i32) -> () { x+y;} => main’s return type is () as x+y; is a statement
11. fn parameters MUST define their types
12. diverging fn’s return type is ‘!’ implying it’ll never return to the caller
    1. fn main() -> ! { }
13. unimplemented!(), todo!(), panic!() are ways to implement diverging fn
14. Ownership
    1. a value can only have 1 owner at all times, be it a variable or a function
    2. once ownership is ‘moved’ from a variable, accessing this variable after will throw error
    3. mutability cab be changed while ‘moving’ ownership
    4. once ownership is ‘referenced’ from a variable, accessing this variable after will NOT throw error
    5. Move: transfers ownership of the value, Reference: creates a reference to owner of value (aka borrowing), Copy: sends copy of value
    6. Only 1 mutable reference possible OR many immutable references possible, but NOT both, in a given scope. Lemma: But possible in the same scope if immutable reference is not being accessed after definition of mutable reference and vice versa. Lemma is true also for 2 mutable references, not just 1 mut and 1 immut reference
    7. * - dereference, & - reference, ref - reference 
15. let s: Box<str> = "hello, world".into(); into() - converts static type to heap here (similar to ‘as’)
16. &str: pronounced string slice
17. Can ONLY append literals to a string, not another string
18. (to_string(), String::from()): &str to String, (as_str(), &): String to &str
19. r"Escapes don't work here: \x3F \u{211D}" prints Escapes don't work here: \x3F \u{211D}
20. String slice (i.e. &str) is a view into a heap-allocated string
21. let s1 = String::from("hi,中国"); let h1 = &s1[3..6]; => h1 is 中, as 中 is 3 bytes long
22. &String -> &str is implicitly convertible by compiler in rust
23. tuples with size > 12 cannot be printed directly with printlin!()


### Dec 5
24. All fields of a struct have to be initialised at the same time. Enum variants can be initialised separately in any order
25. Accessing
    1. Struct: struct_instance_name.var_name
    2. Enum: EnumName::EnumVariable
26. Struct fields are name: value vs Enum variants are mostly only name, but can be name = value too
27. [#derive(Debug)] trait and printlln!(“{:?}”,struct_instance_name OR EnumName::EnumVariable); to print compound types
28. names being accessed after the loop below will throw error coz ownership of its elements was taken by name
        let names = [String::from("liming"),String::from("hanmeimei")];
        for name in names {
            // Do something with name...
        }
29. Iterate the indexing and value in 'a'

        let a = [4, 3, 2, 1];
        for (i,v) in a.iter().enumerate() {
            println!("The {}th element is {}",i+1,v);
        }
