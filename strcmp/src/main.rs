fn strcmp(x: &str, y: &str) -> bool {
    let bytex = x.as_bytes();
    let bytey = y.as_bytes();

    let len1 = bytex.len();
    let len2 = bytey.len();

    let mut i = 0;
    while i < len1 && i < len2 {
        if bytex[i] < bytey[i] {
            return false;
        } else if bytex[i] > bytey[i] {
            return true;
        }
        i += 1;
    }

    len1 > len2
}

fn main(){
    let result1 = strcmp("a", "a");
    println!("result 1: {}", result1);
    let result2 = strcmp("ab", "a");
    println!("result 2: {}", result2);
    let result3 = strcmp("a", "b");
    println!("result 3: {}", result3);
}