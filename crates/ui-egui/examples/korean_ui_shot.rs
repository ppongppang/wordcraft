//! Offscreen Korean interface smoke check; no desktop interaction is required.
use std::sync::atomic::{AtomicBool, Ordering};
use wordcraft_engine::Session;
use wordcraft_ui_egui::{Services, WordApp};

static READY: AtomicBool = AtomicBool::new(false);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or_else(|| std::io::Error::other("provide output PNG path"))?;
    let mut app = WordApp::new(Session::new(wordcraft_doc::Document::new()), Services::default());
    app.integrated_titlebar = true;
    app.run("ui.language", serde_json::json!({"value": "ko"})).map_err(std::io::Error::other)?;
    app.run("text.insert", serde_json::json!({"text": "한국어 문서 테스트\n안녕하세요. 한글 메뉴와 문서 표시를 확인합니다.\n가나다라마바사 아자차카타파하"})).map_err(std::io::Error::other)?;
    let mut harness = egui_kittest::Harness::builder()
        .with_size(egui::vec2(1440.0, 900.0))
        .with_pixels_per_point(1.0)
        .with_max_steps(1_000_000)
        .wgpu()
        .build_ui_state(|ui, app: &mut WordApp| {
            if READY.load(Ordering::Relaxed) {
                let ctx = ui.ctx().clone();
                app.logic(&ctx);
                app.ui(ui);
            }
        }, app);
    harness.input_mut().max_texture_side = Some(8192);
    READY.store(true, Ordering::Relaxed);
    for _ in 0..180 {
        let mut raw = std::mem::take(harness.input_mut());
        harness.state_mut().raw_input_hook(&mut raw);
        *harness.input_mut() = raw;
        harness.step();
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
    if harness.state().ui.language != "ko" {
        return Err(std::io::Error::other("Korean language selection was not retained").into());
    }
    harness.render()?.save(&path)?;
    println!("Korean interface rendered: {path}");
    Ok(())
}
