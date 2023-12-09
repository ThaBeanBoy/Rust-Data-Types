use std::io;
use rand::Rng;

fn main() {
    // # Muteability
    {
        // making x immuteable
        // let x = 5;

        // making x muteable
        let mut x = 5;
        println!("the value of x is {x}");
        x= 6;
        println!("the value of x is {x}");
    }

    // # CONSTANTS
    {
        println!("\n\nCONSTANTS\n");
        /*
        In order to make a constant, you need to
        - `const` key word
        - variable name (in all caps, I think this is a style choice or something)
        - explicitly declare the data type
        - assign a value
        */ 
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
        println!("three hours in seconds is {THREE_HOURS_IN_SECONDS}");
    }
    
    // # SHADOWING
    {
        println!("\n\nSHADOWING\n");
        /* 
        By using shadowing, you can re-use a variabe name and even change that variables'
        type. for instance

        ```rs
        let mut spaces = "   ";
        spaces = spaces.len();
        ```

        will result in a compile error, becuase when you use the `mut` key word and want 
        to re-assign a value, you have to re-assign with the same type. but using 
        shadowing, you can essentially change the type.

        ```rs
        let spaces = "   ";
        let spaces = spaces.len();
        ```

        The first `space` variable is of type string, but using shadowing, we redeclare 
        `spaces` as type `usize``
        */

        let x = 5;

        let x = x + 1;

        {
            // Variable x is going to shadow the previous x but only withing this scope
            let x = x * 2;
            println!("The value of x in the inner scope is: {x}");
        }

        /* 
        The value x will not be shadowed by the x declared in line 22, 
        because that x (in line 22) only shadows within that scope.
        Once the program is out od that scope, x will be 6 again 
        */
        println!("The value of x is: {x}");
    }

    // # DATA TYPE CONVERSION
    {
        println!("\n\nDATA TYPE CONVERSION\n");

        let guess = "42";
        println!("string value of variable `guess` = {guess}");

        /* 
        To convert a value, you need to explicitly declare the data type,
        I think it's because the compiler needs to know the data type 
        it's converting the vale to. 

        Oh bonus, I used shadowing 🙂
         */
        let guess: u32 = "42".parse().expect("Not a number");
        println!("adding 3 to numerical value of `guess` is {}", guess + 3);
    }

    // # DATA TYPES
    {
        /* 
        In rust, there are 2 categories of datatypes:
        1. Scalar types
        2. Compound types

        Scalar is a single value. There are a few scalar data types:
        1. Integers
        2. Floating points
        3. Boolean
        4. Characters
        
        Integers

        There are 2 types of integers, signed & unsigned integers.
        signed integers can be negative or positive whereas unsigned
        are only positive.

        Another thing about integers, is that they come in different sizes. 
        they can be 8, 16, 32, 64 & 128 bits. There's also something called 
        `arch`

        if an integer is signed, you can calculate it's range using the formula
        -[2^(n-1)] to [2^(n-1) - 1]. signed integers use 2s complement.

        */
        let tup: (u32, f64, bool) = (34, 65.78, true);

        /* 
        Arrays have a fixed length & the elements of the array can only be of
        the same type. alot like c++ 🥲, the horrors of trying to insert and
        remove elements in an array in c++ was a headache.

        Arrays are stored in the stack instead of the heap.
        */
        let arr = [67, 1001, 2002, 657, 9999];

        // initialising an array that has the same value
        // [3, 3, 3, 3, 3]
        // let empty_arr = [3; 5];

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("index is not a number");

        println!("The value at index {index} is {}", arr[index])
    }
}
