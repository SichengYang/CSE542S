//main.rs
//Qinzhou Song and Sicheng Yang

fn main() {
    #[cfg(oldexercise)]
    {
        let x = 6;
        let y = 2;

        let result = x * y + x + y;

        println!("Result: {}", result);
    }

    #[cfg(oldexercise)]
    {
        let x = 6;
        let y = 3;

        println!("Larger: {}", {if x > y {x} else {y}});
    }

    #[cfg(oldexercise)]
    {
        let tuple = (6, 3);

        println!("Larger: {}", {match tuple { (x, y) => if x > y {x} else {y} }});
    }

    let mut v = Vec::new();

    for x in 0..10 {
        v.push(x);
    }

    println!("Vector: {:?}", v);

    for j in &v {
        println!("Element: {}", j);
    }

    println!("Vector: {:?}", v);
}
