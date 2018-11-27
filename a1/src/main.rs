
use std::str;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn char_to_int(r: char, sub: u8) -> u8 {
    //convert character to int a=0, b=1...
    r as u8 - sub
}
fn int_to_char(i: u8, sub: u8) -> char {
    //convert int to character
    (i + sub) as char
}

fn to_int_upper(r: char) -> u8 {
    //convert to integer from upper case character
    char_to_int(r, 65)
}

fn from_int_lower(i: u8) -> char {
    //convert from int to char (lower case char)
    int_to_char(i, 97)
}


fn str_upper_to_int(c: &mut str::Chars) -> Vec<u8> {
    //convert string to a vector of ints (string is upper case)
    let mut vec: Vec<u8> = Vec::new();
    loop {
        match c.next() {
            Some(x) => {
                vec.push(to_int_upper(x));
            }
            None => return vec,
        }
    }
}

fn upper_to_lower_morph(c: &str::Chars, a: u8, b: u8) -> String {
    //Affine encoding from a upper case string to a lower case encoded
    let mut cc = c.clone();
    int_to_str_lower(&mut encode_affine(&str_upper_to_int(&mut cc), a, b))
}



fn int_to_str_lower(c: &mut Vec<u8>) -> String {
    //vector of ints converted to lowercase string
    let mut vec: Vec<char> = Vec::new();
    for i in 0..c.len() {
        vec.push(from_int_lower(c[i]));
    }
    vec.into_iter().collect()
}
fn affine_single(c: &u8, a: u8, b: u8) -> u8 {
    //affine encode a single int
    modb((c.clone() as isize * a as isize) + (b as isize)) as u8
}
fn deaffine_single(c: &u8, a1: u8, b: u8) -> u8 {
    //affine decode a single int
    modb((c.clone() as isize - b as isize) * a1 as isize) as u8
}

fn encode_affine(c: &Vec<u8>, a: u8, b: u8) -> Vec<u8> {
    //affine encode set of ints
    let mut x = c.clone();
    for i in 0..x.len() {
        x[i] = affine_single(&x[i], a, b)
    }
    x
}

fn decode_affine(c: &Vec<u8>, a1: u8, b: u8) -> Vec<u8> {
    //decode affine for set of ints
    let mut x = c.clone();
    for i in 0..x.len() {
        x[i] = deaffine_single(&x[i], a1, b)
    }
    x
}

fn de_vig(c: &mut Vec<u8>, key: &Vec<u8>) {
    //decode vigenere cipher using the provided key
    //this could be easily edited to accomodate a 'moving' key as suggested in P4
    let l = key.len();
    for i in 0..c.len() {
        c[i] = modb((c[i] + key[i % l]) as isize) as u8;
    }
}

fn ceaser_1(c: &mut Vec<usize>) {
    //ceaser shift the vector of items in place, shift by 1
    //rather than shifting and re-counting, as we are maping A->B, B->C
    //we can just manually take Z->A and just shuffle all by 1 position in the 
    //distribution vector.
    let l = c.len();
    let last = c[l - 1];
    for i in 1..l {
        c[l - i] = c[l - 1 - i];
        //println!("{}",l-i);
    }
    c[0] = last;
}

fn modb_n(x: isize, m: usize) -> isize {
    //modulo by base n
    let mut x = x.clone();
    while x < 0 {
        x += m as isize;
    }
    x % (m as isize)
}

pub fn modb(x: isize) -> isize {
    //modulo by base 26 (for alphabet)
    modb_n(x, 26)
}
fn modb_inv(x: isize) -> isize {
    //find the modulo inverse of x
    //fine j S.T. x*j==1, j must be less than 26 if it exists
    let x = modb(x);
    for j in 0..26 {
        if modb(x * j) == 1 {
            return j;
        }
    }
    return -1;
}
impl Wordlist {
    //unused word checker implemented, was going to be used for vigenere, maybe used in future asignments
    /*fn check_n(&self, s: &String,from: usize,to:usize) -> Option<String> {
       let mut s = s.clone();
        for i in 0..to-from{
            s.truncate(to-i);
            if self.check(&s){
                 return Some(s.clone());
            }
        }
        None
    }*/

