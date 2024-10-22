struct S<T>{
    x: T,
    y: fn(T) -> T,
    z: fn(T) -> T
}

fn main() {
    let x: f64 = 0.0;
    let cos_sin = |x| {
        println!("cos: {}", f64::cos(f64::sin(x)));
    };

    cos_sin(x);

    let s = S::<f64>{x: 0.0, y: f64::cos, z: f64::sin};
    let s_closure = |s: S<f64>| {
        println!("cos: {}", (s.y)((s.z)(s.x)));
    };

    s_closure(s);

    let mut string = "Hello".to_string();
    let str_closure = |mut s: String| -> String {
        s = s + ", world!";
        s
    };

    println!("orinigal string: {}", string);
    println!("string closure: {}", str_closure(string));
}
