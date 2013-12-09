use std::rt::io;
use std::rt::io::buffered::BufferedReader;
use std::os;

use std::{str, iter};

struct Sudoku {
	r: [[u16, ..9], ..324],
	c: [[u16, ..4], ..729]
}

impl Sudoku {
	pub fn new() -> Sudoku {
		let mut s = Sudoku { r: [[0, ..9], ..324], c: [[0, ..4], ..729] };
		let mut nr = [0, ..324];
        let mut r = 0;
		for i in iter::range(0u16, 9u16) {
			for j in iter::range(0u16, 9u16) {
				for k in iter::range(0u16, 9u16) {
					s.c[r][0] = 9 * i + j;
					s.c[r][1] = (i/3*3 + j/3) * 9 + k + 81;
					s.c[r][2] = 9 * i + k + 162;
					s.c[r][3] = 9 * j + k + 243;
					r += 1;
				}
			}
		}
		for r in iter::range(0u16, 729u16) {
			for c2 in iter::range(0, 4) {
				let k = s.c[r][c2];
				s.r[k][nr[k]] = r;
				nr[k] += 1;
			}
		}
		return s;
	}
	#[inline(always)]
	fn forward(&self, sr: &mut [i8], sc: &mut [u8], c: u16, min: &mut u8, min_c: &mut u16) {
		for &rr in self.r[c].iter() {
			// Take a pointer to avoid repeated bounds checks
			let srrr = &mut sr[rr];
			*srrr += 1;
			if *srrr == 1 {
				for &cc in self.c[rr].iter() {
					// Take a pointer to avoid repeated bounds checks
					let sccc = &mut sc[cc];
					*sccc -= 1;
					if (*sccc < *min) {
						*min = *sccc; *min_c = cc;
					}
				}
			}
		}
	}
	#[inline(always)]
	fn revert(&self, sr: &mut [i8], sc: &mut [u8], c: u16) {
		for &rr in self.r[c].iter() {
			// Take a pointer to avoid repeated bounds checks
			let srrr = &mut sr[rr];
			*srrr -= 1;
			if *srrr == 0 {
				for &i in self.c[rr].iter() {
					sc[i] += 1;
				}
			}
		}
	}
	#[inline(always)]
	fn update(&self, sr: &mut [i8], sc: &mut [u8], r: u16, v: int) -> int {
		let mut min = 10;
        let mut min_c = 0;
		for &i in self.c[r].iter() {
			sc[i] += (v<<7) as u8;
		}
		for &c in self.c[r].iter() {
			if v > 0 { // move forward
				self.forward(sr, sc, c, &mut min, &mut min_c)
			} else {
				self.revert(sr, sc, c)
			}
		}
		return (min as int)<<16 | min_c as int;
	}
	pub fn solve(&self, inp: &str) -> ~[~str] {
		let mut sc = ~[9u8, ..324];
        let mut sr = ~[0i8, ..729];
		let mut cr = [-1i8, ..81]; 
        let mut cc = [-1i16, ..81];
		let mut s = [0, ..81];
        let mut s8 = [48u8, ..81];
		let mut hints = 0;
		for i in iter::range(0, 81) {
			let c = inp[i];// as char;
			s[i] = -1;
			if c >= '1' as u8 && c <= '9' as u8 {
				s[i] = (c - '1' as u8) as int;
				self.update(sr, sc, (i * 9 + s[i]) as u16, 1);
				hints += 1;
				s8[i] = c as u8;
			}
		}

		let mut ret: ~[~str] = ~[];
		let mut i = 0;
        let mut dir = 1;
        let mut cand: int = 10<<16|0;
		loop {
			while i >= 0 && i < 81 - hints {
				if dir == 1 {
					let mut min = (cand>>16) as u8;
					cc[i] = (cand & 0xffff) as i16;
					if min > 1 {
						for (c, &v) in sc.iter().enumerate() {
							if v < min {
								min = v; cc[i] = c as i16;
								if min <= 1 { break; }
							}
						}
					}
					if min == 0 || min == 10 {
						cr[i] = -1; dir = -1; i -= 1;
					}
				}
				let c = cc[i];
				if dir == -1 && cr[i] >= 0 {
					self.update(sr, sc, self.r[c][cr[i]], -1);
				}
				let mut tmp = 9i8;
				for r2 in iter::range(cr[i] + 1, 9) {
					if sr[self.r[c][r2]] == 0 {
						tmp = r2;
						break;
					}
				}
				if tmp < 9 {
					cand = self.update(sr, sc, self.r[c][tmp], 1);
					cr[i] = tmp; dir = 1; i += 1;
				} else {
					cr[i] = -1; dir = -1; i -= 1;
				}
			}
			if i < 0 { break; }
			for j in iter::range(0, i) {
				let r = self.r[cc[j]][cr[j]];
				s8[r/9] = (r%9 + '1' as u16) as u8;
			}
			ret.push(str::from_utf8(s8));
			i -= 1; dir = -1;
		}
		return ret;
	}
}

fn main() {
    use std::rt::io::Reader;
    use std::rt::io::native::stdio;
    use std::rt::io::mem::MemReader;

    let args = os::args();
    let use_default = args.len() == 1u;
    let rdr = if use_default {
        let foo = include_bin!("sudoku.txt");
        ~MemReader::new(foo.to_owned()) as ~Reader
    } else {
        ~stdio::stdin() as ~Reader
    };

    let mut rdr = BufferedReader::new(rdr);
    let s = Sudoku::new();

    loop {
        let line = match io::ignore_io_error(|| rdr.read_line()) {
            Some(ln) => ln, None => break,
        };
        let line = line.trim().to_owned();

        if line.len() == 0u { continue; }
         
        let r = s.solve(line);
        for l in r.iter() {
            println(*l);
        }
        println("");

    }
}
