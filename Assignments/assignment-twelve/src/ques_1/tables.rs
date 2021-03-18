use async_std::task;
use log::debug;
use std::time::Duration;

/// compute function is an async function that polls two futures simultaneously to print two tables in asynchronous manner.
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// No return value.

pub async fn compute() {
    env_logger::init();
    let first = async {
        for index in 1..11 {
            debug!("2*{} = {} ", index, 2 * index);
            task::sleep(Duration::from_secs(1)).await;
        }
    };
    let second = async {
        for index in 1..11 {
            debug!("3*{} = {} ", index, 3 * index);
            task::sleep(Duration::from_secs(1)).await;
        }
    };
    futures::join!(first, second);
}
