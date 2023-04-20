
struct Exerpt<'a> {
    part: &'a str,
}

impl<'a> Exerpt<'a> {
    fn length(&'a self) -> usize {
        self.part.len()
    }
}

fn main() {
    let important_exerpt = Exerpt {
        part: "Major Kusanagi!",
    };
    
    let _exerpt_length = &important_exerpt.length();

    let str2 = "xyz";

    {
        let str3 = "ab";
        let res_tmp = longest(str2, str3, "Hello!");
        println!("The longest tmp string is: {}", res_tmp);
    }

    let res = longest(important_exerpt.part, str2, "Hello!");
    println!("The longest string is: {}", res);
}

use std::fmt::Display;

fn longest<'a, T>(
    x: &'a str, 
    y: &'a str,
    announce: T
) -> &'a str 
where
    T: Display, {
    
    println!("Announcement: {}", announce);
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}