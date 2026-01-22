use crate::utils::utils::open_bufreader;
use crate::utils::dna::{NCount, get_actgn_idx_from_ascii};

use seq_io::fasta as seq_io_fa;
use seq_io::parallel as seq_io_parallel;


pub fn seq_io_parallel_parse(path: &str, n_threads: usize) -> std::io::Result<NCount> {
    let reader = open_bufreader(path).map(seq_io_fa::Reader::new)?;
    let mut nc_count = NCount::new();
    let queue_len = n_threads*2;

    seq_io_parallel::parallel_fasta(
        reader,
        n_threads as u32,
        queue_len,
        |record, found: &mut [u64; 5]| {
            *found = [0, 0, 0, 0, 0];
            record.seq_lines().for_each(|l| {
                l.iter()
                    .for_each(|c| found[get_actgn_idx_from_ascii(c)] += 1)
            });
        },
        |_, found| {
            nc_count.add_by_nc_arr(found);
            None::<()>
        },
    )
    .unwrap();
    Ok(nc_count)
}
