pub fn build_proverb(list: Vec<&str>) -> String {
    let len = list.len();
    let mut proverb = String::from("");
    if len == 0 {
        return proverb;
    }

    for i in 0..len-1 {
        proverb += &format!("For want of a {} the {} was lost.\n", list[i], list[i+1])
    }
    proverb += &format!("And all for the want of a {}.", list[0]);
    proverb
}
