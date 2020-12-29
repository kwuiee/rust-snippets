#![feature(map_first_last, unsigned_abs)]
extern crate bam;
#[macro_use]
extern crate clap;
extern crate plotlib;
extern crate sled;

use std::collections::BTreeMap;
use std::convert::From;
use std::convert::TryInto;
use std::error::Error;

use bam::record;
use bam::BamReader;
use clap::{App, AppSettings};
use plotlib::grid::Grid;
use plotlib::page::Page;
use plotlib::repr::BarChart;
use plotlib::view::{CategoricalView, View};
use sled::Db as Sled;
use sled::{Config, IVec};

struct Bookkeeper {
    inner: Sled,
    cache: BTreeMap<u32, u32>,
    capacity: usize,
}

impl Bookkeeper {
    fn new() -> Result<Self, sled::Error> {
        let config = Config::new().temporary(true);
        Ok(Self {
            inner: config.open()?,
            cache: BTreeMap::new(),
            capacity: 200,
        })
    }

    fn value_mut(&mut self, v: &u32) -> Result<&mut u32, Box<dyn Error>> {
        if self.cache.contains_key(&v) {
            return Ok(self.cache.get_mut(&v).unwrap());
        };
        let bv = u32::to_le_bytes(*v);
        match self.inner.remove(&bv)? {
            Some(old) => {
                if self.cache.len() > self.capacity {
                    let (key, value) = self.cache.pop_first().unwrap();
                    self.inner
                        .insert(&u32::to_le_bytes(key), &u32::to_le_bytes(value))?;
                };
                self.cache
                    .insert(*v, u32::from_le_bytes(<&[u8]>::from(&old).try_into()?));
                Ok(self.cache.get_mut(&v).unwrap())
            }
            None => {
                if self.cache.len() > self.capacity {
                    let (key, value) = self.cache.pop_first().unwrap();
                    self.inner
                        .insert(&u32::to_le_bytes(key), &u32::to_le_bytes(value))?;
                };
                self.cache.insert(*v, 0);
                Ok(self.cache.get_mut(&v).unwrap())
            }
        }
    }

    /// SVG plot.
    ///
    /// ## FIXME
    ///
    /// - X-axis order error.
    fn write_svg(&mut self, output: &str) -> Result<(), Box<dyn Error>> {
        let mut v = CategoricalView::new().x_label("Insert size");
        v.add_grid(Grid::new(20, 1));
        for each in self.inner.iter() {
            let (key, value): (IVec, IVec) = each?;
            let key = u32::from_le_bytes(key.as_ref().try_into()?);
            let value = u32::from_le_bytes(value.as_ref().try_into()?) as f64;
            if key > 2000 {
                continue;
            };
            let bc = BarChart::new(value).label(key.to_string());
            v = v.add(bc);
        }
        while let Some((key, value)) = self.cache.pop_first() {
            if key > 2000 {
                continue;
            };
            let bc = BarChart::new(value as f64).label(key.to_string());
            v = v.add(bc);
        }
        Page::single(&v).save(output)?;
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opts = App::new(crate_name!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .args_from_usage(
            "
            <output> -o, --output=[file] 'Output file path.'
            <bam> 'Input bam file.'
            ",
        )
        .get_matches();
    let bam = opts.value_of("bam").unwrap();
    let output = opts.value_of("output").unwrap();
    let mut bk = Bookkeeper::new()?;
    for i in BamReader::from_path(bam, 0)? {
        let rd = i?;
        let ref flag = rd.flag().0;
        let positive = record::RECORD_PAIRED + record::FIRST_IN_PAIR + record::ALL_SEGMENTS_ALIGNED;
        let negative = record::SECONDARY + record::SUPPLEMENTARY;
        if flag & positive != positive || flag & negative != 0 || rd.ref_id() != rd.mate_ref_id() {
            continue;
        };
        bk.value_mut(&rd.template_len().unsigned_abs())
            .map(|v| *v += 1)?;
    }
    bk.write_svg(output)?;
    Ok(())
}
