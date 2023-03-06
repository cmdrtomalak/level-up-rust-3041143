trait ToUpper {
    fn my_uppercase(&self) -> String;
}

impl ToUpper for &str {
    fn my_uppercase(&self) -> String {
        self.to_uppercase()
    }
}

fn sort_usernames<T: AsRef<str> + PartialOrd + ToUpper>(usernames: &mut Vec<T>) {
    usernames.sort_by(|a, b| a.my_uppercase().partial_cmp(&b.my_uppercase()).unwrap());
}

fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);
    sort_usernames(&mut users);
    println!("sorted:   {:?}", &users);
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}
