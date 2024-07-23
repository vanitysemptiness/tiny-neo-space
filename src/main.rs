mod app;

#[macroquad::main("tiny-neo-space")]
async fn main() {
    app::run().await;
}