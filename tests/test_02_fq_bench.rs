#[cfg(test)]

mod tests {

    use benchmark_fastx_io::singular::fastq_parser::{
        bio_parse, fastq_parallel_parse, fastq_parse, fxread_parse, kseq_parse, needletail_parse,
        noodles_parse, seq_io_parse,
    };

    const LR_PATH: &str = "tests/data/GM24385_1_subset_100.fastq";
    const LR_PATH_GZ: &str = "tests/data/GM24385_1_subset_100.fastq.gz";
    const SR_PATH: &str = "tests/data/D1_S1_L001_R1_001_subset_100.fastq";
    const SR_PATH_GZ: &str = "tests/data/D1_S1_L001_R1_001_subset_100.fastq.gz";
    const FOFN_PATH: &str = "tests/data/fq_fofn.fofn";
    const LR_COUNT: [u64; 5] = [230450, 142423, 157285, 234587, 0];
    const SR_COUNT: [u64; 5] = [6959, 5384, 5382, 7134, 58];
    const FOFN_COUNT: [u64; 5] = [237409, 147807, 162667, 241721, 58];

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

    #[test]
    fn test_kseq_parse() {
        let nc_count = kseq_parse(LR_PATH);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = kseq_parse(LR_PATH_GZ);
        assert_eq!(&LR_COUNT, nc_count.unwrap().get());
        let nc_count = kseq_parse(SR_PATH);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
        let nc_count = kseq_parse(SR_PATH_GZ);
        assert_eq!(&SR_COUNT, nc_count.unwrap().get());
    }

    #[test]
    fn test_fofn_parse() {
        // not available: bio, noodles, fastq, seq_io, fxread, needletail
        let nc_count = kseq_parse(FOFN_PATH);
        assert_eq!(&FOFN_COUNT, nc_count.unwrap().get());
    }
}
