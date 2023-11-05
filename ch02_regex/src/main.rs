use regex::Regex;

fn main() {
    let haystack = "phone: 111-222-3333";

    // let re = Regex::new(r"\d{3}-\d{3}-\d{4}").unwrap();
    //
    // let m = re.find(haystack).unwrap();
    // println!("{:?}", m.range());
    // println!("{}", m.as_str());
    // let re = Regex::new(r"\d{3,4}").unwrap();
    // let ms: Vec<_> = re.find_iter(haystack).map(|x| x.as_str()).collect();
    //
    // for item in ms {
    //     println!("{item}");
    // }
    // println!("-------------------------");
    //
    let re = Regex::new(r"(\d+)-(\d+)-(\d+)").unwrap();

    let mut fields: Vec<_> = vec![];

    for (_, [f1, f2, f3]) in re.captures_iter(haystack).map(|cap| cap.extract()) {
        fields.push((f1, f2, f3));
    }
    for f in fields {
        println!("{:?}", f);
    }
    println!("---------------------------");
    let caps = re.captures(haystack).unwrap();
    // println!("{}", &caps[0]);
    // println!("{}", &caps[1]);
    // println!("{}", &caps[2]);

    caps.iter()
        .for_each(|x| println!("{}", x.unwrap().as_str()));

    println!("---------------------------");
    let hay = "I categorically categorically deny having triskaidekaphobia.";
    let re = Regex::new(r"\b\w{13}\b").unwrap();

    let flag = re.is_match(hay);
    println!("{flag}");
}
