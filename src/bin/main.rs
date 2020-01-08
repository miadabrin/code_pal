use code_pal::todo::TodoItem;

fn main() {
    let todo1 = TodoItem {
        title: String::from("title"),
        description: vec![],
        notes: vec![],
    };
    println!("Hello, world! {:#?}", todo1);
}
