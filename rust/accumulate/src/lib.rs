pub fn map<I, O, F>(input: Vec<I>, mut function: F) -> Vec<O>
where
    F: FnMut(I) -> O,
{
    let mut res = Vec::<O>::with_capacity(input.len());
    for e in input {
        res.push(function(e));
    }
    res
}
