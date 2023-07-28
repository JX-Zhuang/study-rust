fn example() {
    fn code1() {
        #[derive(Debug)]
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        let user2 = User {
            active: user1.active,
            username: user1.username,
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        };
        println!("{}", user1.active);
        // 下面这行会报错
        // println!("{:#?}", user1);
        println!("code1");
    }
    fn code2() {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        fn main() {
            let scale = 2;
            let rect1 = Rectangle {
                width: dbg!(30 * scale),
                height: 50,
            };

            dbg!(&rect1);
        }
        main();
        println!("code2");
    }
    code1();
    code2();
}
fn practice() {
    fn code1() {
        // fix the error
        struct Person {
            name: String,
            age: u8,
            hobby: String,
        }
        fn main() {
            let age = 30;
            let p = Person {
                name: String::from("sunface"),
                age,
                hobby: String::from("basketball"),
            };
        }
        main();
        println!("code1");
    }
    fn code2() {
        struct Unit;
        trait SomeTrait {
            // ...定义一些行为
        }

        // 我们并不关心结构体中有什么数据( 字段 )，但我们关心它的行为。
        // 因此这里我们使用没有任何字段的单元结构体，然后为它实现一些行为
        impl SomeTrait for Unit {}
        fn main() {
            let u = Unit;
            do_something_with_unit(u);
        }

        // 填空，让代码工作
        fn do_something_with_unit(u: Unit) {}
        main();
        println!("code2");
    }
    fn code3() {
        // 填空并修复错误
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);
        fn main() {
            let v = Color(0, 127, 255);
            check_color(v);
        }

        fn check_color(p: Color) {
            let Color(x, _, _) = p;
            assert_eq!(x, 0);
            assert_eq!(p.1, 127);
            assert_eq!(p.2, 255);
            println!("{}", p.0);
        }
        main();
        println!("code3");
    }
    fn code4() {
        struct Person {
            name: String,
            age: u8,
        }
        fn main() {
            let age = 18;
            let mut p = Person {
                name: String::from("sunface"),
                age,
            };

            // how can you believe sunface is only 18?
            p.age = 30;

            // 填空
            p.name = String::from("sunfei");
        }
        main();
        println!("code4");
    }
    fn code5() {
        // 填空
        #[derive(Debug)]
        struct Person {
            name: String,
            age: u8,
        }
        fn main() {
            let person = build_person(String::from("Jesse"), 18);
            println!("{:#?}", person);
        }

        fn build_person(name: String, age: u8) -> Person {
            Person {
                age,
                name,
            }
        }
        main();
        println!("code5");
    }
    fn code6() {
        #[derive(Debug)]
        // 填空，让代码工作
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }
        fn main() {
            let u1 = User {
                email: String::from("someone@example.com"),
                username: String::from("sunface"),
                active: true,
                sign_in_count: 1,
            };
            let u2 = set_email(u1);
            println!("{:#?}", u2);
        }

        fn set_email(u: User) -> User {
            User {
                email: String::from("contact@im.dev"),
                active: false,
                ..u
            }
        }
        main();
        println!("code6");
    }
    fn code7() {
        // 填空，让代码工作
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        fn main() {
            let scale = 2;
            let rect1 = Rectangle {
                width: dbg!(30 * scale), // 打印 debug 信息到标准错误输出 stderr,并将 `30 * scale` 的值赋给 `width`
                height: 50,
            };
            dbg!(&rect1); // 打印 debug 信息到标准错误输出 stderr
            println!("{:#?}", rect1); // 打印 debug 信息到标准输出 stdout
        }
        main();
        println!("code7");
    }
    fn code8() {
        // 修复错误
        #[derive(Debug)]
        struct File {
            name: String,
            data: String,
        }
        fn main() {
            let f = File {
                name: String::from("readme.md"),
                data: "Rust By Practice".to_string(),
            };

            let _name = f.name;

            // 只能修改这一行
            println!("{}", f.data);
        }
        main();
        println!("code8");
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
    // example();
    practice();
}
