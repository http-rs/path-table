use path_table::PathTable;

fn main() {
    let mut table: PathTable<String> = PathTable::new();

    *table.setup("foo") = String::from("hello foo");
    *table.setup("foo/bar") = String::from("hello bar");
    *table.setup("{}") = String::from("hello everything else");

    println!("{}", table.route("foo").unwrap().0); // The resource at this route is the string "hello foo"
    println!("{}", table.route("foo/bar").unwrap().0); // Here, it is "hello bar"
    println!("{}", table.route("baz").unwrap().0); // For all other segments that don't match, the resource is "hello everything else"

    // Note that `{}` will only match a single segment -- there is no resource that exists at "baz/quub"
    if table.route("baz/quub").is_none() {
        println!("There is no route at baz/quub");
    }
}
