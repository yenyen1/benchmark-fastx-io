use crate::utils::utils::open_bufreader;
use crate::utils::dna::NCount;

use bio::io::fasta as bio_fa;
use noodles_fasta::io as noodle_fa;
use seq_io::fasta as seq_io_fa;
use std::io;

pub fn noodles_parse(path: &str) -> io::Result<NCount> {
    let mut reader = open_bufreader(path).map(noodle_fa::Reader::new)?;
    let mut nc_count = NCount::new();

    for record in reader.records() {
        let result = record?;
        let seq = result.sequence();
        seq.as_ref().iter().for_each(|e| nc_count.update(e));
    }
    Ok(nc_count)
}

pub fn bio_parse(path: &str) -> io::Result<NCount> {
    let reader = open_bufreader(path).map(bio_fa::Reader::new)?;
    let mut nc_count = NCount::new();
    for record in reader.records() {
        let result = record?;
        let seq = result.seq();
        seq.iter().for_each(|e| nc_count.update(e));
    }
    Ok(nc_count)
}

pub fn seq_io_parse(path: &str) -> io::Result<NCount> {
    let mut nc_count = NCount::new();
    let mut reader = open_bufreader(path).map(seq_io_fa::Reader::new)?;
    while let Some(record) = reader.next() {
        let record = record.expect("Seq_io Error reading record");
        record
            .seq_lines()
            .for_each(|s| s.iter().for_each(|c| nc_count.update(c)));
    }
    Ok(nc_count)
}
