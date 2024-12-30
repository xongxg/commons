macro_rules! count_exprs {
    () => {0};
    ($head:expr) =>{1};
    ($head:expr,$($tail:expr),*)=>{1+count_exprs!($($tail),*)}
}

macro_rules! reccurence1 {
    ($seq:ident[$ind:ident] : $sty:ty= $($inits:expr),+;...;$recur:expr) => {{

        const MEM_SIZE:usize = count_exprs!($($inits),+);
        struct Recurrence {
            mem: [$sty; MEM_SIZE],
            pos: usize,
        }

        impl Iterator for Recurrence {
            type Item = $sty;

            fn next(&mut self) -> Option<Self::Item> {
                if self.pos < MEM_SIZE {
                    let next_val = self.mem[self.pos];
                    self.pos += 1;
                    Some(next_val)
                } else {
                    let next_val = {
                        let $ind = self.pos;
                        let $seq = IndexOffset {
                            slice: &self.mem,
                            offset: $ind,
                        };

                        $recur
                    };

                    {
                        use std::mem::swap;
                        let mut swap_tmp = next_val;
                        for i in (0..MEM_SIZE).rev() {
                            swap(&mut self.mem[i], &mut swap_tmp);
                        }
                    }
                    self.pos += 1;

                    Some(next_val)
                }
            }
        }

        struct IndexOffset<'a> {
            slice: &'a [$sty; MEM_SIZE],
            offset: usize,
        }

        use std::ops::Index;

        impl<'a> Index<usize> for IndexOffset<'a> {
            type Output = $sty;

            fn index<'b>(&'b self, index: usize) -> &'b Self::Output {
                use std::num::Wrapping;

                let index = Wrapping(index);
                let offset = Wrapping(self.offset);
                let windows = Wrapping(MEM_SIZE);

                let read_index = index - offset + windows;
                &self.slice[read_index.0]
            }
        }

        Recurrence {
            mem: [$($inits),+],
            pos: 0,
        }
    }};
}

fn main() {
    println!("fib add ...................");
    let fib = reccurence1![a[n]:u64= 0,1;...;a[n-2]+a[n-1]];
    for e in fib.take(10) {
        println!("{}", e);
    }

    println!();

    println!("fib mult ...................");
    let fib = reccurence1![f[i]: f64 = 1.0; ...; f[i-1] * i as f64];
    for e in fib.take(10) {
        println!("{}", e);
    }
}
