fn main() {
    // variables are declared using let keyword
    // variables in rust are always written in snake_case
    // rust can correctly infer the variable type 99% of the time

    let x = 9; //if not mentioned, rust infers the type of x
    println!("{}", x);

    // shadowing allows you to asign same variable name multiple times with different types
    
    let x: f64 = 3.14159; //explicitly mentioning the type
    println!("{}", x);

    let x; //declaring the variable and initializing it later, rarely done
    x = 0;
    println!("{}", x);
}