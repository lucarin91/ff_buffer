use ff_buffer;

fn main() {
    let (s, r) = ff_buffer::build::<&str>();

    let el = Box::new("hello");
    println!("send: '{}'", *el);
    s.push(el);

    let el = Box::new("world");
    println!("send: '{}'", *el);
    s.push(el);

    let el = r.pop();
    println!("receive: '{}'", *el);

    let el = r.pop();
    println!("receive: '{}'", *el);
}
