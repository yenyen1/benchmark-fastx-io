# Multi-Threaded Programming in Rust

In my [multi-threaded implementation](../src/parallel/fasta_manu.rs#L12), I use a reader thread to read data and multiple worker threads to perform the computation. The buffers are created and managed by a buffer pool. The filled buffers are passed through a channel from the reader thread to the worker threads.
<br>

**The workflow goes like this:** The reader thread takes an empty buffer from the pool, reads the data, and stores it in the buffer. Then, the buffer is sent to the channel to wait for a worker to process it. The worker threads receive a buffer and carry out the computation. After finishing the task, the buffer is cleared and sent back to the pool for reuse. Once the worker completes its job, the result is sent back to the main thread.

## The channel 
I used `crossbeam_channel` crate to handle sending and receiving data across threads. This crate supports multi-producer multi-consumer (MPMC) and ensures channel access safety without needing to implement `Arc<Mutex<T>>`. Multiple receivers can be implemented straightforward by invoking `rx.clone()`.

## Buffer Pool and Configuration
I use a buffer pool to manage a reuseable buffer set to avoid unnecessary menory alocation. Since a record includes an ID, sequence, and quality scores, I use a struct called [`OwnedRecordSet`](../src/parallel/record.rs). It contains three `u8` vectors to store the IDs, sequences, and quality scores, as well as metadata vector to keep track of the offset and the length of each record.

Since I want to accommodate both short-read and long-read scenarios, the sequence and quality vectors are initialized with a capacity of 4MiB, while the ID vector is initialized with 1MiB. Moreover, the number of records in a single set is capped at 16K, and the total size of the sequence and quality vectors is also capped at 4MiB. These values were configured based on the reported range of Ultra-long Nanopore reads (which average 10Kb to 50Kb, but can theoretically exceed 1Mbp) and the typical range of short reads (< 600bp). 

Note: The longest recorded read length is [4.2Mb](https://nanoporetech.com/blog/news-blog-kilobases-whales-short-history-ultra-long-reads-and-high-throughput-genome). However, the data I have been dealing with has never reached that length. I set the cap at 4MiB to avoid wasting memory and to ensure better load balancing across worker threads, but this is flexible and can be adjusted depending on the specific use case.


<br>

[Back to ReadMe](../readme.md)