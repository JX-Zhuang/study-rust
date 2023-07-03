fn practice1() {
    fn code1() {
        let x: i32 = 1;
        let _y: i32 = 2;
        println!("x is equal to {}", x);
    }
    fn code2() {
        let mut __ = 1;
        __ += 2;
        println!("x = {}", __);
    }
    fn code3() {
        let x: i32 = 10;
        let y: i32 = 5;
        println!("x 的值是 {}, y 的值是 {}", x, y);
        println!("x 的值是 {}, y 的值是 {}", x, y);
    }
    fn code4() {
        let x: i32 = 5;
        {
            let x = 12;
            assert_eq!(x, 12);
        }

        assert_eq!(x, 5);

        let x = 42;
        println!("{}", x); // 输出 "42".
    }
    fn code5() {
        let mut x: i32 = 1;
        x = 7;
        // 遮蔽且再次绑定
        let mut x = x;
        x += 3;
        let y = 4;
        // 遮蔽
        let y = "I can also be bound to text!";
    }
    fn code6() {
        let _x = 1;
    }
    fn code7() {
        let (mut x, y) = (1, 2);
        x += 2;
        assert_eq!(x, 3);
        assert_eq!(y, 2);
    }
    fn code8() {
        let (x, y);
        (x, ..) = (3, 4);
        [.., y] = [1, 2];
        // 填空，让代码工作
        assert_eq!([x, y], [3, 2]);
    }
    code1();
    code2();
    code3();
    code4();
    code5();
    code6();
    code7();
    code8();
}
fn main() {
    practice1();
}
