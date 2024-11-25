use std::{sync::Arc, time::Duration};
use futures::{lock::Mutex};
use async_std::task::{sleep as async_std_sleep,self};
mod modules;



async fn node_test(
    text:String,
    delay: Duration, 
    blockchain : Arc<Mutex<modules::blockchain::Blockchain>>){

    let mut i: u64 = 0;
    loop {
        async_std_sleep(delay).await;
        i += 1;
        let block_data = format!("{}{}",text,i);
        let mut bl = blockchain.lock().await;
            
        bl.add_block(block_data);
        // println!("NODE {}",text);
        bl.check_data();
        drop(bl);
    }
    
    
}


fn main() {
    let block_chain = modules::blockchain::Blockchain::new(1);

    let shared_blockchain = Arc::new(Mutex::new(block_chain));


    

    task::block_on(async {
        task::spawn(
            node_test(
                String::from("1st node - N"),
                Duration::from_secs(4),
                Arc::clone(&shared_blockchain)));
        task::spawn(
            node_test(
                String::from("2nd node  - N"),
                Duration::from_secs(6),
                Arc::clone(&shared_blockchain)))
    });

   loop {
       
   }
        
    

}
