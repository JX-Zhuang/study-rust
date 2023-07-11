fn plus_five(x: i32) -> i32 {
    x + 5
}
fn practice() {
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
    fn code1() {
        let (x, y) = (1, 2);
        let s = sum(x, y);
        assert_eq!(s, 3);
        println!("code1,success");
    }
    fn print() -> () {
        println!("hello,world");
    }
    fn code2() {
        print();
        println!("code2,success");
    }
    fn never_return() -> ! {
        panic!("I return nothing!")
    }
    fn code3_1() {
        never_return();
    }
    fn code3_2() {
        fn main() {
            never_return();
        }

        use std::thread;
        use std::time;

        fn never_return() -> ! {
            // implement this function, don't modify fn signatures
            loop {
                println!("I return nothing");
                // sleeping for 1 second to avoid exhausting the cpu resource
                thread::sleep(time::Duration::from_secs(1));
            }
        }
        main();
    }
    fn code5() {
        // FILL in the blank
        let b = false;

        let _v = match b {
            true => 1,
            // Diverging functions can also be used in match expression
            false => {
                println!("Success!");
                panic!("we have no value for `false`, but we can panic")
            }
        };

        println!("Exercise Failed if printing out this line!");
    }
    code1();
    code2();
    // code3_1();
    // code3_2();
    code5();
}
fn main() {
    let x = plus_five(5);
    println!("{}", x);
    practice();
}
