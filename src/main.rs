fn calculate(x: &mut Vec<&str>) -> Option<usize> {
    let last_elem = x.pop()?;
    Some(last_elem.len())
}

fn main() {
    let mut animals: Vec<&str> = vec!["Giraffe", "Monkey", "Zebra"];

    println!("{:#?}", calculate(&mut animals));
    println!("{:#?}", calculate(&mut animals));
    println!("{:#?}", calculate(&mut animals));
    println!("{:#?}", calculate(&mut animals));


}
