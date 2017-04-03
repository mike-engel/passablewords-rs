#![feature(test)]
extern crate test;
extern crate passablewords;

use passablewords::{check_entropy, check_length, check_password, check_uniqueness};

#[bench]
fn bench_check_short_password(b: &mut test::Bencher) {
    b.iter(|| check_length("short"));
}

#[bench]
fn bench_check_common_password(b: &mut test::Bencher) {
    b.iter(|| check_uniqueness("password"));
}

#[bench]
fn bench_check_simple_password(b: &mut test::Bencher) {
    b.iter(|| check_entropy("Not Too Random"));
}

#[bench]
fn bench_check_ok_password(b: &mut test::Bencher) {
    b.iter(|| check_password("Th1s iS a Sup3rR4ndom PassW0rd!"));
}
