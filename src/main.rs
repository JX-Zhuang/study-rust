fn main() {
    let a: u8 = 255;
    let b = a.wrapping_add(20);
    println!("{}", b);
    let c = 0.1;
    let d = 0.2;
    let e = c + d;
    println!("{}", e);
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    assert!(abc.0 + abc.1 == abc.2);
    // let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    // assert!(xyz.0 + xyz.1 == xyz.2);
    let x = (-42.0_f32).sqrt();
    if x.is_nan() {
        println!("未定义的数学行为");
    }
}
