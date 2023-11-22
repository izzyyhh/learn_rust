fn main() {
    let sum = add(2, 3);
    let tuple = (1, ("inner", "tuple"));

    println!("Hello, world! {}", sum);
    println!("{:?}", tuple);

    let ifresult = if sum > 10 {
        "is greater than 10"
    } else {
        "is less than 10"
    };

    match sum {
        11 => println!("is eleven"),
        _ => println!("rest does not matter"),
    }

    println!("{:?}", ifresult);

    fizz_buzz(50)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn fizz_buzz(of: i32) {
    let mut i = 1;
    let max = of;

    while i <= max {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }

        i += 1;
    }
}
