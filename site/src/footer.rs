use leptos::prelude::*;

use crate::content::{mailto_url, whatsapp_url, CONTACT};
use crate::icons::WhatsAppIcon;

#[component]
pub fn SiteFooter() -> impl IntoView {
    let whatsapp_href = whatsapp_url();
    let mailto_href = mailto_url();

    view! {
        <footer class="site-footer">
            <div class="container footer-contact">
                <span class="footer-contact-label">"Hubungi Kami"</span>
                <div class="footer-contact-links">
                    <a class="btn btn-ghost footer-whatsapp" href=whatsapp_href.clone() target="_blank" rel="noopener noreferrer">
                        <WhatsAppIcon size=16 />
                        {CONTACT.whatsapp_display}
                    </a>
                    <a class="footer-email" href=mailto_href.clone()>{CONTACT.email}</a>
                </div>
            </div>
            <div class="container">
                <a class="brand" href="#top">
                    <img src="logo.png" alt="N-1 Labs" width="26" height="26" />
                    "N-1 LABS"
                </a>
                <span class="footer-tagline">"N-1 · always one step from finished."</span>
                <span>"© 2026 N-1 Labs. All rights reserved."</span>
            </div>
        </footer>
    }
}

#[component]
pub fn WhatsappFloat() -> impl IntoView {
    let whatsapp_href = whatsapp_url();
    let label = format!("Chat via WhatsApp ke {}", CONTACT.whatsapp_display);

    view! {
        <a class="whatsapp-float" href=whatsapp_href target="_blank" rel="noopener noreferrer" aria-label=label>
            <WhatsAppIcon size=28 />
        </a>
    }
}
