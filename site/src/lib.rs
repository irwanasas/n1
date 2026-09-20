pub mod content;
pub mod footer;
pub mod header;
pub mod icons;
pub mod portfolio;
pub mod sections;

use leptos::prelude::*;

use footer::{SiteFooter, WhatsappFloat};
use header::Header;
use sections::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div id="top"></div>
        <Header />

        <HeroSection />
        <PhilosophySection />
        <AboutSection />
        <MissionSection />
        <TechnologySection />
        <WhatWeBuildSection />
        <PortfolioSection />
        <JourneySection />
        <VisionSection />
        <LongTermVisionSection />
        <LlmSection />
        <NationalSecuritySection />
        <ValuesSection />
        <RoadmapSection />
        <PositioningSection />
        <ClosingSection />

        <SiteFooter />
        <WhatsappFloat />
    }
}

#[cfg(feature = "hydrate")]
pub fn hydrate() {
    leptos::mount::hydrate_body(App);
}
