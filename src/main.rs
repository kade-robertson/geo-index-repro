use geo_index::rtree::{sort::HilbertSort, RTreeBuilder};

fn main() {
    let mut builder: RTreeBuilder<f64> = RTreeBuilder::new(1);
    builder.add(-20., -20., 1020., 1020.);
    builder.finish::<HilbertSort>();
}