    fn check(&self, s: &String) -> bool {
        //checks the wordlist for a string (word)
        match self.data.get(&s.len()) {
            Some(l) => l.iter().any(|x| *x == *s),
            None => false,
        }
    }

    fn new() -> Wordlist {
        //generates new wordlist for checking (from file)
        //https://github.com/dwyl/english-words/blob/master/words_alpha.txt
        let filename = "wordlist.txt";
        let mut f = File::open(filename).expect("file not found, make sure it's in CWD");
        let mut contents = String::new();
        f.read_to_string(&mut contents).expect(
            "something went wrong reading the file",
        );
        let mut split = contents.split_whitespace();
        let mut wordlist: HashMap<usize, Vec<String>> = HashMap::new();
        loop {
            match split.next() {
                Some(x) => {
                    let lenlist = wordlist.entry(x.len()).or_insert(Vec::new());
                    lenlist.push(x.to_owned());
                }
                None => return Wordlist { data: wordlist },
            }
        }
    }
}


struct Wordlist {
    //struct to hold set of english words (based on word size)
    data: HashMap<usize, Vec<String>>,
}


#[derive(Clone)]
struct Letter {
    //A letter and it's frequency in the english language
    letter: char,
    freq: f64,
}

#[derive(Clone)]
struct LetterFreq {
    //set of english letters and their frequency
    freq: Vec<Letter>,
}

impl LetterFreq {
    fn new() -> LetterFreq {
        //frequency distribution of english words
        return LetterFreq {
            freq: vec![
                Letter {
                    letter: 'a',
                    freq: 0.082f64,
                },
                Letter {
                    letter: 'b',
                    freq: 0.015f64,
                },
                Letter {
                    letter: 'c',
                    freq: 0.028f64,
                },
                Letter {
                    letter: 'd',
                    freq: 0.043f64,
                },
                Letter {
                    letter: 'e',
                    freq: 0.127f64,
                },
                Letter {
                    letter: 'f',
                    freq: 0.022f64,
                },
                Letter {
                    letter: 'g',
                    freq: 0.02f64,
                },
                Letter {
                    letter: 'h',
                    freq: 0.061f64,
                },
                Letter {
                    letter: 'i',
                    freq: 0.07f64,
                },
                Letter {
                    letter: 'j',
                    freq: 0.002f64,
                },
                Letter {
                    letter: 'k',
                    freq: 0.008f64,
                },
                Letter {
                    letter: 'l',
                    freq: 0.04f64,
                },
                Letter {
                    letter: 'm',
                    freq: 0.024f64,
                },
                Letter {
                    letter: 'n',
                    freq: 0.067f64,
                },
                Letter {
                    letter: 'o',
                    freq: 0.075f64,
                },
                Letter {
                    letter: 'p',
                    freq: 0.019f64,
                },
                Letter {
                    letter: 'q',
                    freq: 0.001f64,
                },
                Letter {
                    letter: 'r',
                    freq: 0.06f64,
                },
                Letter {
                    letter: 's',
                    freq: 0.063f64,
                },
                Letter {
                    letter: 't',
                    freq: 0.091f64,
                },
                Letter {
                    letter: 'u',
                    freq: 0.028f64,
                },
                Letter {
                    letter: 'v',
                    freq: 0.01f64,
                },
                Letter {
                    letter: 'w',
                    freq: 0.023f64,
                },
                Letter {
                    letter: 'x',
                    freq: 0.001f64,
                },
                Letter {
                    letter: 'y',
                    freq: 0.02f64,
                },
                Letter {
                    letter: 'z',
                    freq: 0.001f64,
                },
            ],
        };
    }

// lookup a single character for distribution, probably never be used as
//ints will be used anyway. Plus freq dist is in order so...
    /*fn lookup(&self,c:&char)->f64{
        //will panic if bad char
        let mut f = self.freq.clone();
        f.retain(|l| l.letter==*c);
        f[0].freq
    }*/
    
    fn lookup_n(&self, c: usize) -> f64 {
    //lookup distribution of letter in 'c' place in alphabet
        self.freq[c].freq
    }

