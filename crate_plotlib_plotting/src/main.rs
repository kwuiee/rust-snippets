use plotlib::page::Page;
use plotlib::repr::{BarChart, Histogram, HistogramBins, Plot};
use plotlib::style::{BoxStyle, LineJoin, LineStyle, PointMarker, PointStyle};
use plotlib::view::{CategoricalView, ContinuousView};

fn main() {
    // Scatter plots expect a list of pairs
    let data1 = vec![
        (-3.0, 2.3),
        (-1.6, 5.3),
        (0.3, 0.7),
        (4.3, -1.4),
        (6.4, 4.3),
        (8.5, 3.7),
    ];

    // We create our scatter plot from the data
    let s1: Plot = Plot::new(data1).point_style(
        PointStyle::new()
            .marker(PointMarker::Square) // setting the marker to be a square
            .colour("#DD3355"),
    ); // and a custom colour

    // We can plot multiple data sets in the same view
    let data2 = vec![(-1.4, 2.5), (7.2, -0.3)];
    let s2: Plot = Plot::new(data2).point_style(
        PointStyle::new() // uses the default marker
            .colour("#35C788"),
    ); // and a different colour

    // The 'view' describes what set of data is drawn
    let v = ContinuousView::new()
        .add(s1)
        .add(s2)
        .x_range(-5., 10.)
        .y_range(-2., 6.)
        .x_label("Some varying variable")
        .y_label("The response of something");

    // A page with a single view is then saved to an SVG file
    Page::single(&v).save("scatter.svg").unwrap();

    // Histogram
    let data = [0.3, 0.5, 6.4, 5.3, 3.6, 3.6, 3.5, 7.5, 4.0];
    let h = Histogram::from_slice(&data, HistogramBins::Count(10))
        .style(&BoxStyle::new().fill("burlywood"));
    let v = ContinuousView::new().add(h);
    Page::single(&v).save("histogram.svg").expect("saving svg");

    // Line
    let l1 = Plot::new(vec![(0., 1.), (2., 1.5), (3., 1.2), (4., 1.1)]).line_style(
        LineStyle::new()
            .colour("burlywood")
            .linejoin(LineJoin::Round),
    );
    let v = ContinuousView::new().add(l1);
    Page::single(&v).save("line.svg").expect("saving svg");

    // Bar chart.
    let b1 = BarChart::new(5.3).label("1");
    let b2 = BarChart::new(2.6)
        .label("2")
        .style(&BoxStyle::new().fill("darkolivegreen"));
    let v = CategoricalView::new().add(b1).add(b2).x_label("Experiment");
    Page::single(&v).save("barchart.svg").expect("saving svg");
}
