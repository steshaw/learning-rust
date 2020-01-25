//
// (Modifed) Code from playground from comment by Denys Séguret:
// https://stackoverflow.com/a/58058487/482382
//

#[derive(Debug)]
struct BiRef<'a, 'b> {
    // You can't have just one lifetime here
    a: &'a str,
    b: &'b str,
}

fn lg<'a>(a: &'a str, b: &str) -> &'a str {
    println!("b = {}", b);
    &a
}

fn main() {
    let a = "A".to_string();
    let r = {
        let b = "B".to_string();
        let br = BiRef { a: &a, b: &b };
        lg(&br.a, &br.b)
    };
    println!("lg: {:?}", r);
}
