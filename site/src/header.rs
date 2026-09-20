use leptos::prelude::*;

const NAV_LINKS: &[(&str, &str)] = &[
    ("#philosophy", "Filosofi"),
    ("#about", "Tentang"),
    ("#technology", "Fokus Teknologi"),
    ("#portfolio", "Portofolio"),
    ("#vision", "Visi"),
    ("#contact", "Kontak"),
];

#[component]
pub fn Header() -> impl IntoView {
    let theme = RwSignal::new("light");
    let menu_open = RwSignal::new(false);

    #[cfg(target_arch = "wasm32")]
    {
        Effect::new(move |_| {
            if let Some(html) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
                let is_dark = html.get_attribute("data-theme").as_deref() == Some("dark");
                theme.set(if is_dark { "dark" } else { "light" });
            }
        });
    }

    let toggle_theme = move |_| {
        let next = if theme.get() == "light" { "dark" } else { "light" };
        theme.set(next);
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(html) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
                let _ = html.set_attribute("data-theme", next);
            }
            if let Some(storage) = web_sys::window().and_then(|w| w.local_storage().ok().flatten()) {
                let _ = storage.set_item("n1-theme", next);
            }
        }
    };

    let nav_items = || {
        NAV_LINKS
            .iter()
            .map(|(href, label)| view! { <li><a href=*href>{*label}</a></li> })
            .collect::<Vec<_>>()
    };

    let mobile_nav_items = move || {
        NAV_LINKS
            .iter()
            .map(|(href, label)| {
                view! {
                    <a href=*href on:click=move |_| menu_open.set(false)>{*label}</a>
                }
            })
            .collect::<Vec<_>>()
    };

    view! {
        <header class="site-header">
            <div class="container">
                <a class="brand" href="#top">
                    <img src="logo.png" alt="N-1 Labs" width="34" height="34" />
                    "N-1\u{00a0}LABS"
                </a>

                <nav>
                    <ul class="nav-links">{nav_items()}</ul>
                </nav>

                <a class="btn btn-primary header-cta" href="#portfolio">"Lihat Portofolio"</a>

                <div class="header-actions">
                    <button
                        type="button"
                        class="icon-btn"
                        aria-label=move || if theme.get() == "light" { "Aktifkan tema gelap" } else { "Aktifkan tema terang" }
                        on:click=toggle_theme
                    >
                        {move || if theme.get() == "light" {
                            view! {
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79Z" />
                                </svg>
                            }.into_any()
                        } else {
                            view! {
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <circle cx="12" cy="12" r="4" />
                                    <path d="M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41" />
                                </svg>
                            }.into_any()
                        }}
                    </button>
                    <button
                        type="button"
                        class="icon-btn menu-toggle"
                        aria-label=move || if menu_open.get() { "Tutup menu" } else { "Buka menu" }
                        aria-expanded=move || menu_open.get().to_string()
                        on:click=move |_| menu_open.update(|open| *open = !*open)
                    >
                        {move || if menu_open.get() {
                            view! {
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M18 6 6 18M6 6l12 12" />
                                </svg>
                            }.into_any()
                        } else {
                            view! {
                                <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <path d="M4 7h16M4 12h16M4 17h16" />
                                </svg>
                            }.into_any()
                        }}
                    </button>
                </div>
            </div>

            <div class="mobile-nav-wrap" class:open=move || menu_open.get()>
                <nav class="mobile-nav">
                    {mobile_nav_items}
                    <a class="btn btn-primary mobile-nav-cta" href="#portfolio" on:click=move |_| menu_open.set(false)>
                        "Lihat Portofolio"
                    </a>
                </nav>
            </div>
        </header>
    }
}
