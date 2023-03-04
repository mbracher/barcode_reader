

use std::time::{Duration};
use std::thread::{self};

fn main()  {
    println!("hello");

    thread::sleep(Duration::from_secs(10));

       println!("hooking");
        // let keyboard_handler = KeyboardHandler::new();
        let rx = barcode_reader::install_hook();

        while let Ok(barcode) = rx.recv() {
            println!("evnet: {}", barcode);
        }
        thread::sleep(Duration::from_secs(10));
    

    //keyboard_handler.uninstall();
    println!("should be dropped");
    barcode_reader::uninstall_hook();
    thread::sleep(Duration::from_secs(10));
    println!("done");

}

