use std::io::Write;

pub fn read_string(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap_or_else(|_| println!("Erro de flush, continuando..."));
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Erro de leitura");
    
     buffer.trim().to_string()
}
