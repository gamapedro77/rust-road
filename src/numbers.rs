
pub fn range() {
    let mut sum = 0;
    for i in -3..2 {
        println!("{}", i);
        sum += i
    }
    println!("{}", sum);
    assert!(sum == -5);

    for c in 'a'..='z' {
        print!("{} ",c);
    }
}


pub fn representations() {
    let _v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    let _x = 1_000.000_1;

    let f = 0.1 + 0.2;
    println!("{}", type_of(&f));
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}