extern crate rand;

use std::collections::VecDeque;
use rand::prelude::*;

#[derive(Debug, Clone)]
enum Fn {
    Add(i16),
    Mul(i16),
    Div(i16),
    Mod(i16)
}

impl Fn {
    fn overflow(a: i16) -> i16 {
        if a > 9999 {
            return 9999;
        }

        if a < -9999 {
            return -9999;
        }

        return a;
    }

    fn eval(&self, t: i16) -> i16 {
        match *self {
            Fn::Add(a) => Self::overflow(t.saturating_add(a)),
            Fn::Mul(a) => Self::overflow(t.saturating_mul(a)),
            Fn::Div(a) => Self::overflow(t / a),
            Fn::Mod(a) => Self::overflow(t % a)
        }
    }
}

#[derive(Debug, Clone)]
struct Fns {
    fns: Vec<Fn>
}

impl Fns {
    fn empty() -> Fns {
        Fns {
            fns: Vec::new()
        }
    }

    pub fn len(&self) -> usize {
        return self.fns.len();
    }

    pub fn append(&self, function: Fn) -> Fns {
        let mut new_fns = self.fns.clone();
        new_fns.push(function);
        Fns {
            fns: new_fns
        }
    }

    pub fn eval(&self, a: i16) -> i16 {
        let mut point = a;

        for step in self.fns.iter() {
            point = step.eval(point)
        }

        return point;
    }
}

#[derive(Debug, Clone)]
struct IntFns<'a> {
    options: &'a [Fn],
    option_len: usize,
    index: usize
}

impl<'a> IntFns<'a> {
    fn empty(options: &'a [Fn]) -> IntFns {
        Self::with_index(options, 0)
    }

    fn with_index(options: &'a [Fn], idx: usize) -> IntFns{
        IntFns {
            options: options,
            option_len: options.len(),
            index: idx
        }
    }

    pub fn steps(&self) -> Vec<Fn> {
        let mut vec = Vec::new();
        if self.index == 0 {
            return vec;
        }

        let mut index = self.index - 1;
        while index > 0 {
            vec.push(self.options[index % self.option_len].clone());
            index /= self.option_len;
        }

        vec.reverse();
        vec
    }

    pub fn eval(&self, a: i16) -> i16 {
        let mut point = a;
        let mut index = self.index - 1;

        while index > 0 {
            let step = &self.options[index % self.option_len];
            index = index / self.option_len;
            point = step.eval(point)
        }

        point
    }
}

struct Search<'a> {
    digits: &'a [i16],
    targets: &'a [i16],
    functions: Vec<Fn>
}

enum State<T> {
    Raw(T),
    Compressed(T)
}

impl<'a> Search<'a> {
    pub fn new(targets: &'a [i16]) -> Search<'a> {
        Search {
            digits: &[0,1,2,3,4,5,6,7,8,9],
            targets: targets,
            functions:  {
                let min = -9999;
                let max = 9999;
                let mut fns = Vec::new();
                for i in min..max {
                    fns.push(Fn::Add(i));
                }

                for i in min..max {
                    fns.push(Fn::Mul(i));
                }

                for i in min..max {
                    if i != 0 {
                        fns.push(Fn::Div(i));
                    }
                }

                for i in min..max {
                    if i != 0 {
                        fns.push(Fn::Mod(i));
                    }
                }

                fns
            }
        }
    }

    fn test(&self, fns: &Fns) -> bool {
        for digit in self.digits.iter() {
            if fns.eval(*digit) != self.targets[*digit as usize] {
                return false
            }
        }

        return true;
    }

    fn test_int(&self, fns: &IntFns) -> bool {
        for digit in self.digits.iter() {
            if fns.eval(*digit) != self.targets[*digit as usize] {
                return false
            }
        }

        return true;
    }

    fn start_idx(&self, max_entry: usize) {
        let mut rng = rand::thread_rng();

        for i in 0..max_entry {
            let fns = IntFns::with_index(&self.functions, i);
            // if i % 1 == 0 {
            //     println!("Testing: {:?}", fns.steps());
            // }

            if self.test_int(&fns) {
                println!("Found match! {:?}", fns.steps());
                // for digit in self.digits {
                //     println!("\t{} => {}", digit, fns.eval(*digit));
                // }
            }
        }
    }

    fn start(&self, depth: u16) {
        let mut rng = rand::thread_rng();

        let mut queue = VecDeque::new();
        let initial_fns = State::Raw(Fns::empty());
        queue.push_back(initial_fns);

        while !queue.is_empty() {
            match queue.pop_front().unwrap() {
                State::Raw(fns) => {
                    

                    queue.push_back(State::Compressed(fns));
                },
                State::Compressed(fns) => {
                    for addfn in self.functions.iter() {
                        let new_fns = fns.append(addfn.clone());
                        queue.push_front(State::Raw(new_fns));
                    }
                }
            }
        }
    }

    fn dfs(&self, fns: &Fns, depth: u16) {
        if depth >= 3 {
            return
        }

        for addfn in self.functions.iter() {
            let new_fns = fns.append(addfn.clone());
            // println!("Testing: {:?}", new_fns);
            if self.test(&new_fns) {
                println!("Found match! {:?}", new_fns);
            }
            self.dfs(&new_fns, depth + 1)
        }
    }
}

fn main() {
    let targets: Vec<i16> = (0..10i16)
        .map(|e| 2*e)
        .map(|e| if e > 9 {e-9} else {e})
    .collect();
    println!("Targets: {:?}", targets);
    // let targets = [0, 2, 4, 6, 8, 1, 316, 5i16, 7i16, 9i16];
    // let targets = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let search = Search::new(targets.as_slice());
    search.start_idx(std::usize::MAX);
}
