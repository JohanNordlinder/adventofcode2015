pub fn run(key: &String, number_of_zeros_to_find: u32) -> String {
    for i in 0.. {
        let hash = format!("{:x}", md5::compute(format!("{0}{1}", key, i)));
        let zeros_to_find = (0..number_of_zeros_to_find).into_iter().map(|_| '0').collect::<String>();
        if hash.starts_with(zeros_to_find.as_str()) {
            return i.to_string();
        }
    }
    panic!("No answer found");
}