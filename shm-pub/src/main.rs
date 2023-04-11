
use clap::{App, Arg};
use rand::Rng;
use serde_json;
use serde_json::Value;
use zenoh::shm::SharedMemoryManager;
use zenoh::config::Config;
use zenoh::prelude::r#async::*;
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

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initiate logging
    env_logger::init();

    let (config, path, elem_num, elem_size) = parse_args();

    let value : Value = serde_json::from_str(VALUE).unwrap();

    println!("Opening session...");
    let session = zenoh::open(config).res().await.unwrap();

    println!("Creating Shared Memory Manager...");
    let id = session.zid();
    let mut shm = SharedMemoryManager::make(id.to_string(), elem_num * elem_size).unwrap();

    println!("Allocating Shared Memory Buffer...");
    let publisher = session.declare_publisher(&path).res().await.unwrap();

    let mut rng = rand::thread_rng();
    let mut count = 0;
    loop {
        let mut buff = match shm.alloc(elem_size) {
            Ok(buf) => buf,
            Err(_) => {
                async_std::task::sleep(std::time::Duration::from_millis(SHM_BACKOFF)).await;
                // log::trace!(
                println!(
                    "[ZenohSender: {}] After failing allocation the GC collected: {} bytes -- retrying",
                    id,
                    shm.garbage_collect()
                );
                // log::trace!(
                println!(
                    "[ZenohSender: {}] Trying to de-fragment memory... De-fragmented {} bytes",
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
        println!("[{}] Put SHM Data ('{}') [{}..{}] {:?}", count, path, s_pos, s_pos+16, &buff.as_slice()[s_pos..s_pos+16]);
        publisher.put(buff).res().await?;

//        sleep(Duration::from_millis(250)).await;
        // Dropping the SharedMemoryBuf means to free it.
        count += 1;
    }

}

fn parse_args() -> (Config, String, usize, usize) {
    let args = App::new("zenoh shared-memory pub example with bug")
        .arg(
            Arg::from_usage("-m, --mode=[MODE] 'The zenoh session mode (peer by default).")
                .possible_values(["peer", "client"]),
        )
        .arg(Arg::from_usage(
            "-e, --connect=[ENDPOINT]...  'Endpoints to connect to.'",
        ))
        .arg(Arg::from_usage(
            "-l, --listen=[ENDPOINT]...   'Endpoints to listen on.'",
        ))
        .arg(
            Arg::from_usage("-p, --path=[PATH]        'The key expression to publish onto.'")
                .default_value("demo/example/zenoh-rs-pub"),
        )
        .arg(Arg::from_usage(
            "-c, --config=[FILE]      'A configuration file.'",
        ))
        .arg(Arg::from_usage(
            "-s, --shm-element-size=[size]...   'ELEMENT SIZE'",
        ).default_value("1024"))
        .arg(Arg::from_usage(
            "-n, --shm-element-number=[num]...   'NUM OF ELEMENTS'",
        ).default_value("100"))
        .arg(Arg::from_usage(
            "--no-multicast-scouting 'Disable the multicast-based scouting mechanism.'",
        ))
        .get_matches();

    let mut config = if let Some(conf_file) = args.value_of("config") {
        Config::from_file(conf_file).unwrap()
    } else {
        Config::default()
    };
    if let Some(Ok(mode)) = args.value_of("mode").map(|mode| mode.parse()) {
        config.set_mode(Some(mode)).unwrap();
    }
    if let Some(values) = args.values_of("connect") {
        config
            .connect
            .endpoints
            .extend(values.map(|v| v.parse().unwrap()))
    }
    if let Some(values) = args.values_of("listen") {
        config
            .listen
            .endpoints
            .extend(values.map(|v| v.parse().unwrap()))
    }
    if args.is_present("no-multicast-scouting") {
        config.scouting.multicast.set_enabled(Some(false)).unwrap();
    }

    let path = args.value_of("path").unwrap();
    let elem_num: usize = args.value_of("shm-element-number").unwrap_or(&format!("{N}")).parse().unwrap();
    let elem_size: usize = args.value_of("shm-element-size").unwrap_or(&format!("{ELEM_SIZE}")).parse().unwrap();

    (config, path.to_string(), elem_num, elem_size)
}