    fn get_expected_dist(&self, n: usize) -> Vec<usize> {
        //get expected distribution of letter given the size of text
        let mut v = Vec::new();
        for i in 0..26 {
            v.push((self.lookup_n(i) * n as f64).round() as usize);
        }
        v
    }

    fn get_x2(&self, n: usize, dist: &Vec<usize>) -> usize {
        //get the sum of the differences squared of the actual distrubution of letters,
        //compared to the expected distribution of letters.
        let expected = self.get_expected_dist(n);
        let mut x2 = 0;
        for i in 0..dist.len() {
            x2 += (dist[i] as isize - expected[i] as isize).pow(2) as usize;
        }
        x2
    }

    fn get_best_x2_shift(&self, dist: Vec<usize>) -> u8 {
        //finds the best shift to minimize the x2 differences
        let n = dist.clone().iter().fold(0usize, |sum, val| sum + val);
        let mut best_i = 0u8;
        //let expected = self.get_expected_dist(n);
        let mut best_x2 = <usize>::max_value();
        let mut dist = dist.clone();
        for i in 0u8..27 {
            let x2 = self.get_x2(n, &dist);
            //println!("{:?}\n{:?}",dist,expected);
            if x2 < best_x2 {
                // println!("\tFound new x2 {} for key {}",x2,from_int_lower(i));
                best_x2 = x2;
                best_i = i;
            }
            ceaser_1(&mut dist);
        }
        best_i
    }
}


fn get_cipher_text(i: i64) -> &'static str {
    //static cipher texts from assignment
    //s42639277
    match i {
        1 => "PHCOBNEQ",
        2 => "IPYKMKMPFASCFPFWYAJRTFKTPSJXWTSDMITWTAYX",
        3 => {
            "HCPCEIPZTWMOOBKCHUAEANJOWUPOEBTYFAJNLJRVPXNSRBVKYXURLXGCEYRZTHIYFNKXEIRYZFUYQAQVOQJOCYXOCNJOMIWQSMRKCNGNFHFOCNJOELGODNQDSYNOQNQPSCODSYIBZOPNHUUWTMVIHCVRMFWOMYNVDNJOLCTCPYOOONQUTMUYYYUCVCPSEQCCEBGCPWQXOIHWLSHBZGUYXYYRPLGNPYROCCPDSYJOLLVYQNJOHIQNNUOOEBGNCIPSYAQPCCPQOIXODBGGLMCLTNGKCFADSYTOSUFLPYPXZXKPQCEEWNKODUDYFNVRPDQECHGILHFDSYISCFYKDMQOGCFOYNNIPRROCCGXNYFDSUVRPQCCWYUCQLKQSNGXPXVRLHJOHIWVOHQBXUNVJBCFPVGOYJTODOOKMFACSYEYFFFLPNTEDNGNEIHSYXCCLZGZWUEOTHIOYYTKWSQENIWVOHQDLMUEXYVRLNAYFQGBPGWMSMCPPLKXEBGMZOPDCSVRLHKXWIPNZHVRPLGGPLGXZNGVPMEBPYPCZZEYFLUOMOVDSYTOHUUKWQCIDNJOOUPQPLQPNIPMPUNOOGKMCIRRZHGCMSYRTWJIZOTFZCEOXCIREVGZTWMOOORKYXTONIIXTTGNMYUSOYUSEQCCYIVOLMADZGCUPULYFLPOJVAIZOTCPFHGTNJYFNCDELCMECPQLNVOYNKYYZQBOCUDLHEODIHVPMUDSUPUTFQWPNTODCVGLMPYEHGMPMUKCSVYRYVIZOTZLMUZZLVOYXQBDYFLFNUYXYVSXYUDSYTOHYTOAUVBZFURLHISYACLZOVDSYTKTFYKJMVKECQXDQJYPRCWTHGNEBGZLJGBDIHKYSRKCNAWPGDOCNJOJZQEYXVRPLGKYXCCVYFKHEYKCXSEPMVSZHURZQGFPLPYAUVBZFURLXCZAYCBPXCXOIPDSYYKWEHBZGVRPMVKECQXSYJKOGCNPMWBPVAMLOVSZOULLWMGLLFQWUPMPMVRLNJOHUUXZN"
        }
        _ => panic!("bad input"),
    }
}

