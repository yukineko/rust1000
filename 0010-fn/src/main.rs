fn main() {
    fn sum (x: i32, y: i32) -> i32 {
        x + y
    }
    let f: fn (i32, i32) -> i32  = sum;  // 明示的な型指定 fn型への変換{main::sum}からfn
    println!("{}", f(1, 2));
    let f0: fn(i32, i32) -> i32 = f; // Copyトレイトを実装しているのでコピー可能
    let f1: fn(i32, i32) -> i32 = f; // Copyトレイトを実装しているのでコピー可能
    assert_eq!(f0, f1); // Eqトレイトを実装しているので比較可能
    println!("{}", f0(1, 2));
    println!("{}", f1(1, 2));

    println!("{:p}", f); // std::fmt::Pointerが実装されている
    let f2 = sum;
    let f3 = sum;
    println!("{}", f2(1, 2));
    println!("{}", f3(1, 2));

    // assert_eq!(f2, f3); // compile error f2, f3は{main::sum}型になっている.
}   
