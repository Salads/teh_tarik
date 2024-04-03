fn main() {
    // divisible by 3? Fizz
    // divisible by 5? Buzz
    // divisible by 3 and 5? Fizz Buzz

    // else, print number itself.

    // from 1 to 100
    for i in 1..100
    {
        if i % 3 == 0 && i % 5 == 0
        {
            println!("Fizz Buzz");
        }
        else if i % 3 == 0
        {
            println!("Fizz");
        }
        else if i % 5 == 0
        {
            println!("Buzz");
        }
        else
        {
            println!("{i}");
        }
    }
}
