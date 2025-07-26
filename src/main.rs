fn main() {
    let mut s1 = String::from("hello");
    let len = calcualte_lenth(&s1);
    let s2 = &mut s1;
    change(s2);
    println!("{len} {s2}");
}

fn calcualte_lenth(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
