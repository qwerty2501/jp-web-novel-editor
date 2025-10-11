mod app;
mod menu;
mod novel;

fn main() {
    leptos::mount::mount_to_body(app::App)
}
