use async_std::task;
use log::debug;
use std::time::Duration;

/// print_table function is an async function that polls two futures simultaneously to print two tables in asynchronous manner.
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// No return value.

pub async fn print_table() {
    env_logger::init();
    let table_2 = async {
        for index in 1..11 {
            debug!("2*{} = {} ", index, 2 * index);
            task::sleep(Duration::from_secs(1)).await;
        }
    };
    let table_3 = async {
        for index in 1..11 {
            debug!("3*{} = {} ", index, 3 * index);
            task::sleep(Duration::from_secs(1)).await;
        }
    };
    futures::join!(table_2, table_3);
}
