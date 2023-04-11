
use clap::{App, Arg};
use flume::Receiver;
use rand::Rng;
use serde_json;
use serde_json::Value;
use zenoh::shm::{SharedMemoryManager, SharedMemoryBuf};
const N: usize = 1024 * 1024;
const ELEM_SIZE: usize = 1024;
const SHM_BACKOFF: u64 = 500;

const VALUE: &str = r#"
{
    "house": "1",
    "records": {
      "timestamps": [
        "2019-03-02 14:30:00",
        "2019-03-02 14:31:00",
        "2019-03-02 14:32:00",
        "2019-03-02 14:33:00",
        "2019-03-02 14:34:00",
        "2019-03-02 14:35:00",
        "2019-03-02 14:36:00",
        "2019-03-02 14:37:00",
        "2019-03-02 14:38:00",
        "2019-03-02 14:39:00"
      ],
      "illuminance": [
        7,
        7,
        7,
        7,
        7,
        7,
        7,
        7,
        7,
        7
      ],
      "occupancy": [
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false,
        false
      ],
      "temperature": [
        15.18,
        15.18,
        15.18,
        15.18,
        15.18,
        15.18,
        15.18,
        15.18,
        15.18,
        15.18
      ],
      "humidity": [
        66.51,
        66.51,
        66.51,
        66.51,
        66.51,
        66.51,
        66.51,
        66.51,
        66.51,
        66.51
      ],
      "pressure": [
        1020.63,
        1020.63,
        1020.63,
        1020.63,
        1020.63,
        1020.63,
        1020.63,
        1020.63,
        1020.63,
        1020.63
      ],
      "windspeed": [
        4.3,
        4.3,
        4.3,
        4.3,
        4.3,
        4.3,
        4.3,
        4.3,
        4.3,
        4.3
      ],
      "winddirection": [
        "Norte",
        "Norte",
        "Norte",
        "Norte",
        "Norte",
        "Norte",
        "Norte",
        "Norte",
        "Norte",
        "Norte"
      ],
      "out_pressure": [
        1029.5,
        1029.5,
        1029.5,
        1029.5,
        1029.5,
        1029.5,
        1029.5,
        1029.5,
        1029.5,
        1029.5
      ],
      "out_humidity": [
        99,
        99,
        99,
        99,
        99,
        99,
        99,
        99,
        99,
        99
      ],
      "out_temperature": [
        11,
        11,
        11,
        11,
        11,
        11,
        11,
        11,
        11,
        11
      ],
      "precipitation": [
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0
      ]
    }
  }
"#;


async fn read_task(rx: Receiver<SharedMemoryBuf>, elem_size: usize) {
    let mut count = 0;
    while let Ok(buff) = rx.recv_async().await {
        let s_pos = rand::random::<usize>()%(elem_size-16);
        println!("[{}] Receiving SHM Data [{}..{}] {:?}", count, s_pos, s_pos+16, &buff.as_slice()[s_pos..s_pos+16]);
        count += 1;
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initiate logging
    env_logger::init();

    let (tx, rx) = flume::unbounded::<SharedMemoryBuf>() ;

    let (elem_num, elem_size) = parse_args();
    let value : Value = serde_json::from_str(VALUE).unwrap();

    println!("Creating Shared Memory Manager...");
    let id = std::process::id();
    let mut shm = SharedMemoryManager::make(id.to_string(), elem_num * elem_size).unwrap();


    let _th = async_std::task::spawn(async move {read_task(rx, elem_size).await});

    let mut rng = rand::thread_rng();
    let mut count = 0;
    loop {
        let mut buff = match shm.alloc(elem_size) {
            Ok(buf) => buf,
            Err(_) => {
                async_std::task::sleep(std::time::Duration::from_millis(SHM_BACKOFF)).await;
                // log::trace!(
                println!(
                    "[SHMSender: {}] After failing allocation the GC collected: {} bytes -- retrying",
                    id,
                    shm.garbage_collect()
                );
                // log::trace!(
                println!(
                    "[SHMSender: {}] Trying to de-fragment memory... De-fragmented {} bytes",
                    id,
                    shm.defragment()
                );
                match shm.alloc(elem_size) {
                    Ok(b) => b,
                    Err(e) => {
                        panic!(
                            "Unable to allocated {} in the shared memory buffer! Error: {:?}",
                            elem_size, e
                        )
                    }
                }
            }
        };

        let mut slice = unsafe { buff.as_mut_slice() };
        serde_json::to_writer(&mut slice, &value).unwrap();
        // Write the data
        let s_pos = rng.gen_range(0..elem_size-16);
        println!("[{}] Sending SHM Data [{}..{}] {:?}", count, s_pos, s_pos+16, &buff.as_slice()[s_pos..s_pos+16]);
        tx.send_async(buff).await?;

//        sleep(Duration::from_millis(250)).await;
        // Dropping the SharedMemoryBuf means to free it.
        count += 1;
    }

}

fn parse_args() -> (usize, usize) {
    let args = App::new("zenoh shared-memory pub example")
        .arg(Arg::from_usage(
            "-s, --shm-element-size=[size]...   'ELEMENT SIZE'",
        ).default_value("1024"))
        .arg(Arg::from_usage(
            "-n, --shm-element-number=[num]...   'NUM OF ELEMENTS'",
        ).default_value("100"))
        .get_matches();

    let elem_num: usize = args.value_of("shm-element-number").unwrap_or(&format!("{N}")).parse().unwrap();
    let elem_size: usize = args.value_of("shm-element-size").unwrap_or(&format!("{ELEM_SIZE}")).parse().unwrap();

    (elem_num, elem_size)
}