#[macro_use]
extern crate derive_builder;

#[derive(Builder, Debug, PartialEq)]
#[builder(build_fn(validate = "Self::validate"))]
struct Lorem {
    pub normal: u8,

    // Default value.
    #[builder(default = "42")]
    pub default_value: u8,

    // Hide fields by skipping their setters.
    #[builder(setter(skip))]
    skipped: bool,

    // Convert input value into string.
    #[builder(setter(into))]
    pub string: String,

    #[builder(try_setter, setter(into, name = "foo"))]
    pub try_setter: u8,

    // Custom defaults can delegate to helper methods
    // and pass errors to the enclosing `build()` method via `?`.
    #[builder(default = "self.default_dolor()?")]
    dolor: String,
}

impl LoremBuilder {
    /// Pre-Build Validation
    /// Check that `Lorem` is putting in the right amount of effort.
    fn validate(&self) -> Result<(), String> {
        if let Some(ref v) = self.normal {
            match *v {
                i if i < 20 => Err("Try harder".to_string()),
                i if i > 100 => Err("You'll tire yourself out".to_string()),
                _ => Ok(()),
            }
        } else {
            Ok(())
        }
    }

    // Private helper method with access to the builder struct.
    fn default_dolor(&self) -> Result<String, String> {
        match self.string {
            Some(ref x) if x.chars().count() > 3 => Ok(format!("dolor {}", x)),
            _ => Err("ipsum must at least 3 chars to build dolor".to_string()),
        }
    }
}

fn main() {
    let v = LoremBuilder::default()
        .string("Hello!")
        .try_foo(10)
        .unwrap()
        .normal(30)
        .build()
        .unwrap();
    println!("{:?}", v);
}
