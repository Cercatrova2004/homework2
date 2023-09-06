fn main() {
    let chars: Vec<char> = vec!['a', 'b', 'c', 'd', 'e'];

    let new_chars: Vec<char> = chars
        .iter()
        .map(|&c| (c as u8 + 1) as char)
        .collect();

    println!("{:?}", new_chars);
}