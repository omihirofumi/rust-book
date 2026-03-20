use std::time::Duration;

fn main() {
    trpl::block_on(async {
        // let handle = trpl::spawn_task(async {
        //     for i in 1..10 {
        //         println!("hi numbber {i} from the first task!");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // });

        // for i in 1..5 {
        //     println!("hi number {i} from the second task!");
        //     trpl::sleep(Duration::from_millis(500)).await
        // }

        // handle.await.unwrap();

        // let fut1 = async {
        //     for i in 1..10 {
        //         println!("first: {i}");
        //         trpl::sleep(Duration::from_millis(500)).await;
        //     }
        // };
        // fut1.await;

        // // let fut2 = async {
        // for i in 1..5 {
        //     println!("second: {i}");
        //     trpl::sleep(Duration::from_millis(100)).await;
        // }
        // };

        // trpl::join(fut1, fut2).await;
        // 遅延評価なので、ここで初めて実行される
        // fut1.await;

        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        // awaitキーワードがコード中に現れる順序がプログラム実行時に実際に実行される順
        // なので、上のawaitが全部通過したら、やっとここに来る
        // なので、2000 ms後にメッセージが一気に来る
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    })
}
