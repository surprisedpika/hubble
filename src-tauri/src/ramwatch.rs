use proc_mem::{ ProcMemError, Process };

#[tauri::command]
pub async fn main() {
    println!("ERM");
    let process_res: Result<Process, ProcMemError> = Process::with_name("ItTakesTwo.exe");
    loop {
        if let Ok(ref process) = process_res {
            let memory_res: Result<bool, ProcMemError> = process.read_mem::<bool>(
                // bool isLoading: "ItTakesTwo.exe", 0x07A07A60, 0x180, 0x2b0, 0x0, 0x458, 0xf9;
                process
                    .read_ptr_chain(
                        vec![
                            process.process_base_address,
                            0x07a07a60,
                            0x180,
                            0x2b0,
                            0x0,
                            0x458,
                            0xf9
                        ]
                    )
                    .unwrap()
            );

            if let Ok(memory) = memory_res {
                println!("{}", memory);
            }
        }
    }
}
