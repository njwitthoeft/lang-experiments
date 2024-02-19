mod j;

use j::parse;

fn main() {
    let unparsed_file = std::fs::read_to_string("program.ijs").expect("cannot read ijs file");
    let astnode = parse(&unparsed_file).expect("unsuccessful parse");
    println!("{:?}", &astnode);
}
