use std::borrow::Cow;

fn main() {
    let api = {
        let json = r#"{"msg":"hello"}"#.to_string();
        let api: Api = serde_json::from_str(&json).unwrap();
        api
    };
    println!("{}", api.msg);
}

#[derive(serde::Deserialize)]
struct Api<'a> {
    msg: Cow<'a, str>,
}
