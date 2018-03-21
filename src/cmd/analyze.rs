extern crate heteroprof;

use std::fs;
use std::io::BufReader;
use std::time::SystemTime;
use std::cmp::Ordering;

use interval::Interval;

impl Interval for heteroprof::Compute {
    fn lb(&self) -> f64 {
        return self.start;
    }
    fn ub(&self) -> f64 {
        return self.lb() + self.dur;
    }
}

impl Interval for heteroprof::Transfer {
    fn lb(&self) -> f64 {
        return self.start;
    }
    fn ub(&self) -> f64 {
        return self.lb() + self.dur;
    }
}

pub fn run(path: &str) {
    let start = SystemTime::now();
    let metadata = fs::metadata(path);
    let file = fs::File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut doc = heteroprof::decode_document(&mut reader).unwrap();
    eprintln!("{} computes", doc.computes().len());
    eprintln!("{} transfers", doc.transfers().len());

    // print some info about execution
    let sz = metadata.unwrap().len();
    let dur = start.elapsed().unwrap();
    let secs = (dur.as_secs() as f64) + (dur.subsec_nanos() as f64) / 1e9;
    eprintln!("{} MB", sz / 1024 / 1024);
    eprintln!("{}s elapsed", secs);
    eprintln!("{}MB/s", sz as f64 / secs / 1024 as f64 / 1024 as f64);

    // sort computes
    doc.computes_mut().sort_by(|a, b| -> Ordering {
        return a.cmp_start(b);
    });

    // sort transfers
    doc.transfers_mut().sort_by(|a, b| -> Ordering {
        return a.cmp_start(b);
    });

    let mut ci = 0 as usize;

    let mut freet = 0 as f64;

    // find all transfers not covered by a c
    for t in doc.transfers() {
        // advance c until we're covered or it's past us
        let mut c = &doc.computes()[ci];

        while ci < doc.computes().len() {
            c = &doc.computes()[ci];
            eprintln!("c is to {}-{}", c.lb(), c.ub());
            if c.overlaps(t) {
                eprintln!(
                    "t={}-{} overlapped by c={}-{}",
                    t.lb(),
                    t.ub(),
                    c.lb(),
                    c.ub()
                );
                break;
            }
            if c.lb() >= t.ub() {
                eprintln!("t={}-{} unblocked", t.lb(), t.ub());
                freet += t.ub() - t.lb();
                break;
            }
            ci += 1;
        }
    }
    eprintln!("{}", freet / 1e9);
}
