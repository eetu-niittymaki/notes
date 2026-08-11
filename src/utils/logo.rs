use owo_colors::OwoColorize;

pub fn logo() {
    #[cfg(windows)]
    enable_ansi_support::enable_ansi_support().ok();
    
    const LOGO: &str = 
    r#"                                    
  ███▄▄  ███  ▄██████▄  █████████ ████████ ▄███████▄
  ███▀██▄███ ███▀  ▀███    ███    ███      ███▄▄▄▄  
  ███  ▀▀███ ███    ███    ███    ███▀▀▀    ▀██████▄
  ███    ███ ▀███▄▄███▀    ███    ███▄▄▄▄▄ ▄▄▄▄▄▄███
  ▀▀▀    ▀▀▀   ▀▀▀▀▀▀      ▀▀▀    ▀▀▀▀▀▀▀▀  ▀▀▀▀▀▀▀   
"#;
    println!("\n\n\n\n\n{}", LOGO.fg_rgb::<0x2E, 0x31, 0x92>().bold());
}

