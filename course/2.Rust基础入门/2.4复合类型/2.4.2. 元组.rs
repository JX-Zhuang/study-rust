fn example() {
    fn code1() {
        let tup = (500, 6.4, 1);

        let (x, y, z) = tup;

        println!("code1,The value of y is: {}", y);
    }
    fn code2() {
        let x: (i32, f64, u8) = (500, 6.4, 1);

        let five_hundred = x.0;

        let six_point_four = x.1;

        let one = x.2;
        println!("code2,0:{},1:{},2:{}", five_hundred, six_point_four, one);
    }
    fn code3() {
        fn main() {
            let s1 = String::from("hello");
            let (s2, len) = calculate_length(s1);
            println!("code3,The length of '{}' is {}.", s2, len);
        }
        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len(); // len() 返回字符串的长度
            (s, length)
        }
        main();
    }
    code1();
    code2();
    code3();
}
fn practice() {
    fn code1() {
        let _t0: (u8, i16) = (0, -1);
        // 元组的成员还可以是一个元组
        let _t1: (u8, (i16, u32)) = (0, (-1, 1));
        // 填空让代码工作
        let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));
        println!("code1");
    }
    // 修改合适的地方，让代码工作
    fn code2() {
        let t = ("i", "am", "sunface");
        assert_eq!(t.2, "sunface");
        println!("code2");
    }
    // 修复代码错误
    fn code3() {
        let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
        println!("code3,too long tuple: {:?}", too_long_tuple);
    }
    fn code4() {
        let tup = (1, 6.4, "hello");
        // 填空
        let (x, z, y) = tup;
        assert_eq!(x, 1);
        assert_eq!(y, "hello");
        assert_eq!(z, 6.4);
        println!("code4");
    }
    fn code5() {
        let (x, y, z);
        // 填空
        (y, z, x) = (1, 2, 3);
        assert_eq!(x, 3);
        assert_eq!(y, 1);
        assert_eq!(z, 2);
        println!("code5");
    }
    fn code6() {
        fn main() {
            // 填空，需要稍微计算下
            let (x, y) = sum_multiply((2, 3));

            assert_eq!(x, 5);
            assert_eq!(y, 6);
        }

        fn sum_multiply(nums: (i32, i32)) -> (i32, i32) {
            (nums.0 + nums.1, nums.0 * nums.1)
        }
        main();
        println!("code6");
    }
    code1();
    code2();
    code3();
    code4();
    code5();
    code6();
}
fn main() {
    // example();
    practice();
}
