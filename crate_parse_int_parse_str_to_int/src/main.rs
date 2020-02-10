extern crate num_traits;
extern crate parse_int;

use num_traits::Num::FromStrRadixErr;
use parse_int::parse;

fn main<T>() -> Result<(), T::FromStrRadixErr> {
    let d = parse::<usize>("42")?;
    assert_eq!(42, d);

    let d = parse::<isize>("0x42")?;
    assert_eq!(66, d);

    // you can use underscores for more readable inputs
    let d = parse::<isize>("0x42_424_242")?;
    assert_eq!(1_111_638_594, d);

    // octal explicit
    let d = parse::<u8>("0o42")?;
    assert_eq!(34, d);

    #[cfg(feature = "implicit-octal")]
    {
        let d = parse::<i8>("042")?;
        assert_eq!(34, d);
    }

    let d = parse::<u16>("0b0110")?;
    assert_eq!(6, d);
    Ok(())
}
