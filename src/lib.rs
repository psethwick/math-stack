use std::net::TcpListener;
mod app;

fn available_port() -> Option<u16> {
    (9000..9100).find(|port| TcpListener::bind(("127.0.0.1", *port)).is_ok())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let port = available_port().expect("did not manage to find a port for webserve");
    tauri::async_runtime::spawn(async move {
        if let Err(e) = app::serve(port).await {
            eprintln!("Server error: {}", e);
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(move |app| {
            let _window = tauri::WebviewWindowBuilder::new(
                app,
                "math",
                tauri::WebviewUrl::App(format!("http://127.0.0.1:{}", port).into()),
            )
            .title("MATH")
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
