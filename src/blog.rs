use leptos::*;
use leptos_router::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub date: String,
    pub content: String,
    pub summary: String,
    pub category: String,
}

pub fn get_all_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            id: "agentic-workflows-2025".to_string(),
            title: "Mastering Agentic Workflows in Production".to_string(),
            date: "2025-12-15".to_string(),
            category: "Artificial Intelligence".to_string(),
            summary: "Transitioning from simple LLM calls to autonomous, self-correcting agent systems.".to_string(),
            content: r#"
                <h2>The Shift to Autonomy</h2>
                <p>In 2025, the conversation has moved beyond "how to prompt" to "how to architect agents". An agentic workflow isn't just a single query; it's a closed-loop system capable of planning, executing tools, and reflecting on its own output.</p>
                <p>Key components include:</p>
                <ul>
                    <li><strong>Tool Use:</strong> Giving LLMs access to APIs and local code execution.</li>
                    <li><strong>Self-Correction:</strong> Implementing loops that verify output against constraints.</li>
                    <li><strong>Multi-Agent Orchestration:</strong> Dividing complex tasks among specialized agents.</li>
                </ul>
            "#.to_string(),
        },
        BlogPost {
            id: "rust-for-ai-infrastructure".to_string(),
            title: "Why Rust is Dominating AI Infrastructure".to_string(),
            date: "2025-11-28".to_string(),
            category: "Engineering".to_string(),
            summary: "Exploring why memory safety and zero-cost abstractions are critical for the next generation of AI tools.".to_string(),
            content: r#"
                <h2>Performance Meets Safety</h2>
                <p>While Python remains the king of data science, the infrastructure that powers AI—vector databases, inference engines, and data loaders—is increasingly being rewritten in Rust.</p>
                <p>Projects like <code>candle</code> (minimalist ML framework) and <code>qdrant</code> demonstrate that we can have Python-like flexibility with C++ performance, all while maintaining strict memory safety.</p>
            "#.to_string(),
        },
        BlogPost {
            id: "prompt-engineering-is-coding".to_string(),
            title: "Prompt Engineering is Software Engineering".to_string(),
            date: "2025-10-10".to_string(),
            category: "Development".to_string(),
            summary: "Treating prompts as first-class code citizens: versioning, testing, and CI/CD for LLM instructions.".to_string(),
            content: r#"
                <h2>Prompts as Code</h2>
                <p>Stop hardcoding prompts. As LLMs become integrated into complex systems, prompt management must mirror software development best practices.</p>
                <p>We need unit tests for prompt accuracy, semantic versioning for system instructions, and automated evaluation pipelines to prevent regression when switching models.</p>
            "#.to_string(),
        },
    ]
}

pub fn get_post(id: &str) -> Option<BlogPost> {
    get_all_posts().into_iter().find(|p| p.id == id)
}

#[component]
pub fn PostList() -> impl IntoView {
    let posts = get_all_posts();
    view! {
        <div class="post-list">
            <div class="posts">
                {posts.into_iter().map(|post| view! {
                    <A href=format!("/blog/{}", post.id) class="post-item-link">
                        <div class="post-item">
                            <span class="date">{post.date} " • " {post.category.clone()}</span>
                            <h3>{post.title.clone()}</h3>
                            <p>{post.summary.clone()}</p>
                        </div>
                    </A>
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
pub fn PostDetail() -> impl IntoView {
    let params = use_params_map();
    let post = move || {
        let id = params.with(|p| p.get("id").cloned().unwrap_or_default());
        get_post(&id)
    };

    view! {
        <div class="post-detail">
            {move || match post() {
                Some(p) => view! {
                    <article>
                        <span class="meta">{p.date.clone()} " • " {p.category.clone()}</span>
                        <h1>{p.title.clone()}</h1>
                        <div class="content" inner_html=p.content.clone()></div>
                        <div class="back-link">
                            <A href="/blog">"← All Articles"</A>
                        </div>
                    </article>
                }.into_view(),
                None => view! { <p>"Post not found."</p> }.into_view()
            }}
        </div>
    }
}
