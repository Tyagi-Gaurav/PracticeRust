use time::convert::Second;

pub fn build_proverb(list: &[&str]) -> String {
    let mut proverbs = String::from("");

    if list.len() > 1 {
        let mut i = 1;
        while i < list.len() {
            proverbs.push_str(format!("For want of a {} the {} was lost.", list[i-1], list[i]).as_str());
            proverbs.push_str("\n");
            i = i + 1;
        }
    }

    if list.len() >= 1 {
        proverbs.push_str(format!("And all for the want of a {}.", list[0]).as_str() );
    }

    return proverbs;
}
