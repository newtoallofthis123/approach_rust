#![allow(non_snake_case)]

pub mod stream;
pub mod container;
pub mod node;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn sub(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn does_this_work(){
        let result = sub(2, 2);
        assert_eq!(result, 0);
    }
}
