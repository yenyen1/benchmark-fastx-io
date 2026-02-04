use benchmark_fastx_io::parallel::fastx_parallel::seq_io_parallel_fq_parse;
use benchmark_fastx_io::parallel::fastq_manu::manu_seqio;
use benchmark_fastx_io::parallel::fasta_manu::manu_needletail;
use benchmark_fastx_io::singular::fastq_parser::seq_io_parse;

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const SAMPLE_SIZE: usize = 30;
const LR_PATH_GZ: &str = "data/GM24385_1_subset.fastq.gz";
const SR_PATH_GZ: &str = "data/D1_S1_L001_R1_001_subset.fastq.gz";

fn bench_lr_gz_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("LR");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("seqio_single", |b| {
        b.iter(|| seq_io_parse(black_box(LR_PATH_GZ)))
    });
    group.bench_function("seqio_para1", |b| {
        b.iter(|| seq_io_parallel_fq_parse(black_box(LR_PATH_GZ), black_box(1)))
    });
    group.bench_function("seqio_para2", |b| {
        b.iter(|| seq_io_parallel_fq_parse(black_box(LR_PATH_GZ), black_box(2)))
    });
    group.bench_function("seqio_manu1", |b| {
        b.iter(|| manu_seqio(black_box(LR_PATH_GZ), black_box(1)))
    });
    group.bench_function("seqio_manu2", |b| {
        b.iter(|| manu_seqio(black_box(LR_PATH_GZ), black_box(2)))
    });
    group.bench_function("needletail_manu1", |b| {
        b.iter(|| manu_needletail(black_box(LR_PATH_GZ), black_box(1)))
    });
    group.bench_function("needletail_manu2", |b| {
        b.iter(|| manu_needletail(black_box(LR_PATH_GZ), black_box(2)))
    });
}


fn bench_sr_gz_parallel(c: &mut Criterion) {
    let mut group = c.benchmark_group("SR");
    group.sample_size(SAMPLE_SIZE);

    group.bench_function("seqio_single", |b| {
        b.iter(|| seq_io_parse(black_box(SR_PATH_GZ)))
    });
    group.bench_function("seqio_para1", |b| {
        b.iter(|| seq_io_parallel_fq_parse(black_box(SR_PATH_GZ), black_box(1)))
    });
    group.bench_function("seqio_para2", |b| {
        b.iter(|| seq_io_parallel_fq_parse(black_box(SR_PATH_GZ), black_box(2)))
    });
    group.bench_function("seqio_manu1", |b| {
        b.iter(|| manu_seqio(black_box(SR_PATH_GZ), black_box(1)))
    });
    group.bench_function("seqio_manu2", |b| {
        b.iter(|| manu_seqio(black_box(SR_PATH_GZ), black_box(2)))
    });
    group.bench_function("needletail_manu1", |b| {
        b.iter(|| manu_needletail(black_box(SR_PATH_GZ), black_box(1)))
    });
    group.bench_function("needletail_manu2", |b| {
        b.iter(|| manu_needletail(black_box(SR_PATH_GZ), black_box(2)))
    });
}

criterion_group!(benches, bench_sr_gz_parallel);
criterion_main!(benches);
