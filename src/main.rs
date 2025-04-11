use regex::Regex;

fn main() {
    let folder_name =
        "[因为麻烦，所以讨厌] [demo_studio] 面倒だから嫌いだよ [Mendou dakara kirai da yo] S1";
    let re = Regex::new(r"\[(.*?)\] \[(.*?)\] (.*?) \[(.*?)\] (S\d+)").unwrap();

    if let Some(caps) = re.captures(folder_name) {
        let meaning = caps.get(1).unwrap().as_str();
        let studio = caps.get(2).unwrap().as_str();
        let japanese = caps.get(3).unwrap().as_str();
        let romaji = caps.get(4).unwrap().as_str();
        let code = caps.get(5).unwrap().as_str();

        println!("中文: {}", meaning);
        println!("资源名: {}", studio);
        println!("日文: {}", japanese);
        println!("罗马音: {}", romaji);
        println!("编号: {}", code);
    } else {
        println!("没有匹配到数据");
    }
    println!("Hello, world!");
}
