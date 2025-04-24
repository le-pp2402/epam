use std::str;
use std::io;


fn main() {
    let mut scanner = UnsafeScanner::new(io::stdin().lock());
    
    let mut tc = scanner.token::<i32>();

    loop {
        let p: Vec<char> = scanner.token::<String>().chars().collect();
        let s: Vec<char> = scanner.token::<String>().chars().collect();

        let cp = compress(&p);
        let cs = compress(&s);

        let mut chk: bool = true;

        if cp.len() != cs.len() {
            chk = false;
        } else {
            for i in 0..(cp.len()) {
                if (cp[i].first == cs[i].first && cp[i].second * 2 >= cs[i].second && cs[i].second >= cp[i].second) == false {
                    chk = false;
                } 
            }
        }


        if chk == true {
            println!("YES");
        } else {
            println!("NO");
        }

        tc -= 1;
        if tc == 0 {
            break;
        }
    }

}



pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}


impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }
 
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute(slice.split_ascii_whitespace())
            }
        }
    }
}

