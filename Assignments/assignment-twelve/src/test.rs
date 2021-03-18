#[cfg(test)]
mod tests {
    use crate::ques_1::tables::print_table;
    use crate::ques_2::async_program::run_task;
    use futures::executor::block_on;

    #[test]
    fn tables_check_success() {
        assert_eq!(block_on(print_table()), ());
    }

    #[test]
    fn async_program_check_success() {
        assert_eq!(block_on(run_task()), ());
    }
}
