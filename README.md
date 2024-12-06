# rust4life
Personal notes on my journey to mastering Rust.

## Daily Notes

### Dec 4
1. default types: `i32`, `f64`
2. `usize` is big enough to store ANY pointer / offset in a data structure
3. `assert!(0.1+0.2==0.3);` fails coz `0.1`, `0.2`’s default type is `f64` so `0.1 = 0.100000000000002` etc, but `(0.1_f32 + 0.2+f32 == 0.3)` passes
4. for `i` in `-3..2` { `sum += i` }     => `sum = -5`
5. `char` size = 4 bytes and can hold ANY unicode char
6. `bool` size = 1 byte
7. fn not returning any value returns ‘unit’ type implicitly: `()`. Its size is 0 bytes
8. `let x = 5u32; let z = { x = 2 * x; } => z = ()` (unit type, since `x = 2 * x` is a statement, hence returns nothing)
9. `let z = { 2 * x } => z = 10_u32` (since `2 * x` is an expression, so returns its value)
10. REMEMBER, ‘;’ makes an expression, a statement. Expressions might also end in ‘;’, making them a statement: 
    1. `fn main(x: i32, y: i32) -> () { x+y;}` => main’s return type is `()` as `x+y;` is a statement
11. fn parameters MUST define their types
12. diverging fn’s return type is ‘!’ implying it’ll never return to the caller
    1. `fn main() -> ! { }`
13. `unimplemented!()`, `todo!()`, `panic!()` are ways to implement diverging fn
14. Ownership
    1. a value can only have 1 owner at all times, be it a variable or a function
    2. once ownership is ‘moved’ from a variable, accessing this variable after will throw error
    3. mutability cab be changed while ‘moving’ ownership
    4. once ownership is ‘referenced’ from a variable, accessing this variable after will NOT throw error
    5. Move: transfers ownership of the value, Reference: creates a reference to owner of value (aka borrowing), Copy: sends copy of value
    6. Only 1 mutable reference possible OR many immutable references possible, but NOT both, in a given scope. Lemma: But possible in the same scope if immutable reference is not being accessed after definition of mutable reference and vice versa. Lemma is true also for 2 mutable references, not just 1 mut and 1 immut reference
    7. `*` - dereference, `&` - reference, `ref` - reference 
15. `let s: Box<str> = "hello, world".into();` `into()` - converts static type to heap here (similar to ‘as’)
16. `&str`: pronounced string slice
17. Can ONLY append literals to a string, not another string
18. `(to_string(), String::from())`: &str to String, `(as_str(), &)`: String to &str
19. `r"Escapes don't work here: \x3F \u{211D}"` prints `Escapes don't work here: \x3F \u{211D}`
20. String slice (i.e. &str) is a view into a heap-allocated string
21. `let s1 = String::from("hi,中国"); let h1 = &s1[3..6];` => `h1` is `中`, as `中` is 3 bytes long
22. `&String -> &str` is implicitly convertible by compiler in rust
23. tuples with size > 12 cannot be printed directly with `println!()`


### Dec 5
24. All fields of a struct have to be initialised at the same time. Enum variants can be initialised separately in any order
25. Accessing
    1. Struct: `struct_instance_name.var_name`
    2. Enum: `EnumName::EnumVariable`
26. Struct fields are `name: value` vs Enum variants are mostly only `name`, but can be `name = value` too
27. `#[derive(Debug)]` trait and `println!(“{:?}”,struct_instance_name OR EnumName::EnumVariable);` to print compound types
28. names being accessed after the loop below will throw error coz ownership of its elements was taken by `name`
    ```rust
        let names = [String::from("liming"),String::from("hanmeimei")];
        for name in names {
            // Do something with name...
        }
    ```
29. Iterate the indexing and value in `a`

    ```rust
    let a = [4, 3, 2, 1];
    for (i, v) in a.iter().enumerate() {
        println!("The {}th element is {}", i + 1, v);
    }
    ```
