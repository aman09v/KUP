use async_std::task;
use log::debug;
use std::time::Duration;

/// run function is an async function that polls two futures simultaneously to run two task in asynchronous manner.
///
/// #Arguments
///
/// No Arguments.
///
/// #Return
///
/// No return value.

pub async fn run() {
    let first = async {
        for _index in 0..10 {
            debug!("running task 1");
            task::sleep(Duration::from_secs(2)).await;
        }
    };
    let second = async {
        for _index in 0..10 {
            debug!(" running task 2 ");
            task::sleep(Duration::from_secs(1)).await;
        }
    };
    futures::join!(first, second);
}
