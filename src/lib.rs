mod old_calc;


#[cfg(test)]
mod tests{
    use crate::old_calc;

    #[test]
    fn calc_test(){
        assert_eq!(old_calc::calculate("3+5*-3"), -12);
    }
}