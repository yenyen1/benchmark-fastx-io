pub mod parallel;
pub mod singular;
mod utils;

pub fn run() {
    parallel::fasta_manu::manu_seqio("/Users/yenyenwang/Documents/rust/test-fastx-parser/data/GM24385_1_subset.fasta.gz", 2).expect("failed run");
}
