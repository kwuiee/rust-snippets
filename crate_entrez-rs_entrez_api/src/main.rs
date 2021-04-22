extern crate entrez_rs;

use entrez_rs::errors::Error;
use entrez_rs::eutils::{EFetch, ESearch, Eutils, DB};
use entrez_rs::parser::esearch::ESearchResult;
use entrez_rs::parser::pubmed::PubmedArticleSet;

fn main() -> Result<(), Error> {
    let xml = ESearch::new(DB::Pubmed, "eclampsia").run()?;

    let parsed = ESearchResult::read(&xml);

    println!("{:#?}", &parsed?.id_list.ids);

    let pm_xml = EFetch::new(DB::Pubmed, vec!["33246200", "33243171"]).run()?;

    let pm_parsed = PubmedArticleSet::read(&pm_xml);

    println!("{:?}", pm_parsed?.articles);

    Ok(())
}