fn p1a() {
    //find sqrt(-1) in Z85; x^2 = -1 mod 85 0<=x<=84
    //or x^2=84 0<=x<=84 (96=1; 85=0; 84=-1)
    println!("=====P1A=====");
    println!(
        "Searching for sqrt(-1) in Z85; find x S.T. x^2=84 0<=x<85 (via brute force, incrementing x from 0 to 84)"
    );
    for x in 0..84 {
        if x * x % 85 == 84 {
            println!("Found branch at x = {} + 85k (where k is also in Z85)", x);
        }
    }
}
fn p1b() {
    //find solutions to log_3(8) in Z17
    println!("=====P1B=====");
    println!("Find log_3(8) in Z17; or find k S.T. 3^k=8 mod17 0<=k<17 ");
    println!("Starting at k=0, incrementing to k=16");
    for k in 0..17 {
        if modb_n(3isize.pow(k), 17) == 8 {
            println!("Found solution log_3(8)={}", k);
        }
    }
}
fn p1c() {
    //use euclids method for gcd, find gcd of 547 and 1337
    println!("=====P1C=====");
    println!("For Euclideans, check difference in size of numbers to replace the larger number.");
    println!(
        "Rather than using subtraction to find the difference, modulu can be used in either direction (a%b or b%a)"
    );
    println!("gcd(547,1337)={}", gcd(547, 1337));

    println!("Find inverse of 547 in Z1337; find x S.T. x*547 = 1 mod1337");
    println!("Starting at x=1, and incrementing");
    let mut x = 1;
    loop {
        if modb_n(x * 547, 1337) == 1 {
            println!(
                "Found solution, inverse 547 in Z1337={}; (547*{0})=1 mod1337",
                x
            );
            break;
        }
        x += 1;
    }
}

