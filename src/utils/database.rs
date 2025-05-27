pub(crate) fn regs_to_boxed<T>(items: Vec<T>) -> Vec<Box<T>> {
    let mut data: Vec<Box<T>> = Vec::with_capacity(items.len());

    for item in items {
        data.push(Box::new(item));
    }

    data
}
