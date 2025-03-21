use proc_mem::{ ProcMemError, Process };

#[tauri::command]
pub async fn main() {
    println!("ERM");
    let test: Result<Process, ProcMemError> = Process::with_name("ItTakesTwo.exe");
    loop {
        if let Ok(ref paul) = test {
            let p: Result<bool, ProcMemError> = paul.read_mem::<bool>(
                // bool isLoading: "ItTakesTwo.exe", 0x07A07A60, 0x180, 0x2b0, 0x0, 0x458, 0xf9;
                paul
                    .read_ptr_chain(
                        vec![paul.process_base_address, 0x07a07a60, 0x180, 0x2b0, 0x0, 0x458, 0xf9]
                    )
                    .unwrap()
            );

            if let Ok(craig) = p {
                println!("{}", craig);
            }
        }
    }
}
