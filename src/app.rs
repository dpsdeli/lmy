use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="container">
            <header>
                <h1>"Floater.na"</h1>
                <nav>
                    <a href="#">"About"</a>
                    <a href="#">"Blog"</a>
                </nav>
            </header>
            <main>
                <p>"Welcome to my personal website built with Rust and Leptos!"</p>
            </main>
            <footer>
                <p>"© 2025 My Name. Built with Rust."</p>
            </footer>
        </div>
    }
}
