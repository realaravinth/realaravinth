#![feature(test)]
#![allow(dead_code)]

extern crate test;

const SIZE: usize = 1000;

fn str() -> &'static str {
    "test"
}

fn string() -> String {
    String::from("test")
}

fn create_stack_arr() -> [usize; SIZE] {
    let a: [usize; SIZE] = [0; SIZE];
    a
}

fn stack_arr() -> [usize; SIZE] {
    create_stack_arr()
}

fn heap_arr() -> Vec<usize> {
    ceate_heap_arr()
}

fn ceate_heap_arr() -> Vec<usize> {
    Vec::with_capacity(SIZE)
}

#[cfg(test)]
mod tests {
    use test::{black_box, Bencher};

    use super::*;

    #[bench]
    fn bench_str(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..100 {
                black_box(str());
            }
        });
    }

    #[bench]
    fn bench_string(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..100 {
                black_box(string());
            }
        });
    }

    #[bench]
    fn bench_heap_arr(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..100 {
                black_box(heap_arr());
            }
        });
    }

    #[bench]
    fn bench_stack_arr(b: &mut Bencher) {
        b.iter(|| {
            for _ in 1..100 {
                black_box(stack_arr());
            }
        });
    }
}
