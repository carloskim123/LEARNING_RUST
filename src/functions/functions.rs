// any function or variable should be written in
// - snake_case -> yes
// - kebab-case -> no

fn main() {
    hello();
    tell_height(182);
    human_id("Joel", 55, 182.2);
    let _x:i32 = {
        let price = 5;
        let qty = 10;
        price * qty
    };

    println!("Result is: {}", _x);
}




fn hello() {
    println!("Hello")
    
}

// you can pass input values

fn tell_height(height: i32) {
    println!("My height is {}", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!(
        "My name is {}, I am {} years old, and my height is {} cm",
        name, age, height
    );
}

// ?? Expressions and Statements

// Expression -> anything that returns a value
// ----------
// examples: 5, true | false, add(3,4)
// if condition {value1} else {value2}
// ({code})

// Statement -> anything that does not return a value