
fn search<'a, 'b>(needle: &'a str, haystack: &'b str) -> Option<&'a str> {
    let mut checking_index = 0;
    let mut found = false;

    for c in haystack.chars() {
        if needle.chars().nth(checking_index) == Some(c) {
            checking_index += 1;
            if checking_index == needle.len() {
                found = true;
                break;
            }
        } else {
            checking_index = 0;
            found = false;
        }
    }

    if found {
        Some(needle)
    } else {
        None
    }
}

fn main() {
    let haystack = "hello little person";
    let needle = String::from("ello l");
    let res = search(&needle, haystack);

    match res {
        Some(x) => println!("found \"{}\"", x),
        None => println!("did not find!"),
    }
}