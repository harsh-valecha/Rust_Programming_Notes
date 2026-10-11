# Learning notes for Rust programming

1. file name starts with .rs , to run a file main.rs ->

```
rustc main.rs
./main
```

2. creating a project using cargo project manager

```
cargo new project_name
```

3. To build a project

```
cargo build
```

where inside the project_name/target/debug/ we can find the executable and execute the same using the command

```
./project_name
```

3. To run directly your project

```
cargo run project_name
```

4. To check for any issues in yout project use -

```
cargo check
```

**Need a semicolon at end of line to end statement**

5. Formatting the code -

```
cd src/
rustfmt filename.rs
```

**main.rs -> main function is entry point of program**

second program -

```
fn main() {
    let x = 4; // implicit type decision
    // x = "hello" will not work since i assigned this to a integer

    println!("x is :{}",x);

}
```

By default all the variables in rust are immutable , to
create a mutable variable use **mut** keyword

```
let mut x = 4;
x = 32; // this allows reassignment
```

Redeclaring existing variable is allowed -

```
fn main() {
    let x = 4;
    println!("x is :{}",x);
    let x = 5;
    println!("x is :{}",x);
}

Output -
 Compiling tutorial1 v0.1.0 (/Users/harshvalecha/harsh_work/rust_practice/tutorial1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/tutorial1`
x is :4
x is :5
```

incrementing the variable -

```
fn main() {
    let x = 4;
    println!("x is :{}",x);
    let x = x+1;
    println!("x is :{}",x);
}
```

Scope examples -

```
fn main() {
    let x = 4;
    println!("x is :{}",x);
    {
        // scope
        let x = x - 2;
        println!("x is :{}",x);
    }
    let x = x+1;
    println!("x is :{}",x);
}

output -
x is :4
x is :2
x is :5
```

This also works -

```
fn main() {
    let x = 4;
    println!("x is :{}",x);
    let x = "hello";
    println!("x is :{}",x);
}

x is :4
x is :hello
```
