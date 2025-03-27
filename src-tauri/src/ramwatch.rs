use proc_mem::{ ProcMemError, Process };

#[tauri::command]
pub async fn main() {
    println!("ERM");

    // bool isLoading: "ItTakesTwo.exe", 0x07A07A60, 0x180, 0x2b0, 0x0, 0x458, 0xf9;
    type MemoryType = bool;
    let process_name = "ItTakesTwo.exe";
    let ptr_chain = vec![0x07a07a60, 0x180, 0x2b0, 0x0, 0x458, 0xf9];

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
                    println!("{}", memory);
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
