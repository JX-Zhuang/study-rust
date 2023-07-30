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
    code1();
    code2();
    code3();
}
fn main() {
    example();
}