fn gcd(a: usize, b: usize) -> usize {
    //a is larger, b is smaller (after initial iteration)
    //while the smaller (b) is not 0 (they remainders match)
    //keep splitting into smaller pieces of remainders
    println!("a:{}, b:{}", a, b);
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
pub fn p2a() {
    println!("=====P2A=====");
    //Get cipher
    let mut cipher = &mut get_cipher_text(1).chars();
    let mut v = Vec::new();
    println!("Potential plain texts;");
    for i in 0..26 {
        //morph cipher (1*character + i)mod26
        let temp = upper_to_lower_morph(&mut cipher, 1, i);
        println!("{} ({}x+{})mod{}", temp, 1, i, 26);
        v.push(temp);
    }
    print!("Generating wordlist...");
    let wordlist = Wordlist::new();
    println!("Done");
    println!("Checking candidates for valid words...");
    let mut i = 0;
    for s in v {
        if wordlist.check(&s) {
            println!("MATCH! {}", s);
            println!("Correct shift is a={}.", i);
        }
        i += 1;
    }
}



pub fn p2b() {
    let cipher = &mut get_cipher_text(2).chars();
    let i_cipher = to_int_upper(cipher.next().unwrap()) as isize;
    let t_cipher = to_int_upper(cipher.next().unwrap()) as isize;
    let i = to_int_upper('I') as isize;
    let t = to_int_upper('T') as isize;

    let mut a: isize = 1;
    println!("=====P2B=====");
    println!("Convert known cipher and plaintext to ints (a=0,b=1...)");
    println!(
        "i:{},I:{},t:{},T:{}, meaning {0}->{1} and {2}->{3}",
        i,
        i_cipher,
        t,
        t_cipher
    );
    println!("Find a S.T. a*(i-t) mod 26 = I-T mod 26, starting with a=1 then incrementing");
    loop {
        if modb(a * (i - t)) == modb(i_cipher - t_cipher) {
            break;
        }
        a += 1;
    }
    let mut a1: isize = 1;
    println!("Find inverse a (a1) S.T. a*a1 mod26 = 1, starting with a1=1 then incrementing");
    loop {
        if modb(a1 * a) == 1 {
            break;
        }
        a1 += 1;
    }
    println!("Find b S.T. b = I-(a*i) mod26");
    let b: isize = modb(i_cipher - (a * i));
    println!("a:{}, b:{}, a1:{}", a, b, a1);
    let mut result = decode_affine(
        &str_upper_to_int(&mut get_cipher_text(2).chars()),
        a1 as u8,
        b as u8,
    );

    println!(
        "Using inverse a({}) and b({}), we can decode using (c-b)*a1 mod26 (where c is ciphertext);\n{}",
        a1,
        b,
        int_to_str_lower(&mut result)
    );
}

fn kasiski(cipher: &mut String, min_keylength: usize, max_keylength: usize) -> usize {
    println!("----Kasiski----");
    //get cipher text
    let mut sets: HashMap<String, Vec<usize>> = HashMap::new();

    println!(
        "Generating matches of length {} through {} of ciphertext...",
        min_keylength,
        max_keylength
    );
    for key in min_keylength..max_keylength {
        for i in 0..cipher.len() - key - 1 {
            let k = &cipher[i..i + key];
            let entry = sets.entry(k.clone().to_owned()).or_insert(Vec::new());
            entry.push(i);
        }
    }
    println!("Reducing to only those who appear more than once...");
    sets.retain(|_, value| value.len() > 1);
    println!("Changing set to contain distance between duplicates, instead of positions...");
    for (_, val) in sets.iter_mut() {
        let mut val2: Vec<usize> = Vec::new();
        for v1 in 0..val.len() {
            for v2 in (v1 + 1)..val.len() {
                val2.push(val[v2] - val[v1]);
            }
        }
        *val = val2;
    }
    println!("Consolidating distances, regardless of keylength...");
    //println!("{:?}",sets);
    let mut mults: Vec<usize> = Vec::new();
    for (_, val) in sets.iter() {
        for v in val.clone() {
            mults.push(v);
        }
    }
    println!(
        "Checking distances by suspected keylengths {} through {}...",
        min_keylength,
        max_keylength
    );
    let total = mults.len();
    let mut best = 0usize;
    let mut best_value = 0;
    println!("Using arbitrary 'value' to guess which keylength is the best (but longest) fit");
    println!("Arbitrary value = match_percent*match_percent*key_length");
    println!(
        "This prioritises the match percent (as an integer) significantly more than keylength, but will still prefer those with higher keylengths."
    );
    for m in min_keylength..max_keylength {
        let mut x = 0 as isize;
        for v in mults.clone() {
            if v % m == 0 {
                x += 1;
            }
        }
        let p = (((x * 100) as f64) / (total as f64)) as usize;
        println!(
            "Keylength of {} matched {}% duplicate cipher distances",
            m,
            p
        );
        //Value percent matched highest, value keylength next
        if best_value < p * p * m {
            best_value = p * p * m;
            best = m;
            println!(
                "New best keylength:{}, with new arbitrary value:{}",
                best,
                best_value
            );
        }
    }
    println!("Final best keylength; {}", best);
    println!("----Kasiski DONE----");
    best
}
fn freidman_1(cipher: &mut String, min_keylength: usize, max_keylength: usize) -> usize {
    println!("----Freidman1----");
    let cipherv = str_upper_to_int(&mut cipher.chars());
    let mut best = 0usize;
    let mut best_value = <usize>::max_value();
    for k in min_keylength..max_keylength {
        //println!("Try keylength = {}",k);
        let mut freq: Vec<Vec<isize>> = vec![vec![0isize; 26]; k];
        let mut i = 0usize;
        while i < cipher.len() {
            let c = cipherv[i];
            freq[i % k][c as usize] += 1;
            i += 1;
        }
        //println!("{:?}",freq);
        let mut k0 = vec![0f64; k];
        let l = cipher.len() / k;
        for i in 0..k {
            k0[i] = freq[i].iter().fold(0, |k0, &f| k0 + (f * (f - 1))) as f64 /
                ((l * (l - 1)) as f64);
        }
        print!("For keylength {} φ = ", k);
        let avg = k0.iter().fold(0f64, |r, &x| r + x) / k0.len() as f64;
        print!("Avg({:.4}) ", avg);
        k0.iter().for_each(|&x| print!("{:.3}, ", x));
        println!();
        if (avg * 10000f64 - 667f64).abs() < best_value as f64 {
            best_value = (avg * 10000f64 - 667f64).abs().round() as usize;
            best = k;
        }
    }
    println!("Closest keylength to φ = 0.0667; {}", best);
    println!("----Freidman1 DONE----");
    best
}
fn freidman_2(cipher: &mut String) -> usize {
    println!("----Freidman2----");
    let cipherlength = cipher.len();
    println!("Find k0...");
    let mut freq: Vec<usize> = vec![0; 26];

    for c in cipher.chars() {
        freq[to_int_upper(c) as usize] += 1;
    }
    println!(
        "Frequency distribution (where first is occurences of a, second of b...);\n{:?}",
        freq
    );
    let k0 = freq.iter().fold(0, |k0, &f| k0 + (f * (f - 1))) as f64 /
        ((cipherlength * (cipherlength - 1)) as f64);
    let kp = 0.0667f64;
    let kr = 1f64 / 26f64; // 1/26
    println!("k0={:.4}, kp={:.4}, kr={:.4}", k0, kp, kr);
    println!(
        "Final keysize guess (as float): {:.4}",
        (kp - kr) / (k0 - kr)
    );
    println!("----Freidman2 DONE----");
    ((kp - kr) / (k0 - kr)).round() as usize
}

fn solve_vig(cipher: &mut String, keysize: usize) -> String {
    println!("----Finding Key----");
    println!("Checking for keysize = {}", keysize);

    let cipherv = str_upper_to_int(&mut cipher.chars());
    let mut freq: Vec<Vec<usize>> = vec![vec![0usize; 26]; keysize];
    let mut i = 0usize;
    println!(
        "Generating {} sets of frequency distributions (keylength)",
        keysize
    );
    while i < cipher.len() {
        let c = cipherv[i];
        freq[i % keysize][c as usize] += 1;
        i += 1;
    }
    let lf = LetterFreq::new();

    let mut key: Vec<u8> = Vec::new();
    let mut i = 1;
    println!(
        "Using X^2 method for finding best single keys; difference in distribution per letter squared, summed. (lowest value is closest to expected distribution)"
    );
    for set in freq {
        print!("Finding best single key for set {}; ", i);
        let x = lf.get_best_x2_shift(set);
        println!("{}", from_int_lower(x));
        key.push(x);
        i += 1;
    }
    print!("Expected key; {}", int_to_str_lower(&mut key));
    println!();
    let mut cipher2 = str_upper_to_int(&mut cipher.chars());
    de_vig(&mut cipher2, &key);
    int_to_str_lower(&mut cipher2)
}

fn p2c() {
    println!("=====P2C=====");
    let min_keylength = 3;
    let max_keylength = 15;
    let cipher = get_cipher_text(3).to_owned();
    let kasiski_m = kasiski(&mut cipher.clone(), min_keylength, max_keylength);
    let freidman_1_m = freidman_1(&mut cipher.clone(), min_keylength, max_keylength);
    let freidman_2_m = freidman_2(&mut cipher.clone());

    println!("Kasiski key length guess: {}", kasiski_m);
    println!("Freidman 1 key length guess: {}", freidman_1_m);
    println!("Freidman 2 key length guess: {}", freidman_2_m);

    println!("Using kasiski's keylength guess");
    let solved = solve_vig(&mut cipher.clone(), kasiski_m);
    println!("Deciphered text;\n{}", solved);
}
#[derive(Clone)]
struct Matrix {
    //Matrix set with a 2d array datastructure, using ints not uints for ease of 
    //fixing negatives (rather than ensuring they don't happen ever)
    data: Vec<Vec<isize>>,
}
impl Matrix {
    //create a new matrix from a 1d array (for importing from string)
    fn new(v: Vec<isize>) -> Matrix {
        Matrix { data: v.into_iter().map(|x| vec![x]).collect() }
    }
    fn from_str(s: String, n: usize) -> Vec<Matrix> {
        //convert string to nx1 sized matricies for encode or decoding
        let mut result = Vec::new();
        let ints: Vec<isize> = str_upper_to_int(&mut s.chars())
            .iter()
            .map(|&v| v as isize)
            .collect();
        ints.chunks(n).for_each(
            |x| result.push(Matrix::new(x.to_vec())),
        );
        result
    }

    fn to_text_string(&self) -> String {
        //for using into a readible string
        self.data.iter().fold(String::new(), |s, ref x| {
            s +
                &x.iter().fold(String::new(), |ss, &xx| {
                    ss + &from_int_lower(xx as u8).to_string() + ""
                })
        })
    }

    fn mul_n(&mut self, m: isize) {
        //multiplies all fields by static amount m
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j] = modb(self.data[i][j] * m);
            }
        }
    }
    fn mul(&self, rhs: &Self) -> Self {
        //multiplies self by rhs, limits to 1 dimensional matricies as used in
        //hill cipher. Easily updatedable to support mxn
        let mut result = rhs.clone();
        for j in 0..result.data.len() {
            result.data[j] = vec![
                modb(self.data[j].iter().enumerate().fold(
                    0isize,
                    |sum, (i, &x)| {
                        sum + x * rhs.data[i][0]
                    },
                )),
            ];
        }
        result
    }

    fn to_data_string(&self) -> String {
        //to data string, to see underlying data of matricie
        self.data.iter().fold(String::new(), |s, ref x| {
            s + "| " +
                &x.iter().fold(
                    String::new(),
                    |ss, &xx| ss + &xx.to_string() + " ",
                ) + "|\n"
        })
    }
}


