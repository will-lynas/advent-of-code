use regex::Regex;

pub trait StringNumberParsing {
    fn signed_nums(&self) -> Vec<i64>;
    fn unsigned_nums(&self) -> Vec<u64>;
}

impl StringNumberParsing for str {
    fn signed_nums(&self) -> Vec<i64> {
        let re = Regex::new(r"-?\d+").unwrap();
        re.find_iter(self)
            .map(|mat| mat.as_str().parse::<i64>().unwrap())
            .collect()
    }

    fn unsigned_nums(&self) -> Vec<u64> {
        let re = Regex::new(r"\d+").unwrap();
        re.find_iter(self)
            .map(|mat| mat.as_str().parse::<u64>().unwrap())
            .collect()
    }
}
