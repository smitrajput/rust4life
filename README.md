# rust-ballistics
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
42. a `trait` is like a class for struct, but not exactly. Mainly used when you want some structs to share some functionalities. Notice how `say_hi()` is reimplemented for `Teacher`
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

### Dec 7

46. All functions of a trait must be implemented by the struct that uses it.

47. `trait` as a fn parameter. Used to restrict parameter types to those that implement the trait. Note both implementations of `summary()`
    ```rust
    // Implement `fn summary` to make the code work.
    // Fix the errors without removing any code line
    trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug)]
    struct Post {
        title: String,
        author: String,
        content: String,
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("The author of post {} is {}", self.title, self.author)
        }
    }

    #[derive(Debug)]
    struct Weibo {
        username: String,
        content: String,
    }

    impl Summary for Weibo {
        fn summarize(&self) -> String {
            format!("{} published a weibo {}", self.username, self.content)
        }
    }

    fn main() {
        let post = Post {
            title: "Popular Rust".to_string(),
            author: "Sunface".to_string(),
            content: "Rust is awesome!".to_string(),
        };
        let weibo = Weibo {
            username: "sunface".to_string(),
            content: "Weibo seems to be worse than Tweet".to_string(),
        };

        summary(&post);
        summary(&weibo);

        println!("{:?}", post);
        println!("{:?}", weibo);
    }

    // Implement `fn summary` below.
    fn summary(a: &impl Summary) {
        let output: String = a.summarize();
        
        println!("{}", output);
    }

    //// alternate implementation ////
    fn summary<T: Summary>(a: &T) {
        let output: String = a.summarize();
        
        println!("{}", output);
    }

    ```
48. _General compiler rule: sizes of return types must be known at compile time._

49. `Box` is smart pointer that allocates memory in heap. Used for types whose size is not known at compile time. It allocates and owns the memory in heap. Also deallocates memory when out of scope. `&` on the other hand only points to existing memory. `Box` can be passed across scopes, but `&` has limited lifetime. `Box` can be cloned, but `&` cannot be. `Box` can be used in pattern matching.

50. Dynamic Dispatch
    <img width="1528" alt="Screenshot 2024-12-07 at 9 41 55 AM" src="https://github.com/user-attachments/assets/c5702b60-5c15-4db6-a244-874c83261e80">


51. For functions wanting to return different types that implement a given trait, we cannot use `impl Trait` directly as size of return type is unknown at compile time, so need to dynamic dispatch the return value, using `Box<dyn Trait>`
    ```rust
    struct Sheep {}
    struct Cow {}

    trait Animal {
        fn noise(&self) -> String;
    }

    impl Animal for Sheep {
        fn noise(&self) -> String {
            "baaaaah!".to_string()
        }
    }

    impl Animal for Cow {
        fn noise(&self) -> String {
            "moooooo!".to_string()
        }
    }

    // Returns some struct that implements Animal, but we don't know which one at compile time.
    // FIX the errors here, you can make a fake random, or you can use trait object.
    fn random_animal(random_number: f64) -> Box<dyn Animal> {
        if random_number < 0.5 {
            Box::new(Sheep {})
        } else {
            Box::new(Cow {})
        }
    }

    fn main() {
        let random_number = 0.234;
        let animal = random_animal(random_number);
        println!("You've randomly chosen an animal, and it says {}", animal.noise());
    }
    ```

52. `impl Trait` is a specific trait bound. More generally, when working with generics, the type parameters often use traits as bounds to stipulate what functionality a type implements, in various other ways. Below, `T` is bound to any type that implements `std::ops::Add` trait.
    ```rust
    fn main() {
        assert_eq!(sum(1, 2), 3);
    }

    // Implement `fn sum` with trait bound in two ways.
    fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
        x + y
    }
    ```

53. More trait bounds. Observe the alternate implementation of creating a struct. Also note `Self == Pair<T>`
    ```rust
    struct Pair<T> {
        x: T,
        y: T,
    }

    impl<T> Pair<T> {
        // Self == Pair<T>
        fn new(x: T, y: T) -> Self {
            Self {
                x,
                y,
            }
        }
    }

    impl<T: std::fmt::Debug + PartialOrd + PartialEq> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {:?}", self.x);
            } else {
                println!("The largest member is y = {:?}", self.y);
            }
        }
    }

    #[derive(Debug, PartialOrd, PartialEq)]
    struct Unit(i32);

    fn main() {
        let pair = Pair {
            x: Unit(1),
            y: Unit(3)
        };
        
        // alternate version of creating a struct
        let pair = Pair::new(Unit(1), Unit(3));

        pair.cmp_display();
    }
    ```

### Dec 9

