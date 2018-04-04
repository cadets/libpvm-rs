#![feature(try_from)]
#![feature(slice_patterns)]
#![feature(box_patterns)]

extern crate futures_cpupool;
extern crate libc;
#[macro_use]
extern crate maplit;
extern crate neo4j;
extern crate num_cpus;
extern crate rayon;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate futures;
extern crate nix;
extern crate serde_json;
extern crate tokio_core;

macro_rules! timeit {
    ($E:expr) => {{
        use std::time::Instant;
        let now = Instant::now();
        $E;
        let dur = now.elapsed();
        eprintln!(
            "{} took {:.3}",
            stringify!($E),
            dur.as_secs() as f64 + f64::from(dur.subsec_nanos()) * 1e-9
        );
    }};
}

pub use c_api::*;

pub mod c_api;
pub mod checking_store;
pub mod data;
pub mod ingest;
pub mod invbloom;
pub mod iostream;
pub mod neo4j_glue;
pub mod query;
pub mod trace;
pub mod uuid;
