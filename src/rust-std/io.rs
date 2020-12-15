use std::io::prelude::*;
use std::io::BufWriter;
use std::net::TcpStream;
use criterion::{criterion_group, criterion_main, Criterion};

fn tcp_write() {
    let mut stream = TcpStream::connect("127.0.0.1:1080").unwrap();

    for i in 0..10 {
        stream.write(&[i+1]).unwrap();
    }
}

fn tcp_write_bufwriter() {
    let mut stream = BufWriter::new(TcpStream::connect("127.0.0.1:1080").unwrap());

    for i in 0..10 {
        stream.write(&[i+1]).unwrap();
    }
    stream.flush().unwrap();
}

fn bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("IO Write");

    g.bench_function("Writer", |b| b.iter(|| tcp_write()));

    g.bench_function("BufWriter", |b| b.iter(|| tcp_write_bufwriter()));
}

criterion_group!(benches, bench);
criterion_main!(benches);
