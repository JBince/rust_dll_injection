use std::thread::sleep;
use dll_syringe::{Syringe, process::OwnedProcess};

fn main() {

    // Find the target process
    let target_process = OwnedProcess::find_first_by_name("Notepad.exe").unwrap();
    print!("Found target process: {:?}", target_process);


    // Create a syringe for the target process
    let syringe = Syringe::for_process(target_process);

    // Inject the DLL into the target process
    let injected_payload = syringe.inject("target/debug/injection_dll.dll").unwrap();
    println!("Injected payload: {:?}", injected_payload);

    sleep(std::time::Duration::from_secs(10));

    println!("Ejecting DLL");
    syringe.eject(injected_payload).unwrap();


}