30. Expression is supposed to return a value, statement is not.
31. ```rust
    fn main() {
        let alphabets = ['a', 'E', 'Z', '0', 'x', '9' , 'Y'];

        // Fill the blank with `matches!` to make the code work
        for ab in alphabets {
            assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'))
        }

        println!("Success!");
    } 
    ```
32. Use `matches!` to compare compound types
    ```rust
        enum MyEnum {
            Foo,
            Bar
        }

        fn main() {
            let mut count = 0;

            let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
            for e in v {
                if matches!(e, MyEnum::Foo) { // note
                    count += 1;
                }
            }

            assert_eq!(count, 2);

            println!("Success!");
        }
    ```
33. `if let` > `match` when comparing compound types (enums etc) that have max 1 value. `match` otherwise.
34. `match` and `if let` can introduce shadowed variables
35. Note the partial destructurings in `match` arms (x, y, x and y respectively)
    ```rust

    struct Point {
        x: i32,
        y: i32,
    }

    fn main() {
        // Fill in the blank to let p match the second arm
        let p = Point { x: 4, y: 10 };

        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            // Second arm
            Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
    }
    ```
36. `if x < split` is called a 'match guard'
    ```rust
    // Fill in the blank to make the code work, `split` MUST be used
    fn main() {
        let num = Some(4);
        let split = 5;
        match num {
            Some(x) if x < split => assert!(x < split),
            Some(x) => assert!(x >= split),
            None => (),
        }

        println!("Success!");
    }
    ```
37. On comparing mutable reference in a `match`, the value in the match arm is dereferenced before comparison
    ```rust
    // FIX the error with least changing
    // DON'T remove any code line
    fn main() {
        let mut v = String::from("hello,");
        let r = &mut v;

        match r {
            value => value.push_str(" world!") 
        }
    }
    ```
38. Methods of structs are called on the instance of the struct. Functions are called on the type.
    ```rust
    struct Circle {
        x: f64,
        y: f64,
        radius: f64
    }

    impl Circle {
        fn area(&self) -> f64 {
            std::f64::consts::PI * (self.radius * self.radius)
        }

        fn diameter() -> f64 {
            2 * 10.0
        }
    }

    let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
    println!("{}", c.area()); // => Method
    println!("{}", Circle::diameter()); // => Function
    ```
39. `self` will take the ownership of current struct instance, however, `&self` will only borrow a reference from the instance.
    ```rust
    // Only fill in the blanks, DON'T remove any line!
    #[derive(Debug)]
    struct TrafficLight {
        color: String,
    }

    impl TrafficLight {
        pub fn show_state(&self)  {
            println!("the current state is {}", self.color);
        }
    }
    fn main() {
        let light = TrafficLight{
            color: "red".to_owned(),
        };
        // Don't take the ownership of `light` here.
        light.show_state();
        // ... Otherwise, there will be an error below
        println!("{:?}", light);
    }
    ```

### Dec 6
40. how generic impl for generic sruct is defined
    ```rust
    // Add generic for Val to make the code work, DON'T modify the code in `main`.
    struct Val<T> {
        val: T,
    }

    impl<T> Val<T> {
        fn value(&self) -> &T {
            &self.val
        }
    }

    fn main() {
        let x = Val{ val: 3.0 };
        let y = Val{ val: "hello".to_string()};
        println!("{}, {}", x.value(), y.value());
    }
    ```
41. method (`mixup()`) of a generic struct (`Point<T, U>`) receiving a generic parameter (`Point<V, W>`) returning a generic struct (`Point<T, W>`)
    ```rust
    struct Point<T, U> {
        x: T,
        y: U,
    }

    impl<T, U> Point<T, U> {
        // Implement mixup to make it work, DON'T modify other code.
        fn mixup<V, W>(self, z: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: z.y
            }
        }
    }

    fn main() {
        let p1 = Point { x: 5, y: 10 };
        let p2 = Point { x: "Hello", y: '中'};

        let p3 = p1.mixup(p2);

        assert_eq!(p3.x, 5);
        assert_eq!(p3.y, '中');

        println!("Success!");
    }
    ```
