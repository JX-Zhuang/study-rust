fn example() {
    fn code1() {
        enum IpAddr {
            Ipv4,
            Ipv6,
        }
        let ip1 = IpAddr::Ipv6;
        let ip_str = match ip1 {
            IpAddr::Ipv4 => "127.0.0.1",
            _ => "::1",
        };
        println!("{}", ip_str);
        println!("code1");
    }
    fn code2() {
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
            // --snip--
        }

        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState), // 25美分硬币
        }
        fn main(coin: Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }
        let res = main(Coin::Quarter(UsState::Alabama));
        println!("{}", res);
        println!("code2");
    }
    fn code3() {
        enum Action {
            Say(String),
            MoveTo(i32, i32),
            ChangeColorRGB(u16, u16, u16),
        }
        fn main() {
            let actions = [
                Action::Say("Hello Rust".to_string()),
                Action::MoveTo(1, 2),
                Action::ChangeColorRGB(255, 255, 0),
            ];
            for action in actions {
                match action {
                    Action::Say(s) => {
                        println!("{}", s);
                    }
                    Action::MoveTo(x, y) => {
                        println!("point from (0, 0) move to ({}, {})", x, y);
                    }
                    Action::ChangeColorRGB(r, g, _) => {
                        println!(
                            "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                            r,
                            g
                        );
                    }
                }
            }
        }
        main();
        println!("code3");
    }
    fn code4() {
        let foo = 'f';
        assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));
        let bar = Some(4);
        assert!(matches!(bar, Some(x) if x > 2));
        println!("code4");
    }
    fn code5() {
        let age = Some(30);
        println!("在匹配前，age是{:?}", age);
        if let Some(age) = age {
            println!("匹配出来的age是{:?}", age);
        }

        println!("在匹配后，age是{:?}", age);
        println!("code5");
    }
    code1();
    code2();
    code3();
    code4();
    code5();
}
fn practice() {
    fn code1() {
        // 填空
        enum Direction {
            East,
            West,
            North,
            South,
        }

        fn main() {
            let dire = Direction::South;
            match dire {
                Direction::East => println!("East"),
                Direction::South | Direction::North => {
                    // 在这里匹配 South 或 North
                    println!("South or North");
                }
                _ => println!("West"),
            }
        }
        main();
        println!("code1");
    }
    fn code2() {
        let boolean = true;

        // 使用 match 表达式填空，并满足以下条件
        //
        // boolean = true => binary = 1
        // boolean = false => binary = 0
        let binary = match boolean {
            true => 1,
            false => 0,
        };

        assert_eq!(binary, 1);
        println!("code2");
    }
    fn code3() {
        // 填空
        enum Message {
            Quit,
            Move {
                x: i32,
                y: i32,
            },
            Write(String),
            ChangeColor(i32, i32, i32),
        }

        fn main() {
            let msgs = [
                Message::Quit,
                Message::Move { x: 1, y: 3 },
                Message::ChangeColor(255, 255, 0),
            ];

            for msg in msgs {
                show_message(msg);
            }
        }

        fn show_message(msg: Message) {
            match msg {
                Message::Move { x: a, y: b } => {
                    // 这里匹配 Message::Move
                    assert_eq!(a, 1);
                    assert_eq!(b, 3);
                }
                Message::ChangeColor(_, g, b) => {
                    assert_eq!(g, 255);
                    assert_eq!(b, 0);
                }
                __ => println!("no data in these variants"),
            }
        }
        main();
        println!("code3");
    }
    fn code4() {
        let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];
        // 使用 `matches` 填空
        for ab in alphabets {
            assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'));
        }
        println!("code4");
    }
    fn code5() {
        enum MyEnum {
            Foo,
            Bar,
        }
        fn main() {
            let mut count = 0;
            let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
            for e in v {
                if matches!(e, MyEnum::Foo) {
                    // 修复错误，只能修改本行代码
                    count += 1;
                }
            }
            assert_eq!(count, 2);
        }
        println!("code5");
    }
    fn code6() {
        let o = Some(7);
        // 移除整个 `match` 语句块，使用 `if let` 替代
        if let Some(i) = o {
            println!("This is a really long string and `{:?}`", i);
        }
        println!("code6");
    }
    fn code7() {
        // 填空
        enum Foo {
            Bar(u8),
        }
        let a = Foo::Bar(1);
        if let Foo::Bar(i) = a {
            println!("foobar 持有的值是: {}", i);
        }
        println!("code7");
    }
    fn code8() {
        enum Foo {
            Bar,
            Baz,
            Qux(u32),
        }
        fn main() {
            let a = Foo::Qux(10);
            match a {
                Foo::Bar => println!("match foo::bar"),
                Foo::Baz => println!("match foo::baz"),
                _ => println!("match others"),
            }
        }
        main();
        println!("code8");
    }
    // 就地修复错误
    fn code9() {
        let age = Some(30);
        if let Some(age) = age {
            println!("age:{}", age);
            // 创建一个新的变量，该变量与之前的 `age` 变量同名
            assert_eq!(age, 30);
        } // 新的 `age` 变量在这里超出作用域
        match age {
            // `match` 也能实现变量遮蔽
            Some(age) => println!("age 是一个新的变量，它的值是 {}", age),
            _ => (),
        }
        println!("code9");
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
fn main() {
    // example();
    practice();
}
