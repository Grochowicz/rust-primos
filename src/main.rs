use std::{
    fs::File,
    hint::black_box,
    io::{self, BufWriter, Write},
    thread,
    cmp::min,
    time::{Duration, SystemTime, UNIX_EPOCH}
}
mod par;
mod seq;

const LIMITS: [u32; 3] = [100_000, 1_000_000, 10_000_000];
const OUTPUT_PATH: &str = "benchmark.csv";

fn main() -> io::Result<()> {
    let max_threads = thread::available_parallelism()
        .map(|count| count.get())
        .unwrap_or(1);
    let thread_counts = benchmark_thread_counts(max_threads);
    let mut output = BufWriter::new(File::create(OUTPUT_PATH)?);
    writeln!(output, "title,time")?;

    println!("Benchmarking with up to {max_threads} threads...");

    for limit in LIMITS {
        let (expected, elapsed) = measure(|| seq::primes_up_to(black_box(limit)));
        write_result(&mut output, "seq", limit, 1, elapsed)?;

        for &thread_count in &thread_counts {
            let (actual, elapsed) =
                measure(|| par::primes_up_to(black_box(limit), black_box(thread_count)));

            assert_eq!(
                actual, expected,
                "parallel result differs for n={limit}, threads={thread_count}"
            );
            write_result(&mut output, "par", limit, thread_count, elapsed)?;
        }
    }

    output.flush()?;
    println!("Results written to {OUTPUT_PATH}");
    Ok(())
}

fn benchmark_thread_counts(max_threads: usize) -> Vec<usize> {
    let mut counts: Vec<_> = [1, 2, 4, max_threads]
        .into_iter()
        .filter(|&count| count <= max_threads)
        .collect();
    counts.sort_unstable();
    counts.dedup();
    counts
}

fn measure<T>(operation: impl FnOnce() -> T) -> (T, Duration) {
    let start = Instant::now();
    let result = operation();
    (result, start.elapsed())
}

fn write_result(
    output: &mut impl Write,
    implementation: &str,
    limit: u32,
    threads: usize,
    elapsed: Duration,
) -> io::Result<()> {
    let title = format!("{implementation}_n={limit}_threads={threads}");
    writeln!(output, "{title},{:.9}", elapsed.as_secs_f64())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thread_cases_are_modest_unique_and_reach_available_parallelism() {
        assert_eq!(benchmark_thread_counts(1), vec![1]);
        assert_eq!(benchmark_thread_counts(3), vec![1, 2, 3]);
        assert_eq!(benchmark_thread_counts(6), vec![1, 2, 4, 6]);
        assert_eq!(benchmark_thread_counts(8), vec![1, 2, 4, 8]);
    }
}
