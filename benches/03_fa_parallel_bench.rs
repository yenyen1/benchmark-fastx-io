use benchmark_fastx_io::parallel::fasta_parallel::seq_io_parallel_parse;
use benchmark_fastx_io::parallel::fasta_manu::{manu_seqio, manu_needletail};
use benchmark_fastx_io::singular::fasta_parser::seq_io_parse;

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const SAMPLE_SIZE: usize = 20;
const LR_PATH_GZ: &str = "data/GM24385_1_subset.fasta.gz";
const SR_PATH_GZ: &str = "data/D1_S1_L001_R1_001_subset.fasta.gz";

fn bench_gz_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("seqio_para");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("lr", |b| {
        b.iter(|| seq_io_parse(black_box(LR_PATH_GZ)))
    });
    group.bench_function("lr_seqio_t2", |b| {
        b.iter(|| seq_io_parallel_parse(black_box(LR_PATH_GZ), black_box(2)))
    });
    group.bench_function("lr_seqio_m2", |b| {
        b.iter(|| manu_seqio(black_box(LR_PATH_GZ), black_box(2)))
    });
    group.bench_function("lr_needletail_m2", |b| {
        b.iter(|| manu_needletail(black_box(LR_PATH_GZ), black_box(2)))
    });

    group.bench_function("sr", |b| {
        b.iter(|| seq_io_parse(black_box(SR_PATH_GZ)))
    });
    group.bench_function("sr_seqio_t2", |b| {
        b.iter(|| seq_io_parallel_parse(black_box(SR_PATH_GZ), black_box(2)))
    });
    group.bench_function("sr_seqio_m2", |b| {
        b.iter(|| manu_seqio(black_box(SR_PATH_GZ), black_box(2)))
    });
    group.bench_function("sr_needletail_m2", |b| {
        b.iter(|| seq_io_parallel_parse(black_box(SR_PATH_GZ), black_box(2)))
    });

    group.finish();
}

criterion_group!(benches, bench_gz_parallel,);
criterion_main!(benches);
