const HEADER: &'static str = r"
_____                       __      ____                                
/\  __`\          __        /\ \    /\  _`\                              
\ \ \/\ \  __  __/\_\    ___\ \ \/'\\ \ \/\_\    ___   _____   __  __    
\ \ \ \ \/\ \/\ \/\ \  /'___\ \ , < \ \ \/_/_  / __`\/\ '__`\/\ \/\ \   
 \ \ \\'\\ \ \_\ \ \ \/\ \__/\ \ \\`\\ \ \L\ \/\ \L\ \ \ \L\ \ \ \_\ \  
  \ \___\_\ \____/\ \_\ \____\\ \_\ \_\ \____/\ \____/\ \ ,__/\/`____ \ 
   \/__//_/\/___/  \/_/\/____/ \/_/\/_/\/___/  \/___/  \ \ \/  `/___/> \
                                                        \ \_\     /\___/
                                                         \/_/     \/__/ ";
const SEPARATOR: &'static str =
    r"----------------------------------------------------------------------";
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHOR: &'static str = "Jon Fast";

pub fn get_header() -> String {
    String::from(
        HEADER.to_owned()
            + "\n"
            + SEPARATOR
            + "\n"
            + "Version: "
            + VERSION
            + "\n"
            + "Author: "
            + AUTHOR
            + "\n"
            + SEPARATOR,
    )
}