42. a `trait` is like a class for struct, but not exactly. Notice how `say_hi()` is reimplemented for `Teacher`
    ```rust
    // Fill in the two impl blocks to make the code work.
    // DON'T modify the code in `main`.
    trait Hello {
        fn say_hi(&self) -> String {
            String::from("hi")
        }

        fn say_something(&self) -> String;
    }

    struct Student {}
    impl Hello for Student {
        fn say_something(&self) -> String {
            String::from("I'm a good student")
        }
    }
    struct Teacher {}
    impl Hello for Teacher {
        fn say_hi(&self) -> String {
            String::from("Hi, I'm your new teacher")
        }

        fn say_something(&self) -> String {
            String::from("I'm not a bad teacher")
        }
    }

    fn main() {
        let s = Student {};
        assert_eq!(s.say_hi(), "hi");
        assert_eq!(s.say_something(), "I'm a good student");

        let t = Teacher {};
        assert_eq!(t.say_hi(), "Hi, I'm your new teacher");
        assert_eq!(t.say_something(), "I'm not a bad teacher");

        println!("Success!");
    }
    ```
43. Weird destructuring of struct value (`let &Inches(inches) = self;`), `PartialEq` to allow equality comparison, `PartialOrd` to allow greater / lesser comparison
    ```rust
    // `Centimeters`, a tuple struct that can be compared
    #[derive(PartialEq, PartialOrd)]
    struct Centimeters(f64);

    // `Inches`, a tuple struct that can be printed
    #[derive(Debug)]
    struct Inches(i32);

    impl Inches {
        fn to_centimeters(&self) -> Centimeters {
            let &Inches(inches) = self;

            Centimeters(inches as f64 * 2.54)
        }
    }

    // ADD some attributes to make the code work!
    // DON'T modify other code!
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Seconds(i32);

    fn main() {
        let _one_second = Seconds(1);

        println!("One second looks like: {:?}", _one_second);
        let _this_is_true = (_one_second == _one_second);
        let _this_is_false = (_one_second > _one_second);

        let foot = Inches(12);

        println!("One foot equals {:?}", foot);

        let meter = Centimeters(100.0);

        let cmp =
            if foot.to_centimeters() < meter {
                "smaller"
            } else {
                "bigger"
            };

        println!("One foot is {} than one meter.", cmp);
    }
    ```
44. Operator overloading: overloading `*` operator to use `a.mul(b)` from standard library, for any type that implements `Mul` trait
    ```rust
    use std::ops;

    // Implement fn multiply to make the code work.
    // As mentioned above, `+` needs `T` to implement `std::ops::Add` Trait.
    // So, what about `*`?  You can find the answer here: https://doc.rust-lang.org/core/ops/
    fn multiply<T: std::ops::Mul<Output = T>>(a:T, b:T) -> T {
        a * b // a.mul(b)
    }

    fn main() {
        assert_eq!(6, multiply(2u8, 3u8));
        assert_eq!(5.0, multiply(1.0, 5.0));

        println!("Success!");
    }
    ```
45. Overloading `+` operator to funny-add 2 structs (`Foo + Bar = FooBar`)
    ```rust
    // Fix the errors, DON'T modify the code in `main`.
    use std::ops;

    struct Foo;
    struct Bar;

    #[derive(PartialEq, Debug)]
    struct FooBar;

    #[derive(PartialEq, Debug)]
    struct BarFoo;

    // The `std::ops::Add` trait is used to specify the functionality of `+`.
    // Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
    // The following block implements the operation: Foo + Bar = FooBar
    impl ops::Add<Bar> for Foo {
        type Output = FooBar;

        fn add(self, _rhs: Bar) -> FooBar {
            FooBar
        }
    }

    impl ops::Sub<Bar> for Foo {
        type Output = BarFoo;

        fn sub(self, _rhs: Bar) -> BarFoo {
            BarFoo
        }
    }

    fn main() {
        // DON'T modify the code below.
        // You need to derive some trait for FooBar to make it comparable.
        assert_eq!(Foo + Bar, FooBar);
        assert_eq!(Foo - Bar, BarFoo);

        println!("Success!");
    }
    ```