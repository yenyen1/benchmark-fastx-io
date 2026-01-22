#[cfg(test)]

mod tests {

    use benchmark_fastx_io::parallel::fasta_parallel::seq_io_parallel_parse;
    use benchmark_fastx_io::parallel::fasta_manu::{manu_seqio, manu_needletail};

    const LR_PATH: &str = "tests/data/GM24385_1_subset_100.fasta";
    const LR_PATH_GZ: &str = "tests/data/GM24385_1_subset_100.fasta.gz";
    const SR_PATH: &str = "tests/data/D1_S1_L001_R1_001_subset_100.fasta";
    const SR_PATH_GZ: &str = "tests/data/D1_S1_L001_R1_001_subset_100.fasta.gz";
    const MULTILINE_PATH: &str = "tests/data/multiline.fasta";
    const LR_COUNT: [u64; 5] = [230450, 142423, 157285, 234587, 0];
    const SR_COUNT: [u64; 5] = [6959, 5384, 5382, 7134, 58];
    const ML_COUNT: [u64; 5] = [5, 5, 5, 5, 5];

    #[test]
    fn test_seq_io_para() {
        let nc_count = seq_io_parallel_parse(LR_PATH, 2);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parallel_parse(LR_PATH_GZ, 2);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parallel_parse(SR_PATH, 2);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parallel_parse(SR_PATH_GZ, 2);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        
    }

    #[test]
    fn test_seq_io_manu() {
        let nc_count = manu_seqio(LR_PATH, 1);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = manu_seqio(LR_PATH_GZ, 1);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = manu_seqio(SR_PATH, 1);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = manu_seqio(SR_PATH_GZ, 1);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());

        let nc_count = manu_seqio(LR_PATH, 2);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = manu_seqio(LR_PATH_GZ, 2);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = manu_seqio(SR_PATH, 2);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = manu_seqio(SR_PATH_GZ, 2);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

    #[test]
    fn test_needetail_manu() {
        let nc_count = manu_needletail(LR_PATH, 1);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = manu_needletail(LR_PATH_GZ, 1);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = manu_needletail(SR_PATH, 1);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = manu_needletail(SR_PATH_GZ, 1);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());

        let nc_count = manu_needletail(LR_PATH, 2);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = manu_needletail(LR_PATH_GZ, 2);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = manu_needletail(SR_PATH, 2);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = manu_needletail(SR_PATH_GZ, 2);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

    #[test]
    fn test_fasta_multiline_para() {
        let nc_count = seq_io_parallel_parse(MULTILINE_PATH, 4);
        assert_eq!(&ML_COUNT, nc_count.unwrap().get());
        let nc_count = manu_seqio(MULTILINE_PATH, 4);
        assert_eq!(&ML_COUNT, nc_count.unwrap().get());
        let nc_count = manu_needletail(MULTILINE_PATH, 4);
        assert_eq!(&ML_COUNT, nc_count.unwrap().get());
    }
}
