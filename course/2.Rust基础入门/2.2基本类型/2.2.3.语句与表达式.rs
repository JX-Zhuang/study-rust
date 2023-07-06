// 表达式不能用分号结尾
fn code1() {
    let y = {
        let x = 3;
        x + 1
    };
    println!("code1, y is {}", y);
}
fn code2() {
    fn ret_unit_type() {
        let x = 1;
        //if 语句块也是一个表达式，因此可以用于赋值，也可以直接返回
        let _y = if x % 2 == 1 { "odd" } else { "even" };
    }
    assert_eq!(ret_unit_type(), ())
}
fn practice() {
    fn code0() {
        let x = 5u32;
        let y = {
            let x_squared = x * x;
            let x_cube = x_squared * x;
            x_cube + x_squared + x
        };
        let z = { 2 * x };
        println!("x is {:?}", x);
        println!("y is {:?}", y);
        println!("z is {:?}", z);
    }
    fn code1() {
        let v = {
            let mut x = 1;
            x += 2
        };
        assert_eq!(v, ());
        println!("code1,success");
    }
    fn code2() {
        let v = {
            let x = 3;
            x
        };
        assert!(v == 3);
        println!("code2,success");
    }
    fn code3() {
        let s = sum(1, 2);
        // assert!(s == 3);
        assert_eq!(s, 3);
        println!("code3,success");
    }
    fn sum(x: i32, y: i32) -> i32 {
        x + y
    }
    code0();
    code1();
    code2();
    code3();
}
fn main() {
    code1();
    code2();
    practice();
}
