use tree_sitter::{Parser, Language};

use std::path::Path;
use std::fs::read_to_string;

extern "C" {fn tree_sitter_haskell() -> Language; }

fn main() {
    let mut parser = Parser::new();

    let language = unsafe { tree_sitter_haskell() };
    parser.set_language(language).unwrap();

    // let source_fn = Path::new("test.hs");
    // let source_code = read_to_string(source_fn).unwrap();
    
    // let source_code = "x = 1";
    let source_code = b"type FilePath = String";

    let tree = parser.parse(source_code, None).unwrap();

    let new_type_name = tree.root_node().child(0).unwrap().child(1).unwrap().utf8_text(source_code).unwrap();

    println!("{}", tree.root_node().to_sexp());
    println!("{}", new_type_name);

}




// use tree_sitter_parsers::parse;


// fn main() {
//     let tree = parse("type Memes = String", "haskell");
//     println!("{}", tree.root_node().to_sexp());
// }
