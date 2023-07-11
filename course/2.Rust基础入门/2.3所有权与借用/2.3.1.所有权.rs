fn code1() {
    fn main() {
        let s = String::from("hello");
        takes_ownership(s);
        let s1 = String::from("hello");
        takes_ownership(s1);
        // println!("{}", s);
        let x = 5;
        makes_copy(x);
    }
    fn takes_ownership(some_thing: String) {
        println!("{}", some_thing);
    }
    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }
    main();
}
// fn code2() {
//     fn main() {
//         let s1 = gives_ownership();
//         let s2 = String::from("hello");
//         let s3 = takes_and_gives_back(s2);
//     }
//     fn gives_ownership() -> String {
//         let some_thing = String::from("hello");
//         some_thing
//     }
//     fn takes_and_gives_back(a_string: String) -> String {
//         a_string
//     }
//     main();
// }
fn practice() {
    fn code1() {
        let x = String::from("hello, world");
        let y = x.clone();
        println!("{},{}", x, y);
        println!("code1 success");
    }
    fn code2() {
        // 不要修改 main 中的代码
        fn main() {
            let s1 = String::from("hello, world");
            let s2 = take_ownership(s1);

            println!("{}", s2);
        }

        // 只能修改下面的代码!
        fn take_ownership(s: String) -> String {
            println!("{}", s);
            s
        }
        main();
        println!("code2 success");
    }
    fn code3() {
        fn main() {
            let s = give_ownership();
            println!("{}", s);
        }

        // 只能修改下面的代码!
        fn give_ownership() -> String {
            let s = String::from("hello, world");
            // convert String to Vec
            // 将 String 转换成 Vec 类型
            let _s = s.clone().into_bytes();
            s
        }
        main();
        println!("code3 success");
    }
    fn code4() {
        // 修复错误，不要删除任何代码行
        fn main() {
            let s = String::from("hello, world");

            print_str(s.clone());

            println!("{}", s);
        }

        fn print_str(s: String) {
            println!("{}", s)
        }
        main();
        println!("code4 success");
    }
    fn code5() {
        // 不要使用 clone，使用 copy 的方式替代
        fn main() {
            let x = (1, 2, (), "hello");
            let y = x;
            println!("{:?}, {:?}", x, y);
        }
        main();
        println!("code5 success");
    }
    fn code6() {
        fn main() {
            let s = String::from("hello, ");

            // 只修改下面这行代码 !
            let mut s1 = s;

            s1.push_str("world");
            println!("{}", s1);
        }
        main();
        println!("code6 success");
    }
    fn code7() {
        fn main() {
            let x = Box::new(5);

            let mut y = Box::new(3); // 完成该行代码，不要修改其它行！
            println!("x:{}", x);
            *y = 4;
            println!("x:{},y:{}", x, y);
            assert_eq!(*x, 5);
        }
        main();
        println!("code7 success");
    }
    fn code8() {
        let t = (String::from("hello"), String::from("world"));
        let _s = t.0;
        // 仅修改下面这行代码，且不要使用 `_s`
        println!("{:?}", t.1);
        println!("code8 success");
    }
    fn code9() {
        let t = (String::from("hello"), String::from("world"));

        // 填空，不要修改其它代码
        {
            let (ref s1, ref s2) = t;
            println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
        }
        let (s1, s2) = t.clone();
        println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
        println!("code9 success");
    }

    code1();
    code2();
    code3();
    code4();
    code5();
    code6();
    code7();
    code8();
    code9();
}
fn main1() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };
    println!("The person struct is {:?}", person);
    let Person { name, ref age } = person;
    println!("The person's age is {}", age);
    println!("The person's name is {}", name);
    println!("The person's age from person struct is {}", person.age);
}
fn main() {
    code1();
    // code2();
    practice();
    main1();
}
