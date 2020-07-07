extern crate semver;

use semver::{Version, VersionReq};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // comparsion
    assert!(
        Version::parse("1.2.3")
            == Ok(Version {
                major: 1,
                minor: 2,
                patch: 3,
                pre: vec!(),
                build: vec!()
            })
    );
    assert!(Version::parse("1.2.3-alpha") < Version::parse("1.2.3-beta"));
    assert!(Version::parse("1.2.3") > Version::parse("1.2.3-beta"));

    // operators
    let mut chrome_release = Version::parse("41.5.5377")?;
    chrome_release.increment_major();
    assert_eq!(Ok(chrome_release), Version::parse("42.0.0"));

    // requirment
    let r1 = VersionReq::parse(">= 1.0.0")?;
    let r2 = VersionReq::parse("^1.0.0")?;
    let r3 = VersionReq::parse("~1.2.0")?;
    let r4 = VersionReq::parse("1.*.3")?;
    let v = Version::parse("1.2.3")?;
    assert!(r1.to_string() == ">=1.0.0".to_string());
    assert!(r1.matches(&v) && r2.matches(&v) && r3.matches(&v) && r4.matches(&v));
    Ok(())
}
