pub fn binds(qty: usize) -> String {
    let mut binds = String::new();

    for i in 0..qty {
        binds.push_str(&format!("?, "));
    }

    binds.pop();
    binds.pop();

    binds
}
