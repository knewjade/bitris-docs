#[cfg(test)]
mod tests {
    // ### START IMPORTS ###
    use std::println;
    // ### END IMPORTS ###

    #[test]
    fn foo() {
        println!("hello world!");
    }

    #[test]
    fn bar() {
        {
            println!("hello");
        }
        {
            println!("world");
        }
    }
}
