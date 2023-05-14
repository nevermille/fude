use mudpf::MuPDF;

#[test]
fn hello_world() {
    let mut mu = MuPDF::new();

    let mut out_name = std::env::temp_dir();
    out_name.push("mupdf_hello_world.pdf");

    mu.add_page();
    mu.add_ttf_font("assets/OpenSans-Regular.ttf").unwrap();
    mu.print_text("Hello World!");
    mu.finalize();
    mu.save(out_name.display().to_string().as_str()).unwrap();

    println!("File saved at {}", out_name.display());
}
