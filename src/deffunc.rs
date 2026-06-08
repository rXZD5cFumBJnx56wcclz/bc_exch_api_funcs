pub fn usizezero(a: &usize) -> &usize {
    match a {
        &0 => &usize::MAX,
        _ => a,
    }
}
