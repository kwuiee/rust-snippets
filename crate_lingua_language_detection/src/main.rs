use float_cmp::approx_eq;
use lingua::Language::{English, French, German, Spanish};
use lingua::{Language, LanguageDetector, LanguageDetectorBuilder};

///
///[Ref](https://github.com/pemistahl/lingua-rs)
///
fn main() {
    // Basic
    let languages = vec![English, French, German, Spanish];
    let detector: LanguageDetector = LanguageDetectorBuilder::from_languages(&languages).build();
    let detected_language: Option<Language> = detector.detect_language_of("languages are awesome");

    assert_eq!(detected_language, Some(English));

    // Minimum relative distance
    // let detector = LanguageDetectorBuilder::from_languages(&[English, French, German, Spanish])
    //     .with_minimum_relative_distance(0.25) // minimum: 0.00 maximum: 0.99 default: 0.00
    //     .build();
    let detected_language = detector.detect_language_of("languages are awesome");

    assert_eq!(detected_language, None);

    // Confidence values

    let confidence_values: Vec<(Language, f64)> =
        detector.compute_language_confidence_values("languages are awesome");

    // The more readable version of the assertions below:
    // assert_eq!(
    //     confidence_values,
    //     vec![(English, 1.0), (French, 0.79), (German, 0.75), (Spanish, 0.72)]
    // );

    assert_eq!(confidence_values[0], (English, 1.0_f64));

    assert_eq!(confidence_values[1].0, French);
    assert!(approx_eq!(
        f64,
        confidence_values[1].1,
        0.7917282993701181,
        ulps = 2
    ));

    assert_eq!(confidence_values[2].0, German);
    assert!(approx_eq!(
        f64,
        confidence_values[2].1,
        0.7532048914992281,
        ulps = 2
    ));

    assert_eq!(confidence_values[3].0, Spanish);
    assert!(approx_eq!(
        f64,
        confidence_values[3].1,
        0.7229637749926444,
        ulps = 2
    ));
}
