use std::env;

fn main() {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        println!("Usage: {} word", args[0]);
    } else {
        let word = &args[1];
        println!("{}", numeronym(word));
    }
}

fn numeronym(word: &str) -> String {
    if word.chars().count() < 2 {
        String::from(word)
    } else {
        let len = word.chars().count() - 2;
        let (_, first) = word.char_indices().nth(0).unwrap_or((0, ' '));
        let (_, last) = word.char_indices().last().unwrap_or((0, ' '));
        format!("{}{}{}", first, len, last)
    }
}

#[cfg(test)]
mod test {
    use super::numeronym;

    #[test]
    fn kubernetes() {
        assert_eq!("k8s", &numeronym("kubernetes")[..]);
    }

    #[test]
    fn おはようございます() {
        assert_eq!("お7す", &numeronym("おはようございます")[..]);
    }

    #[test]
    fn zero() {
        assert_eq!("h0i", &numeronym("hi")[..]);
    }
}
