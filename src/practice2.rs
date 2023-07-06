use num::complex::Complex;
use std::mem::size_of_val;
fn code1() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
}
fn code2() {
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小", std::mem::size_of_val(&x));
    let t = true;
    let f: bool = false;
    if f {
        println!("这是段毫无意义的代码");
    }
}
fn practice2() {
    fn code1() {
        let c1 = 'a';
        assert_eq!(size_of_val(&c1), 4);
        let c2 = '中';
        assert_eq!(size_of_val(&c2), 4);
        println!("code1,Success");
    }
    fn code2() {
        let c1 = '中';
        print_char(c1);
    }
    fn print_char(c: char) {
        println!("code2,{}", c);
    }
    fn code3() {
        let _f: bool = false;
        let t = true;
        if t {
            println!("code3,Success!");
        }
    }
    fn code4() {
        let f = true;
        let t = true || false;
        assert_eq!(t, f);
        println!("code4,Success!");
    }
    fn code5() {
        let _v: () = ();
        let v = (2, 3);
        assert_eq!(_v, implicitly_ret_unit());
        println!("code5,Success!")
    }
    fn implicitly_ret_unit() {
        println!("I will return a ()")
    }
    fn code6() {
        let unit: () = ();
        assert!(size_of_val(&unit) == 0);
        println!("code6,Success!");
    }
    code1();
    code2();
    code3();
    code4();
    code5();
    code6();
}
fn main() {
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);
    let c = 0.1;
    let d = 0.2;
    let e = c + d;
    println!("{}", e);
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    assert!(abc.0 + abc.1 == abc.2);
    // let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    // assert!(xyz.0 + xyz.1 == xyz.2);
    let x = (-1_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为");
    }
    for i in 1..=5 {
        println!("{}", i);
    }
    for i in 'a'..='z' {
        println!("{}", i);
    }
    code1();
    code2();
    practice2();
}
