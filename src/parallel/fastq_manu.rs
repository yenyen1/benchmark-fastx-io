use crate::parallel::record::OwnedRecordSet;
use crate::utils::utils::open_bufreader;
use crate::utils::dna::get_actgn_idx_from_ascii;

use std::thread;

use anyhow::Result;
use crossbeam_channel::bounded;
use seq_io::fastq::{Reader, Record};

pub fn manu_parallel(path: &str) -> std::io::Result<()> {
    let num_workers: usize = 2;
    let pool_size: usize = num_workers *2;

    // 0-1. Create channel to pass records to workers
    let (work_tx, work_rx) = bounded::<OwnedRecordSet>(pool_size);
    // 0-2. Create channel to pass results to main thread
    let (result_tx, result_rx) = bounded::<[u64; 5]>(num_workers);
    // 0-3. Create pool manager
    let (pool_tx, pool_rx) = bounded::<OwnedRecordSet>(pool_size);
    for _ in 0..pool_size {
        pool_tx.send(OwnedRecordSet::new()).unwrap();
    }

    // 1. Create reader thread 
    let owned_path = path.to_string();
    let reader_handle = thread::spawn(move || -> Result<()> {
        let mut reader = open_bufreader(&owned_path).map(Reader::new)?;
        let mut record_set = pool_rx.recv()?;
        while let Some(record) = reader.next() {
            let record = record.unwrap();

            if record_set.is_overload(record.seq().len()) {
                work_tx.send(record_set).unwrap();
                record_set = pool_rx.recv()?;
            }
            record_set.push(record.id_bytes(), record.seq(), record.qual());
            
        }
        if !record_set.is_empty() {
            work_tx.send(record_set).unwrap();
        }

        Ok(())
    });

    // 2. Create workers threads
    let mut workers_handles: Vec<thread::JoinHandle<()>> = Vec::new();
    for _ in 0..num_workers {
        let w_rx = work_rx.clone();
        let p_tx = pool_tx.clone();
        let res_tx = result_tx.clone();

        workers_handles.push(thread::spawn(move || {
            let mut result: [u64; 5] = [0; 5];

            // Continuously getting record sets from the pool channel until the reader thread closed
            for mut owned_records in w_rx {

                let mut seq_ptr: usize = 0;
                for &(_id_len, seq_len, _qual_len) in owned_records.meta() {
                    let seq = owned_records.get_seqs_slice(seq_ptr, seq_len);
                    for base in seq {
                        result[get_actgn_idx_from_ascii(base)] += 1;
                    }
                    seq_ptr += seq_len;
                }
                owned_records.clear();
                // if let Err(e) = p_tx.send(owned_records) {
                //     eprintln!("Pool Reciever Closed with reader dropped:{}. No longer need to return the buffer.", e);
                // }
                let _ = p_tx.send(owned_records);

            }
            // send result back to the main thread
            res_tx.send(result).unwrap();
            
        }));
    }

    // 3. Main Thread summarized results
    drop(result_tx);
    let mut sum_result: [u64; 5]= [0; 5];
    for res in result_rx.iter(){
        for i in 0..5 {
            sum_result[i] += &res[i];
        }
    };

    // 4. Wait for reader thread and worker threads closed
    if let Err(e) = reader_handle.join().unwrap() {
        eprintln!("Reader Error: {}", e);
    }
    for h in workers_handles {
        h.join().unwrap();
    }

    Ok(())
}

