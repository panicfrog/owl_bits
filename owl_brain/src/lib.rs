pub mod prelude;
mod data;
mod gc;


pub fn brain_add(left: usize, right: usize) -> usize {
    let a = data::Data::Bool(true);
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = brain_add(2, 2);
        assert_eq!(result, 4);
    }
}
