use crate::utils::dna::{NCount, merge_nc_count};
use crate::utils::utils::open_bufreader;

use bio::io::fastq as bio_fq;
use fastq::{self, Record as fastq_record};
use fxread::initialize_reader as fxread_init_reader;
use kseq::parse_path as kseq_parse_path;
use needletail::parse_fastx_file as needletail_init_reader;
use noodles_fastq::io as noodle_fq;
use seq_io::fastq::{self as seq_io_fq, Record as seq_io_record};

pub fn bio_parse(path: &str) -> std::io::Result<NCount> {
    let mut nc_count = NCount::new();
    let reader = open_bufreader(path).map(bio_fq::Reader::new)?;

    for record in reader.records() {
        let result = record.unwrap();
        result.seq().iter().for_each(|c| nc_count.update(c));
    }
    Ok(nc_count)
}

pub fn noodles_parse(path: &str) -> std::io::Result<NCount> {
    let mut reader = open_bufreader(path).map(noodle_fq::Reader::new)?;
    let mut nc_count = NCount::new();
    reader.records().for_each(|record| {
        let result = record.unwrap();
        result.sequence().iter().for_each(|c| nc_count.update(c));
    });
    Ok(nc_count)
}

/// Fastq parser failed to read Nanopore dataset since read is too long. (buffersize not enough)
/// It does not validate that the sequence or the quality contain only allowed characters.
/// multithreaded setup not support paired-end reads.
///
/// fastq::parse_path can detect gzip, lz4 or plain FASTQ files. <- It seems automatically using multi-thread.
/// It is more stable than and better performance than detect file type with file path (utils::open_bufreader).
pub fn fastq_parse(path: &str) -> std::io::Result<NCount> {
    let mut nc_count = NCount::new();
    fastq::parse_path(Some(path), |reader| {
        reader
            .each(|record| {
                record.seq().iter().for_each(|c| nc_count.update(c));
                true
            })
            .expect("fastq parse path");
    })?;
    Ok(nc_count)
}

pub fn fastq_parallel_parse(path: &str, n_threads: usize) -> std::io::Result<NCount> {
    fastq::parse_path(Some(path), |reader| {
        let mut results: Vec<NCount> = reader
            .parallel_each(n_threads, |record_sets| {
                let mut nc_count = NCount::new();
                for records in record_sets {
                    records.iter().for_each(|record| {
                        record.seq().iter().for_each(|c| nc_count.update(c));
                    });
                }
                nc_count
            })
            .expect("fastq parallel_each failed...");
        merge_nc_count(&mut results).expect("fastq parallel_each merge result failed")
    })
}

pub fn seq_io_parse(path: &str) -> std::io::Result<NCount> {
    let mut nc_count = NCount::new();
    let mut reader = open_bufreader(path).map(seq_io_fq::Reader::new)?;
    while let Some(record) = reader.next() {
        let record = record.unwrap();
        record.seq().iter().for_each(|c| nc_count.update(c));
    }
    Ok(nc_count)
}

pub fn fxread_parse(path: &str) -> std::io::Result<NCount> {
    let mut nc_count = NCount::new();
    let reader = fxread_init_reader(path).unwrap();
    reader.for_each(|record| {
        record.seq().iter().for_each(|c| nc_count.update(c));
    });
    Ok(nc_count)
}

pub fn needletail_parse(path: &str) -> std::io::Result<NCount> {
    let mut nc_count = NCount::new();
    let mut reader = needletail_init_reader(path).unwrap();
    while let Some(record) = reader.next() {
        let record = record.unwrap();
        record.seq().iter().for_each(|c| nc_count.update(c));
    }
    Ok(nc_count)
}

pub fn kseq_parse(path: &str) -> std::io::Result<NCount> {
    let mut nc_count = NCount::new();
    let mut reader = kseq_parse_path(path).unwrap();
    while let Some(record) = reader.iter_record().unwrap() {
        record
            .seq()
            .as_bytes()
            .iter()
            .for_each(|c| nc_count.update(c));
    }
    Ok(nc_count)
}
