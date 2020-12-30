#![feature(map_first_last, unsigned_abs)]
extern crate bam;
#[macro_use]
extern crate clap;
extern crate plotlib;

use std::io::ErrorKind::InvalidData;
use std::io::{Error, Result};

use bam::record::Record;
use bam::{BamReader, RecordReader};
use clap::{App, AppSettings};
use plotlib::grid::Grid;
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{LineJoin, LineStyle};
use plotlib::view::{ContinuousView, View};

/// Read is paired, first in pair, properly mapped.
const P_FLAG: u16 = 0x1 + 0x2 + 0x40;
/// Read is secondary or supplementary.
const N_FLAG: u16 = 0x100 + 0x800;

fn opterr() -> std::io::Error {
    Error::new(InvalidData, "Option error.")
}

fn cli(bam: &str, svg: &str, upper: &usize) -> Result<()> {
    let mut data = vec![0u32; *upper + 1];
    let mut record = Record::new();
    let mut reader = BamReader::from_path(bam, 0)?;
    let mut total = 0;
    while reader.read_into(&mut record)? {
        if record.flag().0 & P_FLAG != P_FLAG
            || record.flag().0 & N_FLAG != 0
            || record.ref_id() != record.mate_ref_id()
        {
            continue;
        };
        let tlen = record.template_len().unsigned_abs() as usize;
        if &tlen > upper {
            continue;
        };
        data[tlen] += 1;
        total += 1;
    }
    // Plot line.
    let line = Plot::new(
        data.into_iter()
            .enumerate()
            .map(|(i, j)| (i as f64, (j as f64) / (total as f64)))
            .collect(),
    )
    .line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round)
            .width(1.0),
    );
    let mut view = ContinuousView::new()
        .add(line)
        .x_label("插入片段大小(bp)")
        .y_label("比例");
    view.add_grid(Grid::new(8, 8));
    Page::single(&view).save(svg).expect("saving svg");
    Ok(())
}

fn main() -> Result<()> {
    let opts = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .args_from_usage(
            "
            <svg> -o=[FILE] 'Output svg file path.'
            [upper] -m=[NUMBER] 'Maximum insert size to record. Bigger number costs more memory.'
            <bam> 'Input bam file.'
            ",
        )
        .get_matches();
    let bam: &str = opts.value_of("bam").ok_or_else(opterr)?;
    let svg: &str = opts.value_of("svg").ok_or_else(opterr)?;
    let upper: usize = opts
        .value_of("upper")
        .unwrap_or("500")
        .parse()
        .map_err(|_| opterr())?;
    cli(bam, svg, &upper)
}
