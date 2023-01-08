fn main() {
    let x = 5;

    let x = x + 1;

    {
        // shadowing
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("X: {}, Y: {}, Z: {}", x, y, z);
    println!("X: {}, Y: {}, Z: {}", tup.0, tup.1, tup.2);

    let arr: [u8; 5] = [1, 2, 3, 4, 5];
    let same_all_arr = [100; 4];
}
