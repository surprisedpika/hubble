use proc_mem::{ ProcMemError, Process };

#[tauri::command]
pub async fn main() {
    println!("ERM");

    let process_name = "ItTakesTwo.exe";

    // bool isLoading: "ItTakesTwo.exe", 0x07A07A60, 0x180, 0x2b0, 0x0, 0x458, 0xf9;
    // type MemoryType = bool;
    // let ptr_chain = vec![0x07a07a60, 0x180, 0x2b0, 0x0, 0x458, 0xf9];

    // byte skippable: "ItTakesTwo.exe", 0x07A07A60, 0x180, 0x2b0, 0x0, 0x390, 0x318;
    // type MemoryType = u8;
    // let ptr_chain = vec![0x07a07a60, 0x180, 0x2b0, 0x0, 0x390, 0x318];

    // string255 chapterString: "ItTakesTwo.exe", 0x07A07A60, 0x180, 0x368, 0x8, 0x1e8, 0x0;
    type MemoryType = [u8; 32];
    let ptr_chain = vec![0x07a07a60, 0x180, 0x368, 0x8, 0x1e8, 0x0];

    let process_res: Result<Process, ProcMemError> = Process::with_name(process_name);
    loop {
        if let Ok(ref process) = process_res {
            let ptr_chain_res = process.read_ptr_chain({
                let mut chain = vec![process.process_base_address];
                chain.extend(ptr_chain.clone());
                chain
            });

            if let Ok(ptr_chain) = ptr_chain_res {
                let memory_res: Result<MemoryType, ProcMemError> = process.read_mem::<MemoryType>(
                    ptr_chain
                );

                if let Ok(memory) = memory_res {
                    // println!("{:?}", memory);
                    for bytes in memory.chunks(2) {
                        if bytes.len() != 2 {
                            continue;
                        }

                        let character = ((bytes[1] as u16) << 8) | (bytes[0] as u16);

                        if character == 0 {
                            break;
                        }

                        if let Some(letter) = char::from_u32(character as u32) {
                            print!("{}", letter);
                        } else {
                            break;
                        }
                    }

                    print!("\n");
                } else {
                    println!("Error reading memory!");
                }
            } else {
                println!("Error in pointer chain!");
            }
        } else {
            println!("Error in reading process!");
        }
    }
}
