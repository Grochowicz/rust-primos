use std::{
    cmp::min,
    fs::File,
    io::{BufWriter, Write},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

mod seq;
// 4.294.967.295

fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let mut i = 10u32;
    let mut diffs: Vec<(u32, Duration)> = Vec::new();

    while i < u32::MAX {
        let start = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock before UNIX_EPOCH");

        for _j in 1..10 {
            let _tot = seq::contar_primos(i as usize);
        }

        let end = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System clock before UNIX_EPOCH");

        let diff = end.saturating_sub(start);
        diffs.push((i, diff));

        if i == u32::MAX {
            break;
        }
        i = min(i.saturating_mul(10), u32::MAX);
    }

    // Write the list of time differences to a file
    let file = File::create("time_differences.txt")?;
    let mut writer = BufWriter::new(file);

    for (limit, diff) in &diffs {
        writeln!(
            writer,
            "n = {}: {:?} ({} ms / {} µs)",
            limit,
            diff,
            diff.as_millis(),
            diff.as_micros()
        )?;
    }
    writer.flush()?;

    println!("Saved {} measurements to time_differences.txt", diffs.len());
    Ok(())
}