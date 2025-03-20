use proc_mem::{ ProcMemError, Process };

#[tauri::command]
pub async fn main() {
    println!("ERM");
    let test: Result<Process, ProcMemError> = Process::with_name("SplitFiction.exe");
    loop {
        if let Ok(ref paul) = test {
            let p: Result<u32, ProcMemError> = paul.read_mem::<u32>(0x161e5cb89b8);
            if let Ok(craig) = p {
                println!("{}", craig);
            }
        }
    }
}
