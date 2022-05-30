// cargo-deps: comrak = "0.12.1"

use std::fs;
use std::path::Path;

use comrak;

fn main() {
    // Make Build Dir
    if !Path::new("./build").exists() {
        fs::create_dir("./build").unwrap();
    }

    // Init Comrak
    let mut opt = comrak::ComrakOptions::default();
    opt.extension.table = true;
    opt.extension.strikethrough = true;
    opt.extension.autolink = true;
    opt.extension.header_ids = Some("".to_owned());
    opt.extension.footnotes = true;
    opt.parse.smart = true;
    opt.render.unsafe_ = true;

    for mut i in fs::read_dir(".").unwrap().map(|x| x.unwrap().path()) {
        if i.extension().is_none() || i.extension().unwrap().to_str().unwrap() != "md" {
            continue;
        }

        println!("[BUILDING] {}", &i.to_string_lossy());
        let md = fs::read_to_string(&i).unwrap();
        i.set_extension("");

        fs::write(
            Path::new("./build").join(&i),
            comrak::markdown_to_html(&md, &opt),
        )
        .unwrap();
    }
}