54. `hatch_a_bird()` returns a `Box<dyn Bird>`, which is a trait object, meaning it can return any type that implements the `Bird` trait, without knowing the exact type at compile time (also note that return types for all arms in `match` must be same)
    ```rust
    trait Bird {
        fn quack(&self) -> String;
    }

    struct Duck;
    impl Duck {
        fn swim(&self) {
            println!("Look, the duck is swimming")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }

    impl Bird for Duck {
        fn quack(&self) -> String{
            "duck duck".to_string()
        }
    }

    impl Bird for Swan {
        fn quack(&self) -> String{
            "swan swan".to_string()
        }
    }

    fn main() {
        // FILL in the blank.
        let duck: Duck = Duck;
        duck.swim();

        let bird = hatch_a_bird(2);
        // This bird has forgotten how to swim, so below line will cause an error.
        // bird.swim();
        // But it can quak.
        assert_eq!(bird.quack(), "duck duck");

        let bird = hatch_a_bird(1);
        // This bird has forgotten how to fly, so below line will cause an error.
        // bird.fly();
        // But it can quak too.
        assert_eq!(bird.quack(), "swan swan");

        println!("Success!");
    }   

    // IMPLEMENT this function.
    fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
        match species {
            1 => Box::new(Swan),
            2 => Box::new(Duck),
            _ => panic!()
        }
    }

    ```

55. Array of trait objects. Note how every element of `birds` is a pointer with size of `usize`, coz we can't specify `Duck` or `Swan` whose size is unknown at compile time. Also `&dyn Bird` == `Box<dyn Bird>`
    ```rust
    trait Bird {
        fn quack(&self);
    }

    struct Duck;
    impl Duck {
        fn fly(&self) {
            println!("Look, the duck is flying")
        }
    }
    struct Swan;
    impl Swan {
        fn fly(&self) {
            println!("Look, the duck.. oh sorry, the swan is flying")
        }
    }

    impl Bird for Duck {
        fn quack(&self) {
            println!("{}", "duck duck");
        }
    }

    impl Bird for Swan {
        fn quack(&self) {
            println!("{}", "swan swan");
        }
    }

    fn main() {
        // FILL in the blank to make the code work.
        let birds: [&dyn Bird; 2] = [&Swan, &Duck]; // usize

        for bird in birds {
            bird.quack();
            // When duck and swan turn into Birds, they all forgot how to fly, only remember how to quack.
            // So, the code below will cause an error.
            // bird.fly();
        }
    }
    ```

56. Creating a trait for a standard lib type
    ```rust
    // FILL in the blanks.
    trait Draw {
        fn draw(&self) -> String;
    }

    impl Draw for u8 {
        fn draw(&self) -> String {
            format!("u8: {}", self)
        }
    }

    impl Draw for f64 {
        fn draw(&self) -> String {
            format!("f64: {}", self)
        }
    }

    fn main() {
        let x = 1.1f64;
        let y = 8u8;

        // Draw x.
        draw_with_box(Box::new(x));

        // Draw y.
        draw_with_ref(&y);

        println!("Success!");
    }

    fn draw_with_box(x: Box<dyn Draw>) {
        x.draw();
    }

    fn draw_with_ref(x: &dyn Draw) {
        x.draw();
    }
    ```

57. Note: `Box<dyn Foo>` doesn't work in `dynamic_dispatch(a: &dyn Foo)`
    ```rust
    trait Foo {
        fn method(&self) -> String;
    }

    impl Foo for u8 {
        fn method(&self) -> String { format!("u8: {}", *self) }
    }

    impl Foo for String {
        fn method(&self) -> String { format!("string: {}", *self) }
    }

    // IMPLEMENT below with generics.
    fn static_dispatch<T: Foo>(a: T) {
        a.method();
    }

    // Implement below with trait objects.
    fn dynamic_dispatch(a: &dyn Foo) {
        a.method();
    }

    fn main() {
        let x = 5u8;
        let y = "Hello".to_string();

        static_dispatch(x);
        dynamic_dispatch(&y);

        println!("Success!");
    }
    ```

58. A trait is object-safe when (1) its methods DO NOT return `Self` (2) there are NO generic parameters. Note return type of `f()` is `Box<dyn MyTrait>`, so that its size is known at compile time.
    ```rust
    // Use at least two approaches to make it work.
    // DON'T add/remove any code line.
    trait MyTrait {
        fn f(&self) -> Box<dyn MyTrait>;
    }

    impl MyTrait for u32 {
        fn f(&self) -> Box<dyn MyTrait> { Box::new(42) }
    }

    impl MyTrait for String {
        fn f(&self) -> Box<dyn MyTrait> { Box::new(self.clone()) }
    }

    fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait>  {
        x.f()
    }

    fn main() {
        my_function(Box::new(13_u32));
        my_function(Box::new(String::from("abc")));

        println!("Success!");
    }
    ```
