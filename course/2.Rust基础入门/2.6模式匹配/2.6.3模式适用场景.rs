fn example() {
    fn code1() {
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
        println!("code1");
    }
    fn code2() {
        fn print_coordinates(&(x, y): &(i32, i32)) {
            println!("Current location: ({}, {})", x, y);
        }
        fn main() {
            let point = (3, 5);
            print_coordinates(&point);
        }
        main();
        println!("code2");
    }
    code1();
    code2();
}
fn main() {
    example();
}
