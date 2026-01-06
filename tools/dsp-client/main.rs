#![feature(ptr_metadata)]
#![feature(string_from_utf8_lossy_owned)]
#![feature(addr_parse_ascii)]

use std::{
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
    time::Duration,
};

#[repr(C, packed)]
struct Package {
    cmd: u8,
    len: u32,
    trailing: [u8],
}

#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Debug)]
#[repr(C, packed)]
struct ResultData {
    gpr: [u16; 32],
}

struct Output {
    name: String,
    state: Vec<ResultData>,
}

fn main() -> anyhow::Result<()> {
    let adr = std::env::args().nth(1).expect("expected address argument");
    let output_dir = PathBuf::from(
        std::env::args()
            .nth(2)
            .unwrap_or("../tests/dsp/results".to_string()),
    );
    let input_bin_dir = std::env::args()
        .nth(3)
        .unwrap_or("../tests/dsp/apps".to_string());

    let mut bins = vec![];
    for file in std::fs::read_dir(input_bin_dir)? {
        let file = file?;
        if !file.file_type()?.is_file() {
            continue;
        }
        if file.path().extension().and_then(|s| s.to_str()) == Some("bin") {
            println!("{}", file.path().display());
            bins.push(std::fs::read(file.path())?);
        }
    }

    let bins_len = u32::try_from(bins.len()).unwrap();
    let mut bins = bins.into_iter();

    let mut socket = TcpStream::connect_timeout(
        &std::net::SocketAddr::parse_ascii(adr.as_bytes()).expect("invalid ip address"),
        Duration::from_secs(5),
    )?;
    socket.set_nonblocking(false)?;
    while let Err(e) = socket.set_read_timeout(Some(Duration::from_secs(3))) {
        match e.kind() {
            std::io::ErrorKind::WouldBlock => continue,
            _ => anyhow::bail!("failed to read from socket: {e}"),
        }
    }

    let mut read = vec![0; u16::MAX as usize];
    let mut recv_ctr = 0;
    let mut cur_ctr = 0;
    let mut cur_len = 0;
    let mut cur_name = "".to_string();
    let mut outputs = vec![];
    let mut results = vec![];
    loop {
        recv_ctr += socket.read(&mut read[recv_ctr..]).expect("failed to read");

        if recv_ctr < 5 {
            continue;
        }

        let package = unsafe {
            core::ptr::from_raw_parts::<Package>(read.as_ptr() as *const _, read.len() - 5)
                .as_ref()
                .unwrap()
        };

        if recv_ctr < u32::from_be(package.len) as usize {
            continue;
        }

        match package.cmd {
            // Begin new fuzzing test
            0x00 => {
                if cur_ctr != cur_len {
                    panic!("mismatched received test cases!");
                }
                if cur_len != 0 {
                    outputs.push(Output {
                        name: core::mem::take(&mut cur_name),
                        state: core::mem::take(&mut results),
                    });
                }

                cur_ctr = 0;
                cur_len = u32::from_be_bytes(package.trailing[0..4].try_into().unwrap());
                cur_name = String::from_utf8_lossy_owned(
                    package.trailing[4..]
                        .iter()
                        .take_while(|&&b| b != b'\0')
                        .cloned()
                        .collect(),
                );

                println!("new fuzzing test! '{cur_name}', cases: {cur_len}");
            }
            // Send test result
            0x01 => {
                let cnt = (u32::from_be(package.len) - 5) / size_of::<ResultData>() as u32;
                for i in 0..cnt as usize {
                    let result_data: &ResultData = bytemuck::from_bytes(
                        &package.trailing
                            [i * size_of::<ResultData>()..(i + 1) * size_of::<ResultData>()],
                    );
                    let mut result_data = *result_data;
                    for f in 0..32 {
                        result_data.gpr[f] = u16::from_be(result_data.gpr[f]);
                    }

                    results.push(result_data);
                }
                cur_ctr += cnt;
                println!("{cur_name} progress! {cur_ctr}/{cur_len}");
            }
            // request the next task
            0x80 => {
                if let Some(bin) = bins.next() {
                    socket.write_all(&u32::try_from(bin.len()).unwrap().to_be_bytes())?;
                    socket.write_all(&bin)?;
                } else {
                    // If no task left, signal that by returning a size of 0.
                    socket.write_all(&0u32.to_be_bytes())?;
                }
            }
            // request total amount of tasks
            0x90 => {
                socket.write_all(&bins_len.to_be_bytes())?;
            }
            // quit command
            0xff => {
                println!("done!");
                break;
            }
            cmd => panic!("invalid command! {cmd:#x}"),
        }

        let to_drain = u32::from_be(package.len);

        recv_ctr -= to_drain as usize;
        read.rotate_left(to_drain as usize);
    }

    outputs.push(Output {
        name: core::mem::take(&mut cur_name),
        state: core::mem::take(&mut results),
    });

    std::fs::remove_dir_all(&output_dir)?;
    std::fs::create_dir(&output_dir)?;

    for output in outputs {
        let mut output_bin = vec![];
        for state in output.state {
            for gpr in state.gpr {
                output_bin.extend(gpr.to_be_bytes());
            }
        }

        let output_file = output_dir.join(format!("{}.result.bin", output.name));
        println!("outputting to file '{}'", output_file.display());
        std::fs::write(output_file, output_bin)?;
    }

    Ok(())
}
