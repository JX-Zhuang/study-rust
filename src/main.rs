fn example() {
    fn code1() {
        let x = 5;
        let y = &x;
        assert_eq!(x, 5);
        assert_eq!(*y, 5);
    }
    fn code2() {
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        println!("The length of '{}' is {}.", s1, len);

        fn calculate_length(s: &String) -> usize {
            s.len()
        }
    }
    fn code3() {
        let mut s = String::from("hello");
        change(&mut s);
        println!("code3,{}", s);
        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }
    }
    fn code4() {
        let mut s = String::from("hello");
        {
            let s1 = &mut s;
            s1.push_str(",s1");
            println!("s1:{}", s1);
        }
        let s2 = &mut s;
        s2.push_str(",s2");
        println!("s2:{}", s2);
        println!("s:{}", s);
    }
    fn code5() {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);
        let r3 = &mut s;
        println!("{}", r3);
    }
    fn code6() {
        let reference_to_nothing = dangle();
        fn dangle() -> String {
            let s = String::from("hello");
            s
            // &s
        }
    }
    code1();
    code2();
    code3();
    code4();
    code5();
    code6();
}
fn practice() {
    fn code1() {
        let x = 5;
        // 填写空白处
        let p = &x;

        println!("x 的内存地址是 {:p}", p);
        println!("code1 success");
    }
    fn code2() {
        let x = 5;
        let y = &x;
        // 只能修改以下行
        assert_eq!(5, *y);
        println!("code2 success");
    }
    fn code3() {
        fn main() {
            let mut s = String::from("hello, ");

            borrow_object(&s)
        }

        fn borrow_object(s: &String) {}
        main();
        println!("code3 success");
    }
    fn code4() {
        fn main() {
            let mut s = String::from("hello, ");
            push_str(&mut s);
            println!("s 是 {}", s);
        }

        fn push_str(s: &mut String) {
            s.push_str("world")
        }
        main();
        println!("code4 success");
    }
    fn code5() {
        fn main() {
            let mut s = String::from("hello, ");
            // 填写空白处，让代码工作
            let p = &mut s;
            p.push_str("world");
            println!("p 是 {},address is {:p}", p, &p);
            println!("s 是 {},address is {:p}", s, &s);
        }
        main();
        println!("code5 success");
    }
    fn code6() {
        fn main() {
            let c = '中';
            let r1 = &c;
            // 填写空白处，但是不要修改其它行的代码
            let ref r2 = c;
            assert_eq!(*r1, *r2);
            // 判断两个内存地址的字符串是否相等
            assert_eq!(get_addr(r1), get_addr(r2));
        }
        // 获取传入引用的内存地址的字符串形式
        fn get_addr(r: &char) -> String {
            println!("{:p}", r);
            format!("{:p}", r)
        }
        main();
        println!("code6 success");
    }
    fn code7() {
        // 移除代码某个部分，让它工作
        // 你不能移除整行的代码！
        let mut s = String::from("hello");
        let r1 = &mut s;
        let r2 = &mut s;
        // println!("{}, {}", r1, r2);
        println!("code7 success");
    }
    fn code8() {
        fn main() {
            // 通过修改下面一行代码来修复错误
            let mut s = String::from("hello, ");

            borrow_object(&mut s)
        }

        fn borrow_object(s: &mut String) {}
        main();
        println!("code8 success");
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
    example();
    practice();
}
