// ASCII to ACGTN
// A|a: 0 ; C|c:1 ; G|g:2 ; T|t:3 ; other: 4
const ASCII_TO_ACGTN_IDX: [usize; 256] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 0, 4, 1, 4, 4, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 0, 4, 1, 4, 4, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
];

pub fn get_actgn_idx_from_ascii(c: &u8) -> usize {
    ASCII_TO_ACGTN_IDX[*c as usize]
}

#[derive(Default)]
pub struct NCount {
    data: [u64; 5],
}

impl NCount {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn update(&mut self, c: &u8) {
        self.data[ASCII_TO_ACGTN_IDX[*c as usize]] += 1;
    }
    // pub fn add(&mut self, c: &u8, count: u64) {
    //     self.data[ASCII_TO_ACGTN_IDX[*c as usize]] += count;
    // }
    pub fn get(&mut self) -> &[u64; 5] {
        &self.data
    }
    pub fn get_from_idx(&self, idx: usize) -> u64 {
        self.data[idx]
    }
    pub fn merge(&mut self, nc_count: &NCount) {
        for i in 0..self.data.len() {
            self.data[i] += nc_count.get_from_idx(i);
        }
    }
    pub fn add_by_nc_arr(&mut self, nc_arr: &[u64; 5]) {
        for i in 0..self.data.len() {
            self.data[i] += nc_arr[i]
        }
    }
    pub fn print(&mut self) {
        // A|a: 0 ; C|c:1 ; G|g:2 ; T|t:3 ; other: 4
        let [a, c, g, t, n] = self.data;
        println!("A: {}; C: {}; G: {}; T: {}; N: {}", a, c, g, t, n);
    }
}

pub fn merge_nc_count(counts: &mut Vec<NCount>) -> Option<NCount> {
    if counts.is_empty() {
        return None;
    }
    if counts.len() == 1 {
        return counts.pop();
    }
    let mut merge_count = NCount::new();
    counts.iter().for_each(|c| merge_count.merge(c));
    Some(merge_count)
}