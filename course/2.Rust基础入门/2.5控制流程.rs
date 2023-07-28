fn example() {
    fn code1() {
        for i in 0..5 {
            println!("{}", i);
        }
        println!("code1");
    }
    fn code2() {
        let a = [4, 3, 2, 1];
        for (i, v) in a.iter().enumerate() {
            println!("第{}个元素是{}", i + 1, v);
        }
        println!("code2");
    }
    fn code3() {
        let mut n = 0;
        while n <= 5 {
            println!("{}!", n);
            n += 1;
        }
        println!("code3");
    }
    code1();
    code2();
    code3();
}
fn practice() {
    // 填空
    fn code1() {
        let n = 5;
        if n < 0 {
            println!("{} is negative", n);
        } else if n > 0 {
            println!("{} is positive", n);
        } else {
            println!("{} is zero", n);
        }
        println!("code1");
    }
    // 修复错误
    fn code2() {
        let n = 5;
        let big_n = if n < 10 && n > -10 {
            println!(" 数字太小，先增加 10 倍再说");
            10 * n
        } else {
            println!("数字太大，我们得让它减半");
            n / 2
        };
        println!("{} -> {}", n, big_n);
        println!("code2");
    }
    fn code3() {
        for n in 1..100 {
            // 修改此行，让代码工作
            if n == 100 {
                panic!("NEVER LET THIS RUN");
            }
        }
        println!("code3");
    }
    // 修复错误，不要新增或删除代码行
    fn code4() {
        let names = [String::from("liming"), String::from("hanmeimei")];
        for name in &names {
            // do something with name...
            println!("{:?}", name);
        }

        println!("{:?}", names);

        let numbers = [1, 2, 3];
        // numbers中的元素实现了 Copy，因此无需转移所有权
        for n in numbers {
            // do something with name...
        }

        println!("{:?}", numbers);
        println!("code4");
    }
    fn code5() {
        let a = [4, 3, 2, 1];
        // 通过索引和值的方式迭代数组 `a`
        for (i, v) in a.iter().enumerate() {
            println!("第{}个元素是{}", i + 1, v);
        }
        println!("code5");
    }
    // 填空，让最后一行的  println! 工作 !
    fn code6() {
        // 一个计数值
        let mut n = 1;

        // 当条件为真时，不停的循环
        while n < 10 {
            if n % 15 == 0 {
                println!("fizzbuzz");
            } else if n % 3 == 0 {
                println!("fizz");
            } else if n % 5 == 0 {
                println!("buzz");
            } else {
                println!("{}", n);
            }
            n += 1;
        }
        println!("n 的值是 {}, 循环结束", n);
        println!("code6");
    }
    // 填空，不要修改其它代码
    fn code7() {
        let mut n = 0;
        for i in 0..=100 {
            if n == 66 {
                break;
            }
            n += 1;
        }
        assert_eq!(n, 66);
        println!("code7");
    }
    // 填空，不要修改其它代码
    fn code8() {
        let mut n = 0;
        for i in 0..=100 {
            if n != 66 {
                n += 1;
                continue;
            }
            break;
        }
        assert_eq!(n, 66);
        println!("code8");
    }
    // 填空，不要修改其它代码
    fn code9() {
        let mut count = 0u32;
        println!("Let's count until infinity!");
        // 无限循环
        loop {
            count += 1;
            if count == 3 {
                println!("three");
                // 跳过当此循环的剩余代码
                continue;
            }
            println!("{}", count);
            if count == 5 {
                println!("OK, that's enough");
                break;
            }
        }
        assert_eq!(count, 5);
        println!("code9");
    }
    // 填空
    fn code10() {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        assert_eq!(result, 20);
        println!("code10");
    }
    // 填空
    fn code11() {
        let mut count = 0;
        'outer: loop {
            'inner1: loop {
                if count >= 20 {
                    // 这只会跳出 inner1 循环
                    break 'inner1; // 这里使用 `break` 也是一样的
                }
                count += 2;
            }
            count += 5;
            'inner2: loop {
                if count >= 30 {
                    break 'outer;
                }
                continue 'outer;
            }
        }
        assert!(count == 30);
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
fn main() {
    // example();
    practice();
}
