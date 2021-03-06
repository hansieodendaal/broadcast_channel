use futures::{executor::block_on, stream, StreamExt};
use tari_broadcast_channel::bounded;

fn main() {
    let (publisher, subscriber1) = bounded(10, 1);
    let subscriber2 = subscriber1.clone();
    let subscriber3 = subscriber1.clone();

    assert_eq!(subscriber1.get_receiver_id(), 10000);
    assert_eq!(subscriber2.get_receiver_id(), 10001);
    assert_eq!(subscriber3.get_receiver_id(), 10002);

    block_on(async move {
        stream::iter(1..15).map(Ok).forward(publisher).await.unwrap();
    });

    let received1: Vec<u32> = block_on(async { subscriber1.map(|x| *x).collect().await });
    let received2: Vec<u32> = block_on(async { subscriber2.map(|x| *x).collect().await });
    // Test that only the last 10 elements are in the received list.
    let expected = (5..15).collect::<Vec<u32>>();
    assert_eq!(received1, expected);
    assert_eq!(received2, expected);
}
