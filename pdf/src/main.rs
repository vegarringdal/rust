use usvg::fontdb;



fn main() {
    let svg = std::fs::read_to_string("test2.svg").unwrap();


    let mut options = usvg::Options::default();
    options.fontdb = fontdb::Database::new();
    options.fontdb.load_system_fonts(); 

    let tree =
        usvg::Tree::from_str(&svg, &options.to_ref()).map_err(|err| err.to_string()).unwrap();

    // This can only fail if the SVG is malformed. This one is not.
    let pdf = svg2pdf::convert_str(&svg, svg2pdf::Options::default()).unwrap();
    let mut options = svg2pdf::Options::default();

    let pdf = svg2pdf::convert_tree(&tree, options);

    // ... and now you have a Vec<u8> which you could write to a file or
    // transmit over the network!
    std::fs::write("target/example.pdf", pdf).unwrap();
}
