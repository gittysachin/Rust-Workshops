use std::collections::HashMap;

#[derive(Deserialize)]
struct MyStruct<'a> {
    data: &'a str,
}

fn deserialize() {
    let s: MyStruct = serde_json::from_str("{\"data\":\"Hello, world\"}");
}

fn main() {
    let strr = "hello";
    let string = String::from("hello");
    println!("{:?}", string.as_ptr());
    println!("{:?}", string.len());
    println!("{:?}", string.capacity());
    println!("{:x?}", unsafe {
        std::mem::transmute::<String, [u8; std::mem::size_of::<String>()]>(string)
    });
    println!("{:x?}", unsafe {
        std::mem::transmute::<&str, [u8; std::mem::size_of::<&str>()]>(strr)
    });

    let path = "faust.txt";
    let contents = std::fs::read(path).unwrap();
    let contents: Vec<u16> = contents
        .chunks(2)
        .map(|bytes| {
            let [first, second] = [bytes[0], bytes[1]];
            (first as u16) << 8 | (second as u16)
        })
        .collect();
    println!("{:?}", contents);
    let contents = String::from_utf16(&contents[..]).unwrap();
    println!("{}", contents);
}

fn process(string: String) -> HashMap<String, Vec<String>> {
    let mut result: HashMap<String, Vec<String>> = HashMap::new();
    for line in string.lines() {
        for sentence in line.split_terminator('.') {
            for word in sentence.trim().split_whitespace() {
                if word.chars().next().unwrap().is_uppercase() {
                    // result.insert(word, vec![sentence]);
                    result
                        .entry(word.to_string())
                        .or_default()
                        .push(sentence.to_string());
                }
            }
        }
    }
    // todo!()
    result
}

fn newprocess<'a>(string: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
    let mut result: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in string.lines() {
        for sentence in line.split_terminator('.') {
            for word in sentence.trim().split_whitespace() {
                if word.chars().next().unwrap().is_uppercase() {
                    // result.insert(word, vec![sentence]);
                    result.entry(word).or_default().push(sentence);
                }
            }
        }
    }
    // todo!()
    result
}
