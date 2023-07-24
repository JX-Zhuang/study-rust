fn example() {
    // fn code1() {
    //     fn main() {
    //         let my_name = "Pascal";
    //         greet(my_name);
    //     }

    //     fn greet(name: String) {
    //         println!("Hello, {}!", name);
    //     }
    //     main();
    // }
    fn code2() {
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
        let s1 = &s[..];
        println!("s is {},slen is {}", *&s, s.len());
        println!("hello is {}", hello);
        println!("world is {}", world);
        assert_eq!(s, s1);
    }
    fn code3() {
        let s = "中国人";
        let a = &s[0..3];
        println!("a is {}", a);
        println!("{}", s.len());
    }
    // fn code4() {
    //     fn main() {
    //         let mut s = String::from("hello world");
    //         let word = first_word(&s);
    //         s.clear(); // error!
    //         println!("the first word is: {}", word);
    //     }
    //     fn first_word(s: &String) -> &str {
    //         &s[..1]
    //     }
    //     main();
    // }
    fn code5() {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
    }
    fn code6() {
        let mut s = String::from("hello ");
        s.push_str("rust");
        println!("追加字符串 push_str() -> {}", s);
        s.push('!');
        println!("追加字符 push() -> {}", s);
    }
    fn code7() {
        let mut s = String::from("Hello rust!");
        s.insert(5, ',');
        println!("插入字符 insert() -> {}", s);
        s.insert_str(6, " I like");
        println!("插入字符串 insert_str() -> {}", s);
    }
    fn code8() {
        let string_replace = String::from("I like rust. Learning rust is my favorite!");
        let new_string_replace = string_replace.replace("rust", "Rust");
        dbg!(new_string_replace);
    }
    fn code9() {
        let string_replace = "I like rust. Learning rust is my favorite!";
        let new_string_replace = string_replace.replacen("rust", "Rust", 1);
        println!("string_replace is {}", string_replace);
        dbg!(new_string_replace);
    }
    fn code10() {
        let mut string_replace_range = String::from("I like rust!");
        string_replace_range.replace_range(7..8, "R");
        println!("string_replace_range is {}", string_replace_range);
        dbg!(string_replace_range);
    }
    fn code11() {
        let mut string_pop = String::from("rust pop 中文!");
        let p1 = string_pop.pop();
        let p2 = string_pop.pop();
        dbg!(p1);
        dbg!(p2);
        dbg!(string_pop);
    }
    fn code12() {
        let mut string_remove = String::from("测试remove方法");
        println!("string_remove 占 {} 个字节", std::mem::size_of_val(string_remove.as_str()));
        string_remove.remove(0);
        string_remove.remove(3);
        dbg!(string_remove);
    }
    fn code13() {
        let mut string_truncate = String::from("测试truncate");
        string_truncate.truncate(3);
        dbg!(string_truncate);
    }
    fn code14() {
        let mut string_clear = String::from("string clear");
        string_clear.clear();
        dbg!(string_clear);
    }
    fn code15() {
        let string_append = String::from("hello ");
        let string_rust = String::from("rust");
        let result = string_append + &string_rust;
        let mut result = result + "!";
        result += "!!!";
        println!("连接字符串 + -> {}", result);
    }
    fn code16() {
        let s1 = "hello";
        let s2 = String::from("rust");
        let s = format!("{} {}!", s1, s2);
        println!("{}", s);
    }
    fn code17() {
        for c in "中国人".chars() {
            println!("{}", c);
        }
        for c in "中国人".bytes() {
            println!("{}", c);
        }
    }
    code2();
    code3();
    // code4();
    code5();
    code6();
    code7();
    code8();
    code9();
    code10();
    code11();
    code12();
    code13();
    code14();
    code15();
    code16();
    code17();
}
fn practice1() {
    fn code1() {
        // 修复错误，不要新增代码行
        let s: &str = "hello, world";
        println!("code1 is success");
    }
    fn code2() {
        // 使用至少两种方法来修复错误
        fn main() {
            let s: Box<str> = "hello, world".into();
            greetings(&s)
        }

        fn greetings(s: &str) {
            println!("{}", s)
        }
        main();
        println!("code2 is success");
    }
    fn code3() {
        let mut s = String::from("");
        s.push_str("hello, world");
        s.push('!');
        assert_eq!(s, "hello, world!");
        println!("code3 is success");
    }
    fn code4() {
        let mut s = String::from("hello");
        s.push(',');
        s.push_str(" world");
        s += "!";
        println!("{}", s);
        println!("code4 is success");
    }
    fn code5() {
        let s = String::from("I like dogs");
        // 以下方法会重新分配一块内存空间，然后将修改后的字符串存在这里
        let s1 = s.replace("dogs", "cats");
        assert_eq!(s1, "I like cats");
        println!("code5 is success");
    }
    fn code6() {
        let s1 = String::from("hello,");
        let s2 = String::from("world!");
        let s3 = s1.clone() + &s2;
        assert_eq!(s3, "hello,world!");
        println!("{}", s1);
        println!("code6 is success");
    }
    fn code7() {
        fn main() {
            let s = "hello, world";
            greetings(s);
            println!("{}", s);
        }

        fn greetings(s: &str) {
            println!("{}", s)
        }
        main();
        println!("code7 is success");
    }
    fn code8() {
        // let s = "hello, world".to_string();
        // let s1 = s;

        let s = "hello, world";
        let s1: &str = s;
        println!("code8 is success");
    }
    fn code9() {
        // 你可以使用转义的方式来输出想要的字符，这里我们使用十六进制的值，例如 \x73 会被转义成小写字母 's'
        // 填空以输出 "I'm writing Rust"
        let byte_escape = "I'm writing Ru\x73\x74!";
        println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

        // 也可以使用 Unicode 形式的转义字符
        let unicode_codepoint = "\u{211D}";
        let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

        println!("Unicode character {} (U+211D) is called {}", unicode_codepoint, character_name);

        // 还能使用 \ 来连接多行字符串
        let long_string =
            "String literals
                            can span multiple lines.
                            The linebreak and indentation here \
                             can be escaped too!";
        println!("{}", long_string);
        println!("code9 is success");
    }
    /* 填空并修复所有错误 */
    fn code10() {
        let raw_str = "Escapes don't work here: \x3F \u{211D}";
        // 修改上面的行让代码工作
        assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

        // 如果你希望在字符串中使用双引号，可以使用以下形式
        let quotes = r#"And then I said: "There is no escape!""#;
        println!("{}", quotes);

        // 如果希望在字符串中使用 # 号，可以如下使用：
        let delimiter = r###"A string with "# in it. And even "##!"###;
        println!("{}", delimiter);

        // 填空
        let long_delimiter = r###"Hello, "##""###;
        assert_eq!(long_delimiter, "Hello, \"##\"");
        println!("code10 is success");
    }
    fn code11() {
        let s1 = String::from("hi,中国");
        let h = &s1[0..1]; // 修改当前行来修复错误，提示: `h` 字符在 UTF-8 格式中只需要 1 个字节来表示
        assert_eq!(h, "h");

        let h1 = &s1[3..6]; // 修改当前行来修复错误，提示: `中` 字符在 UTF-8 格式中需要 3 个字节来表示
        assert_eq!(h1, "中");
        println!("code11 is success");
    }
    fn code12() {
        // 填空，打印出 "你好，世界" 中的每一个字符
        for c in "你好，世界".chars() {
            println!("{}", c);
        }
        println!("code12 is success");
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
    code12();
}
fn practice2() {
    // 修复代码中的错误，不要新增代码行!
    fn code1() {
        let arr = [1, 2, 3];
        let s1: &[i32] = &arr[0..2];
        let s2: &str = "hello, world";
        println!("code1 success");
    }
    fn code2() {
        let arr: [char; 3] = ['中', '国', '人'];
        let slice = &arr[..2];
        // 修改数字 `8` 让代码工作
        // 小提示: 切片和数组不一样，它是引用。如果是数组的话，那下面的 `assert!` 将会通过： '中'和'国'是char类型，char类型是Unicode编码，大小固定为4字节，两个字符为8字节。
        assert!(std::mem::size_of_val(&arr) == 12);
        assert!(std::mem::size_of_val(&slice) == 16);
        println!("code2 success");
    }
    fn code3() {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        // 填空让代码工作起来
        let slice: &[i32] = &arr[1..4];
        assert_eq!(slice, &[2, 3, 4]);
        println!("code3 success");
    }
    fn code4() {
        let s = String::from("hello");
        let slice1 = &s[0..2];
        // 填空，不要再使用 0..2
        let slice2 = &s[..2];
        assert_eq!(slice1, slice2);
        println!("code4 success");
    }
    fn code5() {
        let s = "你好，世界";
        // 修改以下代码行，让代码工作起来
        let slice = &s[0..3];
        assert!(slice == "你");
        println!("code5 success");
    }
    fn code6() {
        // 修复所有错误
        fn main() {
            let mut s = String::from("hello world");
            // 这里, &s 是 `&String` 类型，但是 `first_character` 函数需要的是 `&str` 类型。
            // 尽管两个类型不一样，但是代码仍然可以工作，原因是 `&String` 会被隐式地转换成 `&str` 类型，
            // 如果大家想要知道更多，可以看看 Deref 章节: https://course.rs/advance/smart-pointer/deref.html
            let ch = first_character(&s);
            println!("the first character is: {}", ch);
            s.clear(); // error!
        }
        fn first_character(s: &str) -> &str {
            &s[..1]
        }
        main();
        println!("code6 success");
    }
    code1();
    code2();
    code3();
    code4();
    code5();
    code6();
}
fn practice3() {
    fn code1() {
        // 填空并修复错误
        // 1. 不要使用 `to_string()`
        // 2. 不要添加/删除任何代码行
        fn main() {
            let mut s: String = String::from("hello, ");
            s.push_str("world");
            s.push('!');

            move_ownership(s.clone());

            assert_eq!(s, "hello, world!");

            println!("Success!")
        }

        fn move_ownership(mut s: String) {
            s.replace_range(0..5, "hi");
            println!("ownership of \"{}\" is moved here!", s)
        }
        main();
        println!("code1 success");
    }
    // 填空
    fn code2() {
        let mut s = String::from("hello, world");

        let slice1: &str = &s; // 使用两种方法
        assert_eq!(slice1, "hello, world");

        let slice2 = &s[0..5];
        assert_eq!(slice2, "hello");

        let slice3: &mut String = &mut s;
        slice3.push('!');
        assert_eq!(slice3, "hello, world!");

        println!("code2 success");
    }

    code1();
    code2();
}
fn main() {
    // example();
    // practice1();
    // practice2();
    practice3();
}
