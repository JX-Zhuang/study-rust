fn example() {
    fn code1() {
        let x = Some(5);
        let y = 10;
        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched,y = {:?}", y),
            _ => println!("Default case,x = {:?}", x),
        }
        println!("at the end: x = {:?}, y = {:?}", x, y);
        println!("code1");
    }
    fn code2() {
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
        println!("code2");
    }
    fn code3() {
        let x = 5;
        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
        println!("code3");
    }
    fn code4() {
        let x = 'c';
        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
        println!("code4");
    }
    fn code5() {
        struct Point {
            x: i32,
            y: i32,
        }
        fn main() {
            let p = Point { x: 0, y: 7 };
            let Point { x: a, y: b } = p;
            assert_eq!(p.x, a);
            assert_eq!(p.y, b);
        }
        main();
        println!("code5");
    }
    fn code6() {
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };
        match p {
            Point { x, y: 0 } => println!("On the x axis at {}", x),
            Point { x: 0, y } => println!("On the y axis at {}", y),
            Point { x, y } => println!("On neither axis: ({}, {})", x, y),
        }
        println!("code6");
    }
    fn code7() {
        enum Message {
            Quit,
            Move {
                x: i32,
                y: i32,
            },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        let msg = Message::ChangeColor(0, 160, 255);
        match msg {
            Message::Quit => println!("The Quit variant has no data to destructure."),
            Message::Move { x, y } => println!("x:{},y:{}", x, y),
            Message::Write(text) => println!("{}", text),
            Message::ChangeColor(r, g, b) => println!("r:{},g:{},b:{}", r, g, b),
        }
        println!("code7");
    }
    fn code8() {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }

        enum Message {
            Quit,
            Move {
                x: i32,
                y: i32,
            },
            Write(String),
            ChangeColor(Color),
        }

        fn main() {
            let msg = Message::ChangeColor(Color::Rgb(0, 160, 255));

            match msg {
                Message::ChangeColor(Color::Rgb(r, g, b)) => {
                    println!("Change the color to red {}, green {}, and blue {}", r, g, b)
                }
                Message::ChangeColor(Color::Hsv(h, s, v)) => {
                    println!("Change the color to hue {}, saturation {}, and value {}", h, s, v)
                }
                _ => (),
            }
        }
        main();
        println!("code8");
    }
    fn code9() {
        let arr: &[u16] = &[114, 514];
        if let [x, ..] = arr {
            assert_eq!(x, &114);
        }
        if let &[.., y] = arr {
            assert_eq!(y, 514);
        }
        let arr: &[u16] = &[];
        assert!(matches!(arr, [..]));
        assert!(!matches!(arr, [x, ..]));
        println!("code9");
    }
    fn code10() {
        enum Message {
            Hello {
                id: i32,
            },
        }
        let msg = Message::Hello { id: 5 };
        match msg {
            Message::Hello { id: id_variable @ 3..=7 } => {
                println!("Found an id in range: {}", id_variable);
            }
            Message::Hello { id: 10..=12 } => {
                println!("Found an id in another range");
            }
            Message::Hello { id } => {
                println!("Found some other id: {}", id);
            }
        }
        println!("code10");
    }
    fn code11() {
        #[derive(Debug)]
        struct Point {
            x: i32,
            y: i32,
        }
        let p @ Point { x: px, y: py } = Point { x: 10, y: 23 };
        println!("x: {}, y: {}", px, py);
        println!("{:?}", p);
        let point = Point { x: 10, y: 5 };
        if let p @ Point { x: 10, y } = point {
            println!("x is 10 and y is {} in {:?}", y, p);
        } else {
            println!("x was not 10 :(");
        }
        println!("code11");
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
    code10();
    code11();
}
fn practice() {
    fn code1() {
        fn main() {
            match_number(1);
            match_number(3);
            match_number(6);
            match_number(11);
        }
        fn match_number(n: i32) {
            match n {
                // 匹配一个单独的值
                1 => println!("One!"),
                // 使用 `|` 填空，不要使用 `..` 或 `..=`
                2 | 3 | 4 | 5 => println!("match 2 -> 5"),
                // 匹配一个闭区间的数值序列
                6..=10 => { println!("match 6 -> 10") }
                _ => { println!("match 11 -> +infinite") }
            }
        }
        main();
        println!("code1");
    }
    fn code2() {
        struct Point {
            x: i32,
            y: i32,
        }
        fn main() {
            // 填空，让 p 匹配第二个分支
            let p = Point { x: 2, y: 10 };
            match p {
                Point { x, y: 0 } => println!("On the x axis at {}", x),
                // 第二个分支
                Point { x: 0..=5, y: y @ (10 | 20 | 30) } => println!("On the y axis at {}", y),
                Point { x, y } => println!("On neither axis: ({}, {})", x, y),
            }
        }
        main();
        println!("code2");
    }
    fn code3() {
        // 修复错误
        enum Message {
            Hello {
                id: i32,
            },
        }
        fn main() {
            let msg = Message::Hello { id: 5 };
            match msg {
                Message::Hello { id: id @ 3..=7 } => println!("id 值的范围在 [3, 7] 之间: {}", id),
                Message::Hello { id: newid @ (10 | 11 | 12) } => {
                    println!("id 值的范围在 [10, 12] 之间: {}", newid)
                }
                Message::Hello { id } => println!("Found some other id: {}", id),
            }
        }
        main();
        println!("code3");
    }
    fn code4() {
        // 填空让代码工作，必须使用 `split`
        fn main() {
            let num = Some(4);
            let split = 5;
            match num {
                Some(x) if x < split => assert!(x < split),
                Some(x) => assert!(x >= split),
                None => (),
            }
        }
        main();
        println!("code4");
    }
    fn code5() {
        let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);
        match numbers {
            (first, .., last) => {
                assert_eq!(first, 2);
                assert_eq!(last, 2048);
            }
        }
        println!("code5");
    }
    // 修复错误，尽量少地修改代码
    // 不要移除任何代码行
    fn code6() {
        let mut v = String::from("hello,");
        let r = &mut v;
        let a = match r {
            value => {
                value.push_str(" world!");
                value
            }
        };
        println!("{}", a);
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
