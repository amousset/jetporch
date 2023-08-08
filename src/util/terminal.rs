
// ==============================================================================================================
// PUBLIC API
// ==============================================================================================================

pub fn markdown_print(markdown: &String) {
    termimad::print_text(markdown);
}

pub fn banner(msg: String) {
    let markdown = String::from(format!("|:-|\n\
                                        |{}|\n\
                                        |-", msg));
    markdown_print(&markdown);
}

pub fn two_column_table(header_a: String, header_b: String, elements: Vec<(String,String)>) {
    let mut buffer = String::from("|:-|:-\n");
    buffer.push_str(
        &String::from(format!("|{}|{}\n", header_a, header_b))
    );
    for (a,b) in elements.iter() {
        buffer.push_str(&String::from("|-|-\n"));
        buffer.push_str(
            &String::from(format!("|{}|{}\n", a, b))
        );
    }
    buffer.push_str(&String::from("|-|-\n"));
    markdown_print(&buffer);
}

pub fn captioned_display(caption: String, body: String) {

    banner(caption.clone());
    println!("");
    for line in body.lines() {
        println!("    {}", line);
    }
    println!("");

}