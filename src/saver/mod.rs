//! Module which regroup all Saver

pub mod definition;
pub mod entity;
pub mod relation;
pub mod semdef;
pub mod semmet;
pub mod semrel;

/// Alias for CSV writer used by saver
type Writer = csv::Writer<std::io::BufWriter<std::fs::File>>;

/// Macro to initialize CSV Writer
#[macro_export]
macro_rules! writer {
    ($dir:expr, $name:expr,$header:expr) => {{
        let mut w =
            csv::Writer::from_writer(std::io::BufWriter::with_capacity(
                1024 * 1024,
                std::fs::File::create($dir.join(concat!($name, ".csv")))?,
            ));
        w.write_record($header)?;
        w
    }};
}
