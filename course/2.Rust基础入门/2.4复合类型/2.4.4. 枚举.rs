fn example() {
    fn code1() {
        #[derive(Debug)]
        enum PokerSuit {
            Clubs,
            Spades,
            Diamonds,
            Hearts(u8),
        }
        let heart = PokerSuit::Hearts(20);
        let diamond = PokerSuit::Diamonds;
        print_suit(heart);
        print_suit(diamond);
        fn print_suit(card: PokerSuit) {
            println!("{:?}", card);
        }
        println!("code1");
    }
    code1();
}
fn practice() {
    fn code1() {
        // 修复错误
        enum Number {
            Zero,
            One,
            Two,
        }

        enum Number1 {
            Zero = 0,
            One,
            Two,
        }

        // C语言风格的枚举定义
        enum Number2 {
            Zero = 0,
            One = 1,
            Two = 2,
        }

        fn main() {
            // 通过 `as` 可以将枚举值强转为整数类型
            assert_eq!(Number::One as u8, Number1::One as u8);
            assert_eq!(Number1::One as u8, Number2::One as u8);
        }
        main();
        println!("code1");
    }
    fn code2() {
        #[derive(Debug)]
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
            let msg1 = Message::Move {
                x: 1,
                y: 2,
            }; // 使用x = 1, y = 2 来初始化
            let msg2 = Message::Write(String::from("hello,world")); // 使用 "hello, world!" 来初始化
            println!("msg1,{:?}", msg1);
            println!("msg2,{:?}", msg2);
        }
        main();
        println!("code2");
    }
    fn code3() {
        // 仅填空并修复错误
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
            let msg = Message::Move { x: 1, y: 1 };
            if let Message::Move { x: a, y: b } = msg {
                assert_eq!(a, b);
            } else {
                panic!("不要让这行代码运行！");
            }
        }
        main();
        println!("code3");
    }
    fn code4() {
        #[derive(Debug)]
        // 填空，并修复错误
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
            let msgs: [Message; 3] = [
                Message::Quit,
                Message::Move { x: 1, y: 3 },
                Message::ChangeColor(255, 255, 0),
            ];

            for msg in msgs {
                show_message(msg);
            }
        }

        fn show_message(msg: Message) {
            println!("{:?}", msg);
        }
        main();
        println!("code4");
    }
    fn code5() {
        // 填空让 `println` 输出，同时添加一些代码不要让最后一行的 `panic` 执行到
        fn main() {
            let five = Some(5);
            let six = plus_one(five);
            let none = plus_one(None);

            if let Some(n) = six {
                println!("{}", n);
                return
            }

            panic!("不要让这行代码运行！");
        }

        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }
        main();
        println!("code5");
    }
    code1();
    code2();
    code3();
    code4();
    code5();
}
fn main() {
    // example();
    practice();
}
