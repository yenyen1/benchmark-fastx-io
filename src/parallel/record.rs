const ID_BYTE_LIMIT: usize = 1024 * 1024;
const SEQ_BYTE_LIMIT: usize = 4 * 1024 * 1024;
const RECORDSET_SIZE_LIMIT: usize = 16 * 1024;

pub struct OwnedRecordSet {
    ids: Vec<u8>,
    seqs: Vec<u8>,
    quals: Vec<u8>,
    meta: Vec<(usize, usize, usize)>, // (id_len, seq_len, qual_len)
}

impl OwnedRecordSet {
    pub fn new() -> Self {
        Self {
            ids: Vec::with_capacity(ID_BYTE_LIMIT),
            seqs: Vec::with_capacity(SEQ_BYTE_LIMIT),
            quals: Vec::with_capacity(SEQ_BYTE_LIMIT),
            meta: Vec::with_capacity(RECORDSET_SIZE_LIMIT),
        }
    }
    pub fn clear(&mut self) {
        self.ids.clear();
        self.seqs.clear();
        self.quals.clear();
        self.meta.clear();
    }
    pub fn push(&mut self, id: &[u8], seq: &[u8], qual: &[u8]) {
        self.ids.extend_from_slice(id);
        self.seqs.extend_from_slice(seq);
        self.quals.extend_from_slice(qual);
        self.meta.push((id.len(), seq.len(), qual.len()));
    }
    pub fn is_overload(&self, cur_seq_size: usize) -> bool {
        self.meta.len() >= RECORDSET_SIZE_LIMIT || self.seqs.len() + cur_seq_size > SEQ_BYTE_LIMIT 
    }
    pub fn is_empty(&self) -> bool {
        self.meta.is_empty()
    }
    pub fn meta(&self) -> &[(usize, usize, usize)] {
        &self.meta
    }
    pub fn get_seqs_slice(&self, seq_ptr: usize, seq_len: usize) -> &[u8] {
        &self.seqs[seq_ptr..seq_ptr+seq_len]
    }
    // pub fn get_quals_slice(&self, qual_ptr: usize, qual_len: usize) -> &[u8] {
    //     &self.quals[qual_ptr..qual_ptr+qual_len]
    // } 
}