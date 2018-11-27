
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
    modb_n(x, 2)
}

#[derive(Clone)]
struct Matrix {
    //Matrix set with a 2d array datastructure, using ints not uints for ease of
    //fixing negatives (rather than ensuring they don't happen ever)
    data: Vec<Vec<isize>>,
}
impl Matrix {
    //create a new matrix from a 1d array (for importing from string)
    fn new() -> Matrix {
        Matrix { data: Vec::new() }
    }

    fn push(&mut self, v: Vec<u8>) {
        self.data.push(v.into_iter().map(|x| x as isize).collect());
    }

    fn to_text_string(&self) -> String {
        //for using into a readible string
        self.data.iter().fold(String::new(), |s, x| {
            s + "\n" + &x.iter().map(|i| i.to_string()).collect::<String>()
        })
        //   self.data
        // .iter()
        // .map(|x| {
        //     (x.iter().map(|i| i.to_string()).collect::<String>()).push('\n')
        // })
        // .collect::<String>()
    }

    fn mul_n(&mut self, m: isize) {
        //multiplies all fields by static amount m
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                self.data[i][j] = modb(self.data[i][j] * m);
            }
        }
    }
    fn lhs_mul(&self, lhs: &Vec<u8>) -> Vec<u8> {
        let mut result = lhs.clone();
        for i in 0..lhs.len() as usize {
            result[i] = 0;
            for j in 0..lhs.len() as usize {
                result[i] = result[i] + lhs[j] * self.data[i][j] as u8
            }
            result[i] = result[i] % 2;
        }
        result
    }

    fn mul(&self, rhs: &Self) -> Self {
        //multiplies self by rhs, limits to 1 dimensional matricies as used in
        //hill cipher. Easily updatedable to support mxn
        let mut result = rhs.clone();
        for j in 0..result.data.len() {
            result.data[j] = vec![
                modb(
                    self.data[j]
                        .iter()
                        .enumerate()
                        .fold(0isize, |sum, (i, &x)| sum + x * rhs.data[i][0]),
                ),
            ];
        }
        result
    }

    fn to_data_string(&self) -> String {
        //to data string, to see underlying data of matricie
        self.data.iter().fold(String::new(), |s, ref x| {
            s + "| "
                + &x.iter()
                    .fold(String::new(), |ss, &xx| ss + &xx.to_string() + " ") + "|\n"
        })
    }
}


struct BlumBlumShub {
    n: usize,
    s: usize,
}

impl BlumBlumShub {
    fn new(p: usize, q: usize, s: usize) -> BlumBlumShub {
        BlumBlumShub { n: p * q, s: s }
    }

    fn next(&mut self) -> usize {
        self.s = (self.s * self.s) % self.n;
        self.s
    }
}

fn test_bbs() {
    let mut bbs = BlumBlumShub::new(7, 19, 2);
    for _i in 0..7 {
        let x = bbs.next();
        println!("{}, {}", x, x % 2);
    }
}

fn str_to_ints(s: &String) -> Vec<u8> {
    s.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}
fn ints_to_str(v: Vec<u8>) -> String {
    v.into_iter().map(|i| i.to_string()).collect()
}

fn solve_lfsr(plaintext: String, ciphertext: String, m: u8) -> String {
    let plain = str_to_ints(&plaintext);
    let cipher = str_to_ints(&ciphertext);
    let mut keystream: Vec<u8> = Vec::new();
    println!("Calculate l (keystream);");
    println!("XOR'ing P and C for L");
    for i in 0..plain.len() as usize {
        keystream.push((plain[i] + cipher[i]) % 2)
    }
    println!(
        "P: {}\nC: {}\nL: {}",
        plaintext,
        ciphertext,
        ints_to_str(keystream.clone())
    );
    println!("Create Matrix of;");
    println!("|L1 L2 ... Lm-1 Lm  |");
    println!("|L2 L3 ... Lm-2 Lm-1|");
    for _ in 0..3 {
        println!("|.                 .|")
    }
    println!("|Lm Lm+1 ...    Lm+m|");
    let mut matrix = Matrix::new();
    for i in 0..(m as usize) {
        matrix.push(keystream[i..(i + m as usize)].to_vec());
    }
    println!("{}", matrix.to_text_string());
    let input = keystream[m as usize..(2usize * m as usize)].to_vec();
    let consts = matrix.lhs_mul(&input);
    println!(
        "Lm - l2m: |{}|\nResult of multiplication;\n{}",
        ints_to_str(input),
        ints_to_str(consts)
    );
    ints_to_str(keystream)
}

fn p1() {
    println!("\n#######P1#######");
    let plaintext = "10110111010011";
    let ciphertext = "11011101100010";
    let m = 6;
    //solve_lfsr(plaintext.to_owned(), ciphertext.to_owned(), m);
    solve_lfsr(plaintext.to_owned(), ciphertext.to_owned(), m);
}
fn p2() {
    println!("\n#######P2#######");
    //let c = vec![1, 1, 0, 1, 0, 1];
    let c = vec![1, 0, 1, 1, 0, 1];
    let mut values = vec![1, 0, 1, 1, 1, 0];
    for i in 0..10000 {
        let mut temp = 0;
        for j in 0..(c.len()) {
            temp += modb(values[i + j] * c[j]);
        }
        values.push(modb(temp));
    }
    for i in 0..10 {
        println!("{}", i);
    }
    let end = values.len() - 1;
    let mut foundPeriod = false;
    let mut period = 4;
    while !foundPeriod {
        println!("Trying period {}", period);
        foundPeriod = true;
        for i in 0..period {
            if values[end - i] != values[end - period - i]
                || values[end - i] != values[end - (2 * period) - i]
                || values[end - i] != values[end - (3 * period) - i]
            {
                foundPeriod = false;
                period += 1;
                break;
            }
        }
    }
    println!("Found period {}", period);
    for i in 1..5000 {
        if i % period == 0 {
            println!("{}", values[i - 1]);
        } else {
            print!("{}", values[i - 1]);
        }
    }

    //println!("{:?}", values);
}

fn p3() {
    println!("\n#######P3#######");
}
fn p4() {
    println!("\n#######P4#######");
}
fn p5() {
    println!("\n#######P5#######");
}

fn main() {
    test_bbs();
    p1();
    p2();
    p3();
    p4();
    p5();
}
