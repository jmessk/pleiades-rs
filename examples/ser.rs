use std::borrow::Cow;

fn main() {
    let api = {
        let msg = "Hello, world!".to_string();
        let msg = Msg::new(msg).msg;
        // let api = Api::new("asdf"); // ok
        let api = Api::new(msg); // ok
                                 // let api = Api::new(msg.as_str()); // err
        api
    };
    println!("{}", api.msg);
}

struct Msg {
    pub msg: String,
}

impl Msg {
    fn new(msg: impl Into<String>) -> Msg {
        Msg { msg: msg.into() }
    }
}

#[derive(serde::Deserialize)]
struct Api<'a> {
    msg: Cow<'a, str>,
}

impl<'a> Api<'a> {
    fn new(msg: impl Into<Cow<'a, str>>) -> Api<'a> {
        Api { msg: msg.into() }
    }
}