fn matricies_to_str(v: &Vec<Matrix>) -> String {
    //convert set of matricies to their human readible text
    v.iter().fold(String::new(), |s, x| s + &x.to_text_string())
}

fn p3a() -> Matrix {
    println!("=====P3A=====");
    let k = Matrix { data: vec![vec![2, 3], vec![9, 6]] };
    println!("K;\n{}", k.to_data_string());
    println!("Find inverse K");
    let adbc = k.data[0][0] * k.data[1][1] - k.data[0][1] * k.data[1][0];
    println!("Det(K)=ad-bc={}", adbc);
    println!("Find discrete (mod26) inverse Det(K)");
    let adbc_inv = modb_inv(adbc);
    println!("Inverse Det(K)={}", adbc_inv);
    let mut k_inv = Matrix {
        data: vec![
            vec![modb(k.data[1][1]), modb(-k.data[0][1])],
            vec![modb(-k.data[1][0]), modb(k.data[0][0])],
        ],
    };
    println!("Inverse K (without inverse Det(K);");
    println!("{}", k_inv.to_data_string());
    k_inv.mul_n(adbc_inv);
    println!("Inverse K;");
    println!("{}", k_inv.to_data_string());
    k_inv
}

fn p3b(k_inv: Matrix) {
    println!("=====P3B=====");
    let cipher = "HFFKXGMQSCRV".to_owned();
    let matricies = Matrix::from_str(cipher, 2);
    println!("As key is 2x2 matrix, splitting cipher into sets of 2 (chars in Z26);");
    println!("{}", matricies_to_str(&matricies).to_uppercase());
    matricies.iter().for_each(|m| print!("{:?}, ", m.data));
    println!();
    println!("Multiplying each set of 2 chars by inverse K...");
    println!("Combining resulting matricies...");
    println!("Converting back to string...");
    let f: Vec<Matrix> = matricies.iter().map(|ref m| k_inv.mul(*m)).collect();
    println!("Plaintext; {}", matricies_to_str(&f));
}

fn p1() {
    println!("\n#######P1#######");
    p1a();
    p1b();
    p1c();
}
fn p2() {
    println!("\n#######P2#######");
    p2a();
    p2b();
    p2c();
}
fn p3() {
    println!("\n#######P3#######");
    p3b(p3a());
}

fn main() {
    p1();
    p2();
    p3();
}
