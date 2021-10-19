use rand::distributions::Slice;
use rand::Rng;

const UPPER_LITERAL: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

const LOWER_LITERAL: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

const NUMBERIC: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

const SPECIAL_LITERAL: [char; 9] = ['&', '#', '$', '!', '@', '^', '*', '<', '>'];

pub enum PwdRule {
    Upper = 1 << 0,
    Lower = 1 << 1,
    Digit = 1 << 2,
    Special = 1 << 3,
}

pub fn gen_pwd(num: u8, len: u8, rule: usize) -> Vec<String> {
    let mut items = vec![];

    if rule & PwdRule::Upper as usize != 0 {
        items.append(&mut UPPER_LITERAL.to_vec());
    }
    if rule & PwdRule::Lower as usize != 0 {
        items.append(&mut LOWER_LITERAL.to_vec());
    }
    if rule & PwdRule::Digit as usize != 0 {
        items.append(&mut NUMBERIC.to_vec());
    }
    if rule & PwdRule::Special as usize != 0 {
        items.append(&mut SPECIAL_LITERAL.to_vec());
    }

    if items.is_empty() {
        return vec![];
    }
    let rng = &mut rand::thread_rng();
    let mut res = vec![];
    for _ in 0..num {
        // let r: Vec<char> = items
        // .choose_multiple(&mut rng, len as usize)
        //     .take(len)
        //     .cloned()
        //     .collect();
        // items.shuffle(&mut rng);
        let s = Slice::new(&items).unwrap();
        let r: String = rng.sample_iter(&s).take(len as usize).collect();
        res.push(r);
    }
    res
}
