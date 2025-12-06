#[cfg(test)]

mod tests {

    use test_bioinformatics_io::fastq_parser::{
        bio_parse, fastq_parallel_parse, fastq_parse, fxread_parse, noodles_parse,
        seq_io_parallel_parse, seq_io_parse, needletail_parse,
    };

    const LR_PATH: &str = "tests/data/GM24385_1_subset_100.fastq";
    const LR_PATH_GZ: &str = "tests/data/GM24385_1_subset_100.fastq.gz";
    const SR_PATH: &str = "tests/data/D1_S1_L001_R1_001_subset_100.fastq";
    const SR_PATH_GZ: &str = "tests/data/D1_S1_L001_R1_001_subset_100.fastq.gz";
    const LR_COUNT: [u64; 5] = [230450, 142423, 157285, 234587, 0];
    const SR_COUNT: [u64; 5] = [6959, 5384, 5382, 7134, 58];

    #[test]
    fn test_bio_parse() {
        let nc_count = bio_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = bio_parse(LR_PATH_GZ);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = bio_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = bio_parse(SR_PATH_GZ);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

    #[test]
    fn test_noodles_parse() {
        let nc_count = noodles_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = noodles_parse(LR_PATH_GZ);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = noodles_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = noodles_parse(SR_PATH_GZ);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

    #[test]
    fn test_fastq_parse() {
        let nc_count = fastq_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = fastq_parse(SR_PATH_GZ);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        // let result = fastq_parse(LR_PATH);
        // fastq parse path: Custom { kind: InvalidData, error: "Fastq record is too long" }

        let nc_count = fastq_parallel_parse(SR_PATH, 4);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = fastq_parallel_parse(SR_PATH_GZ, 4);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        // let result = fastq_parallel_parse(LR_PATH, 4);
        // fastq parse path: Custom { kind: InvalidData, error: "Fastq record is too long" }
    }

    #[test]
    fn test_seq_io_parse() {
        let nc_count = seq_io_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parse(LR_PATH_GZ);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parse(SR_PATH_GZ);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());

        let nc_count = seq_io_parallel_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parallel_parse(LR_PATH_GZ);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parallel_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = seq_io_parallel_parse(SR_PATH_GZ);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

    #[test]
    fn test_fxread_parse() {
        let nc_count = fxread_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = fxread_parse(LR_PATH_GZ);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = fxread_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = fxread_parse(SR_PATH_GZ);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

    #[test]
    fn test_needetail_parse() {
        let nc_count = needletail_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = needletail_parse(LR_PATH_GZ);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = needletail_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = needletail_parse(SR_PATH_GZ);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }
}
