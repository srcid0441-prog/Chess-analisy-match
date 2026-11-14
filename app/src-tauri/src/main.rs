// Capivara para computador: uma janela nativa (Rust + Tauri) com o site dentro.
// Esconde o terminal preto no Windows quando o app é gerado em modo release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{WebviewUrl, WebviewWindowBuilder};

// Links "abrir em nova aba" (ex.: ver a partida no Chess.com) viram navegação normal,
// que o Rust intercepta abaixo e manda para o navegador do computador.
const LINKS_EXTERNOS: &str = r#"
document.addEventListener('click', function (e) {
  var a = e.target && e.target.closest ? e.target.closest('a[href]') : null;
  if (!a || a.target !== '_blank') return;
  if (/^https?:/i.test(a.href)) { e.preventDefault(); location.href = a.href; }
}, true);
"#;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            WebviewWindowBuilder::new(app, "principal", WebviewUrl::App("index.html".into()))
                .title("Capivara")
                .inner_size(1400.0, 900.0)
                .min_inner_size(900.0, 600.0)
                .center()
                .theme(Some(tauri::Theme::Dark))
                .initialization_script(LINKS_EXTERNOS)
                .on_navigation(|url| {
                    // páginas do próprio app continuam dentro da janela
                    let local =
                        matches!(url.scheme(), "tauri" | "asset" | "about" | "blob" | "data")
                            || matches!(
                                url.host_str(),
                                Some("tauri.localhost") | Some("localhost")
                            );
                    if local {
                        return true;
                    }
                    // qualquer site de fora abre no navegador do computador
                    let _ = open::that(url.as_str());
                    false
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("não deu para abrir o Capivara");
}
