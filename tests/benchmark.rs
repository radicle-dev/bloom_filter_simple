use bloom_filter_simple::{BloomFilter, DefaultBloomFilter, SeededBloomFilter};
use std::time::Instant;

#[test]
#[ignore]
fn measure_inserting_km() {
    let mut element_count = 2;
    for _ in 0..23 {
        inserting_km(element_count);
        element_count *= 2;
    }
}

fn inserting_km(element_count: usize) {
    let mut bloom_filter = DefaultBloomFilter::new(element_count, 0.001);
    let start = Instant::now();
    for i in 0..element_count {
        bloom_filter.insert(&i);
    }
    println!(
        "Inserting {} elements into KMBloomFilter took {:?}, or {:?} per element",
        element_count,
        start.elapsed(),
        start.elapsed() / element_count as u32
    );
}

#[test]
#[ignore]
fn measure_inserting_seeded() {
    let mut element_count = 2;
    for _ in 0..23 {
        inserting_seeded(element_count);
        element_count *= 2;
    }
}

fn inserting_seeded(element_count: usize) {
    let mut bloom_filter = SeededBloomFilter::new(element_count, 0.001);
    let start = Instant::now();
    for i in 0..element_count {
        bloom_filter.insert(&i);
    }
    println!(
        "Inserting {} elements into SeededBloomFilter took {:?}, or {:?} per element",
        element_count,
        start.elapsed(),
        start.elapsed() / element_count as u32
    );
}

#[test]
#[ignore]
fn measure_checking_km() {
    let mut element_count = 2;
    for _ in 0..23 {
        checking_km(element_count);
        element_count *= 2;
    }
}

fn checking_km(element_count: usize) {
    let mut bloom_filter = DefaultBloomFilter::new(element_count, 0.001);
    for i in 0..element_count {
        bloom_filter.insert(&i);
    }
    let start = Instant::now();
    for i in 0..element_count {
        bloom_filter.contains(&i);
    }
    println!(
        "Checking {} elements in KMBloomFilter took {:?}, or {:?} per element",
        element_count,
        start.elapsed(),
        start.elapsed() / element_count as u32
    );
}

#[test]
#[ignore]
fn measure_checking_seeded() {
    let mut element_count = 2;
    for _ in 0..23 {
        checking_seeded(element_count);
        element_count *= 2;
    }
}

fn checking_seeded(element_count: usize) {
    let mut bloom_filter = SeededBloomFilter::new(element_count, 0.001);
    for i in 0..element_count {
        bloom_filter.insert(&i);
    }
    let start = Instant::now();
    for i in 0..element_count {
        bloom_filter.contains(&i);
    }
    println!(
        "Checking {} elements in SeededBloomFilter took {:?}, or {:?} per element",
        element_count,
        start.elapsed(),
        start.elapsed() / element_count as u32
    );
}
