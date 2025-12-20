use leptos::*;
use leptos_router::*;
use crate::blog::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <div class="container">
                <header>
                    <h1>"Liu Ming-Yu"</h1>
                    <div class="subtitle">"AI Product Engineer"</div>
                    <nav>
                        <A href="/">"Home"</A>
                        <A href="/blog">"Blog"</A>
                        <a href="https://github.com/dpsdeli">"Github"</a>
                    </nav>
                </header>
                <main>
                    <Routes>
                        <Route path="/" view=Home/>
                        <Route path="/blog" view=PostList/>
                        <Route path="/blog/:id" view=PostDetail/>
                    </Routes>
                </main>
                <footer>
                    <p>"© 2025 Liu Ming-Yu. Built with Rust & Leptos."</p>
                </footer>
            </div>
        </Router>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="home">
            <p>"Exploring the frontier of Agentic Workflows, LLM integration, and high-performance Software Engineering."</p>
            <p>"My mission is to bridge the gap between advanced AI models and production-ready applications."</p>
        </div>
    }
}
