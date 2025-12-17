#![feature(ptr_metadata)]
#![feature(string_from_utf8_lossy_owned)]

use std::{io::Read, net::TcpStream};

#[repr(C, packed)]
struct Package {
    cmd: u8,
    len: u32,
    trailing: [u8],
}

#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
#[repr(C, packed)]
struct ResultData {
    gpr: [u16; 32],
}

fn main() {
    let adr = std::env::args().nth(1).expect("expected address argument");

    let mut receiver = TcpStream::connect(adr).expect("failed to establish socket");

    let mut read = vec![0; u16::MAX as usize];
    let mut recv_ctr = 0;
    let mut cur_ctr = 0;
    let mut cur_len = 0;
    let mut cur_name = "".to_string();
    loop {
        recv_ctr += receiver
            .read(&mut read[recv_ctr..])
            .expect("failed to read");

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
                let mut results = vec![];
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
            // Skip test
            0x02 => {
                println!("skipped!");
                cur_ctr = 0;
                cur_len = 0;
                cur_name = "".to_string();
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
}
