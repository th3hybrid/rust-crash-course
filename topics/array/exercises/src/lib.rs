pub fn zeros() -> [u32; 100] {
    [0; 100]
}

pub fn first_3(s: &[u32]) -> &[u32] {
    &s[0..3]
}

pub fn last_3(s: &[u32]) -> &[u32] {
    &s[4..7]
}
