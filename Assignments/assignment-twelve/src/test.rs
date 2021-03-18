#[cfg(test)]
mod tests {
    use crate::ques_1::tables::compute;
    use crate::ques_2::async_program::run;
    use futures::executor::block_on;

    #[test]
    fn tables_check_success() {
        assert_eq!(block_on(compute()), ());
    }

    #[test]
    fn async_program_check_success() {
        assert_eq!(block_on(run()), ());
    }
}
