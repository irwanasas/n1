use leptos::prelude::*;

use crate::content::*;
use crate::icons::{BrandIcon, EmailIcon, GITHUB_PATH, INSTAGRAM_PATH, TIKTOK_PATH, WHATSAPP_PATH, YOUTUBE_PATH};
use crate::portfolio::Portfolio;

#[component]
pub fn HeroSection() -> impl IntoView {
    let whatsapp_href = whatsapp_url();
    let mailto_href = mailto_url();

    view! {
        <section class="hero">
            <div class="container">
                <div class="hero-eyebrow">"Company Profile · 2026"</div>
                <div class="progress-badge" aria-hidden="true">
                    <span class="progress-track">
                        <span class="progress-fill"></span>
                    </span>
                    <span class="progress-label">"N-1 · Always One Short"</span>
                </div>
                <h1>
                    "Building Indonesia's Own " <span>"AI Technology"</span> " Capability"
                </h1>
                <p>
                    "N-1 Labs adalah perusahaan teknologi AI asal Indonesia yang membangun AI Agent, software
                    systems, automation, dan digital products — dengan arah jangka panjang menjadi AI technology
                    company yang membangun teknologinya sendiri."
                </p>
                <div class="hero-ctas">
                    <a class="btn btn-primary" href="#portfolio">"Lihat Portofolio"</a>
                    <a class="btn btn-ghost" href="#vision">"Pelajari Visi Kami"</a>
                </div>
                <div class="hero-stats">
                    {STATS.iter().map(|stat| view! {
                        <div class="stat">
                            <b>{stat.value}</b>
                            <span>{stat.label}</span>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
                <div class="hero-socials">
                    <a class="hero-social-link" href="https://github.com/n-1repo" aria-label="GitHub" target="_blank" rel="noopener noreferrer">
                        <BrandIcon path=GITHUB_PATH />
                    </a>
                    <a class="hero-social-link" href="https://instagram.com/" aria-label="Instagram" target="_blank" rel="noopener noreferrer">
                        <BrandIcon path=INSTAGRAM_PATH />
                    </a>
                    <a class="hero-social-link" href="https://tiktok.com/" aria-label="TikTok" target="_blank" rel="noopener noreferrer">
                        <BrandIcon path=TIKTOK_PATH />
                    </a>
                    <a class="hero-social-link" href="https://youtube.com/" aria-label="YouTube" target="_blank" rel="noopener noreferrer">
                        <BrandIcon path=YOUTUBE_PATH />
                    </a>
                    <a class="hero-social-link" href=whatsapp_href aria-label="WhatsApp" target="_blank" rel="noopener noreferrer">
                        <BrandIcon path=WHATSAPP_PATH />
                    </a>
                    <a class="hero-social-link" href=mailto_href aria-label="Email">
                        <EmailIcon />
                    </a>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn PhilosophySection() -> impl IntoView {
    view! {
        <section id="philosophy">
            <div class="container">
                <div class="kicker">"00 · Filosofi"</div>
                <h2 class="section-title">"N-1: Selalu Satu Langkah Sebelum Selesai"</h2>
                <div class="divider"></div>

                <p class="section-lead" style="max-width:720px">
                    "Nama kami bukan kebetulan. Dalam matematika, N melambangkan bilangan penuh — hasil akhir,
                    versi sempurna, titik selesai. Kami menamai diri " <b>"N-1"</b> " karena percaya titik itu tidak
                    pernah benar-benar sampai: selalu ada satu hal yang kurang, satu hal yang bisa dibangun,
                    diperbaiki, atau dikembangkan lagi."
                </p>

                <div class="manifesto">
                    <p class="eq">
                        <span class="no">"N-1 = Selesai."</span>
                        <br />
                        <span class="yes">"N-1 = Terus Bergerak."</span>
                    </p>
                </div>

                <p>"Perhatikan dua penanda yang muncul berulang di sepanjang halaman ini:"</p>
                <div class="legend-pair">
                    <span class="tag existing">"Existing Capability"</span>
                    <span class="arrow">"— apa yang sudah kami bangun dan pakai secara nyata"</span>
                </div>
                <div class="legend-pair">
                    <span class="tag vision">"Future Vision"</span>
                    <span class="arrow">"— satu langkah yang selalu menunggu di depan"</span>
                </div>
                <p>
                    "Bagi kami, kedua penanda itu bukan sekadar label kategori — itu adalah N dan N-1: apa yang
                    telah tercapai, dan satu hal yang sengaja kami biarkan belum selesai, karena di situlah kerja
                    berikutnya dimulai."
                </p>

                <div class="subhead">"Bukti, Bukan Sekadar Klaim"</div>
                <div class="evidence-grid">
                    <div class="evidence-card">
                        <h4>"Journey yang Berhenti di \u{201c}Vision\u{201d}"</h4>
                        <p>
                            "Technology Journey kami sengaja berakhir di tahap yang masih berlabel Vision — bukan
                            karena belum selesai ditulis, tapi karena memang belum tercapai."
                        </p>
                        <a href="#journey">"Lihat Technology Journey →"</a>
                    </div>
                    <div class="evidence-card">
                        <h4>"Roadmap Menuju Horizon, Bukan Garis Akhir"</h4>
                        <p>
                            "Development Roadmap kami berakhir di \u{201c}2030+\u{201d} — sebuah horizon terbuka, bukan
                            tenggat yang bisa dicoret selesai."
                        </p>
                        <a href="#roadmap">"Lihat Roadmap →"</a>
                    </div>
                    <div class="evidence-card">
                        <h4>"Enam Proyek, Enam Fondasi"</h4>
                        <p>
                            "Setiap proyek yang telah kami bangun bukan tujuan akhir, melainkan satu lapisan fondasi
                            menuju kapabilitas berikutnya."
                        </p>
                        <a href="#portfolio">"Lihat Project Portfolio →"</a>
                    </div>
                </div>

                <p class="philosophy-closing">
                    "Begitulah cara kami bekerja: N-1 hari ini, mendekati N esok — tanpa pernah benar-benar
                    sampai."
                </p>
            </div>
        </section>
    }
}

#[component]
pub fn AboutSection() -> impl IntoView {
    view! {
        <section id="about">
            <div class="container">
                <div class="kicker">"01 · Tentang Kami"</div>
                <h2 class="section-title">"About N-1 Labs"</h2>
                <div class="divider"></div>

                <p class="section-lead" style="max-width:720px">
                    "N-1 Labs adalah perusahaan teknologi asal Semarang, Jawa Tengah, yang berfokus pada
                    pengembangan Artificial Intelligence, AI Agent, software systems, automation, digital
                    products, dan teknologi komputasi."
                </p>
                <p style="max-width:720px">
                    "Didirikan pada 2 September 2026, N-1 Labs hadir bukan sebagai software house biasa,
                    melainkan sebagai perusahaan yang membangun kapabilitasnya secara bertahap — dimulai dari
                    software engineering, menuju sistem bisnis dan automation, hingga akhirnya pada pengembangan
                    AI Agent dan AI Infrastructure. Setiap proyek yang dikerjakan N-1 Labs dirancang untuk
                    memperkuat fondasi teknis menuju arah tersebut."
                </p>
                <p style="max-width:720px">
                    "Dalam jangka panjang, N-1 Labs memiliki tujuan untuk berkembang menjadi "
                    <b>"AI technology company"</b>
                    " yang mampu membangun teknologi AI-nya sendiri — termasuk
                    kontribusi terhadap kemampuan Indonesia dalam mengembangkan Large Language Model (LLM)
                    secara mandiri."
                </p>

                <div class="subhead" style="margin-top:32px">"Ringkasan Identitas"</div>
                <div class="panel">
                    <table class="identity-table">
                        <tbody>
                            {IDENTITY.iter().map(|row| view! {
                                <tr>
                                    <td>{row.label}</td>
                                    <td>{row.value}</td>
                                </tr>
                            }).collect::<Vec<_>>()}
                        </tbody>
                    </table>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn MissionSection() -> impl IntoView {
    view! {
        <section id="mission">
            <div class="container">
                <div class="kicker">"02 · Cara Kami Mewujudkannya"</div>
                <h2 class="section-title">"Mission"</h2>
                <div class="divider"></div>
                <p class="section-lead">
                    "Misi N-1 Labs disusun berdasarkan tujuh prinsip kerja: Develop, Integrate, Automate,
                    Research, Build, Scale, dan Protect."
                </p>

                <div class="grid grid-2" style="margin-top:16px">
                    {MISSION_PILLARS.iter().map(|pillar| {
                        let style = if pillar.wide { "grid-column:1 / -1" } else { "" };
                        view! {
                            <div class="panel" style=style>
                                <h4 style="margin:0 0 8px 0;font-size:14.5px">{pillar.title}</h4>
                                <p style="margin:0;font-size:13.5px;color:var(--muted)">{pillar.body}</p>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn TechnologySection() -> impl IntoView {
    view! {
        <section id="technology">
            <div class="container">
                <div class="kicker">"03 · Kapabilitas Teknologi"</div>
                <h2 class="section-title">"Technology Focus"</h2>
                <div class="divider"></div>
                <p class="section-lead">
                    "Kapabilitas N-1 Labs dikelompokkan ke dalam enam domain teknologi, dari sistem yang telah
                    dibangun hingga arah riset masa depan."
                </p>

                <div class="grid grid-2" style="margin-top:16px">
                    {TECH_DOMAINS.iter().map(|domain| {
                        let class = if domain.vision { "panel domain-card vision" } else { "panel domain-card" };
                        view! {
                            <div class=class>
                                <h3>
                                    <span class="dot"></span>
                                    {domain.name}
                                    {domain.vision.then(|| view! { <span class="tag vision" style="margin-left:6px">"Vision"</span> })}
                                </h3>
                                <ul>
                                    {domain.items.iter().map(|item| view! { <li>{*item}</li> }).collect::<Vec<_>>()}
                                </ul>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn WhatWeBuildSection() -> impl IntoView {
    view! {
        <section id="what-we-build">
            <div class="container">
                <div class="kicker">"04 · Output Nyata"</div>
                <h2 class="section-title">"What We Build"</h2>
                <div class="divider"></div>
                <p class="section-lead">
                    "N-1 Labs membangun sistem yang digunakan secara nyata — bukan sekadar prototipe atau konsep.
                    Setiap produk dirancang untuk menjadi bagian dari infrastruktur bisnis penggunanya."
                </p>

                <div class="grid grid-2" style="margin-top:16px">
                    {BUILD_AREAS.iter().map(|area| view! {
                        <div class="panel">
                            <h4 style="margin:0 0 8px 0;font-size:14.5px">{area.title}</h4>
                            <p style="margin:0;font-size:13.5px;color:var(--muted)">{area.body}</p>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>

                <div class="subhead" style="margin-top:28px">"Enam Proyek yang Telah Dibangun"</div>
                <div class="panel">
                    <ul class="clean project-list">
                        {PROJECTS.iter().map(|project| view! { <li>{project.title}</li> }).collect::<Vec<_>>()}
                    </ul>
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn PortfolioSection() -> impl IntoView {
    view! {
        <section id="portfolio">
            <div class="container">
                <div class="kicker">"05 · Rekam Jejak"</div>
                <h2 class="section-title">"Project Portfolio"</h2>
                <div class="divider"></div>
                <p class="section-lead">
                    "Enam proyek berikut menggambarkan perjalanan N-1 Labs dalam membangun kapabilitas — dari
                    software engineering, sistem bisnis, automation, hingga AI Agent. Setiap proyek dijelaskan
                    berdasarkan permasalahan, solusi, fungsi, dampak, dan hal yang dipelajari N-1 Labs."
                </p>
                <span class="tag existing">"Existing Capability"</span>

                <Portfolio />
            </div>
        </section>
    }
}

#[component]
pub fn JourneySection() -> impl IntoView {
    let len = JOURNEY.len();
    view! {
        <section id="journey">
            <div class="container">
                <div class="kicker">"06 · Perjalanan Kapabilitas"</div>
                <h2 class="section-title">"Technology Journey"</h2>
                <div class="divider"></div>
                <p class="section-lead">
                    "Proyek-proyek yang telah dikerjakan N-1 Labs bukan kumpulan proyek yang berdiri sendiri,
                    melainkan tahapan yang membangun kapabilitas perusahaan secara bertahap menuju teknologi AI
                    berskala nasional."
                </p>

                <div class="timeline">
                    {JOURNEY.iter().enumerate().map(|(i, step)| {
                        let dot_class = if step.vision { "tdot vision" } else { "tdot" };
                        view! {
                            <div class="tstep">
                                <div class="tline">
                                    <div class=dot_class></div>
                                    {(i < len - 1).then(|| view! { <div class="tbar"></div> })}
                                </div>
                                <div class="tbody">
                                    <h4>
                                        {step.title}
                                        {step.vision.then(|| view! { <span class="tag vision">"Vision"</span> })}
                                    </h4>
                                    <p>{step.body}</p>
                                </div>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
                <p class="philosophy-closing">
                    "Delapan tahap ini sengaja berhenti di sebuah Vision, bukan garis akhir — karena bagi N-1,
                    tidak ada garis akhir."
                </p>
            </div>
        </section>
    }
}

#[component]
pub fn VisionSection() -> impl IntoView {
    let flow_len = POSITIONING_FLOW.len();
    view! {
        <section id="vision">
            <div class="container">
                <div class="kicker">"07 · Arah Perusahaan"</div>
                <h2 class="section-title">"Vision"</h2>
                <div class="divider"></div>

                <div class="quote-block">
                    <p>
                        "\u{201c}Membangun teknologi Artificial Intelligence Indonesia yang mandiri, berdaulat, dan
                        mampu digunakan untuk kebutuhan masyarakat, industri, serta kepentingan strategis
                        nasional.\u{201d}"
                    </p>
                </div>

                <p>
                    "Visi ini menjadi arah jangka panjang N-1 Labs, yang secara bertahap dijabarkan ke dalam
                    beberapa fokus pengembangan:"
                </p>
                <ul class="clean">
                    {VISION_POINTS.iter().map(|point| view! { <li>{*point}</li> }).collect::<Vec<_>>()}
                </ul>

                <div class="subhead">"Positioning Arah Perusahaan"</div>
                <div class="panel flow-chain">
                    {POSITIONING_FLOW.iter().enumerate().map(|(i, step)| view! {
                        <div class="step" style="display:flex;align-items:center;gap:10px;flex:1 1 auto">
                            <span>{*step}</span>
                            {(i < flow_len - 1).then(|| view! { <span class="arrow">"→"</span> })}
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
                <p style="font-size:12px;color:var(--muted);margin-top:12px">
                    "N-1 Labs tidak memposisikan dirinya sebagai penyedia chatbot maupun software development
                    semata, melainkan sebagai perusahaan yang membangun jenjang kapabilitas menuju teknologi AI
                    berskala nasional."
                </p>
            </div>
        </section>
    }
}

#[component]
pub fn LongTermVisionSection() -> impl IntoView {
    view! {
        <section id="long-term-vision">
            <div class="container">
                <div class="kicker">"08 · Masa Depan"</div>
                <h2 class="section-title">"Long-Term Vision"</h2>
                <div class="divider"></div>
                <span class="tag vision">"Future Vision / Aspiration"</span>

                <div class="quote-block" style="margin-top:16px">
                    <p>
                        "\u{201c}Today we build AI applications. Tomorrow we build AI infrastructure. Eventually, we
                        aim to contribute to Indonesia's own AI foundation models.\u{201d}"
                    </p>
                </div>

                <p>
                    "N-1 Labs saat ini fokus membangun AI Agent dan sistem berbasis AI untuk kebutuhan bisnis.
                    Namun, arah jangka panjang perusahaan mengarah pada kontribusi terhadap kapabilitas Indonesia
                    dalam membangun teknologi AI-nya sendiri — termasuk Large Language Model (LLM). Tahapan
                    konseptualnya dijelaskan lebih detail di bagian berikut."
                </p>
            </div>
        </section>
    }
}

#[component]
pub fn LlmSection() -> impl IntoView {
    view! {
        <section id="llm">
            <div class="container">
                <div class="kicker">"09 · Kedaulatan Teknologi"</div>
                <h2 class="section-title">"Building Indonesia's Own LLM"</h2>
                <div class="divider"></div>
                <span class="tag vision">"Future Vision / Aspiration"</span>

                <p style="margin-top:16px">
                    "Salah satu tujuan jangka panjang N-1 Labs adalah berkontribusi terhadap kemampuan Indonesia
                    dalam membangun Large Language Model (LLM) sendiri — sebagai bagian dari kedaulatan teknologi
                    bangsa di bidang kecerdasan buatan."
                </p>
                <p>
                    <b>"N-1 Labs saat ini belum memiliki LLM sendiri."</b>
                    " Kapabilitas yang dimiliki saat ini
                    berada pada tahap membangun AI Agent dan aplikasi berbasis AI. Pengembangan menuju model
                    engineering dan foundation model merupakan arah jangka panjang yang akan dibangun secara
                    bertahap."
                </p>

                <div class="subhead">"Mengapa Ini Penting"</div>
                <ul class="clean">
                    {LLM_REASONS.iter().map(|reason| view! { <li>{*reason}</li> }).collect::<Vec<_>>()}
                </ul>

                <div class="subhead">"Tahapan Menuju Kemampuan LLM"</div>
                <div class="grid grid-2">
                    {LLM_STAGES.iter().map(|stage| view! {
                        <div class="panel">
                            <h4 style="margin:0 0 6px 0;font-size:13.5px">{stage.title}</h4>
                            <p style="margin:0;font-size:12.5px;color:var(--muted)">{stage.body}</p>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn NationalSecuritySection() -> impl IntoView {
    view! {
        <section id="national-security">
            <div class="container">
                <div class="kicker">"10 · Kontribusi Strategis"</div>
                <h2 class="section-title">"National Security & Defense Technology"</h2>
                <div class="divider"></div>
                <span class="tag vision">"Future Vision / Aspiration"</span>

                <p style="margin-top:16px">
                    "N-1 Labs memiliki aspirasi jangka panjang untuk bekerja sama dengan institusi pemerintah,
                    pertahanan, dan militer Indonesia dalam mengembangkan teknologi AI yang dapat mendukung
                    kebutuhan strategis nasional, di antaranya:"
                </p>

                <div class="grid grid-2">
                    {SECURITY_AREAS.iter().map(|area| view! {
                        <div class="panel">
                            <p style="margin:0;font-size:13.5px">{*area}</p>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>

                <div class="quote-block" style="margin-top:20px">
                    <p>
                        "\u{201c}Mengembangkan teknologi AI yang dapat menjadi bagian dari technological capability
                        Indonesia dalam menjaga keamanan dan kedaulatan negara.\u{201d}"
                    </p>
                </div>

                <p style="font-size:12.5px;color:var(--muted);margin-top:12px">
                    <b style="color:var(--ink)">"Catatan penting:"</b>
                    " N-1 Labs bukan perusahaan militer, dan
                    saat ini belum memiliki kerja sama resmi dengan institusi pertahanan atau militer mana pun.
                    Bagian ini menggambarkan arah dan aspirasi jangka panjang perusahaan, bukan kemitraan yang
                    sudah berjalan."
                </p>
            </div>
        </section>
    }
}

#[component]
pub fn ValuesSection() -> impl IntoView {
    view! {
        <section id="values">
            <div class="container">
                <div class="kicker">"11 · Prinsip Kerja"</div>
                <h2 class="section-title">"Company Values"</h2>
                <div class="divider"></div>
                <p class="section-lead">
                    "Ketujuh prinsip ini adalah cara kami menjaga filosofi N-1 tetap hidup dalam setiap keputusan
                    — termasuk keputusan untuk tidak pernah menyebut sebuah sistem sebagai \u{201c}selesai\u{201d}."
                </p>

                <div class="panel">
                    {VALUES.iter().enumerate().map(|(i, value)| view! {
                        <div class="value-row">
                            <div class="value-num">{format!("{:02}", i + 1)}</div>
                            <div class="value-body">
                                <h4>{value.title}</h4>
                                <p>{value.body}</p>
                            </div>
                        </div>
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn RoadmapSection() -> impl IntoView {
    view! {
        <section id="roadmap">
            <div class="container">
                <div class="kicker">"12 · Jalan ke Depan"</div>
                <h2 class="section-title">"Development Roadmap"</h2>
                <div class="divider"></div>
                <p style="font-size:12.5px;color:var(--muted);margin-bottom:20px">
                    "Roadmap berikut bersifat aspirasional — menggambarkan arah pengembangan, bukan janji atau
                    kepastian waktu."
                </p>

                {ROADMAP_PHASES.iter().map(|phase| view! {
                    <div class="phase">
                        <div class="phase-head">
                            <h4>{phase.phase}</h4>
                            <span class="yr">{phase.year}</span>
                        </div>
                        <div class="phase-body">
                            <ul>
                                {phase.items.iter().map(|item| view! { <li>{*item}</li> }).collect::<Vec<_>>()}
                            </ul>
                        </div>
                    </div>
                }).collect::<Vec<_>>()}
                <p class="philosophy-closing">
                    "Fase terakhir berhenti di \u{201c}2030+\u{201d}, bukan tanggal — sebuah horizon yang terus
                    bergerak seiring kami terus membangun."
                </p>
            </div>
        </section>
    }
}

#[component]
pub fn PositioningSection() -> impl IntoView {
    let len = POSITIONING_CARDS.len();
    view! {
        <section id="positioning">
            <div class="container">
                <div class="kicker">"13 · Cara Kami Diposisikan"</div>
                <h2 class="section-title">"Company Positioning"</h2>
                <div class="divider"></div>

                <div class="quote-block">
                    <p>
                        "\u{201c}An Indonesian AI Technology Company Building Intelligent Systems for Business,
                        Industry, and the Future of National Technology.\u{201d}"
                    </p>
                </div>

                <p class="section-lead">
                    "Positioning inti ini konsisten di seluruh konteks komunikasi, dengan penekanan yang
                    disesuaikan menurut audiens:"
                </p>

                <div class="grid grid-2" style="margin-top:8px">
                    {POSITIONING_CARDS.iter().enumerate().map(|(i, card)| {
                        let style = if i == len - 1 { "grid-column:1 / -1" } else { "" };
                        view! {
                            <div class="pos-card" style=style>
                                <h4>{card.title}</h4>
                                <p>{card.body}</p>
                            </div>
                        }
                    }).collect::<Vec<_>>()}
                </div>
            </div>
        </section>
    }
}

#[component]
pub fn ClosingSection() -> impl IntoView {
    view! {
        <section id="contact" class="closing">
            <div class="container">
                <div class="kicker">"14 · Closing Statement"</div>
                <h2>
                    "Dari sistem yang dibangun hari ini,"
                    <br />
                    "menuju teknologi bangsa di masa depan."
                </h2>
                <p>
                    "N-1 Labs adalah perusahaan teknologi AI asal Semarang yang membangun AI Agent, sistem bisnis,
                    dan produk digital sebagai fondasi menuju kapabilitas yang lebih besar: AI infrastructure, AI
                    research, dan pada akhirnya, teknologi AI Indonesia yang mandiri."
                </p>
                <p>
                    "Setiap proyek yang kami kerjakan adalah satu langkah dalam perjalanan panjang — bukan sekadar
                    produk, tetapi bagian dari upaya membangun kedaulatan teknologi bangsa. Kami membangun apa
                    yang dibutuhkan hari ini, sambil terus bergerak menuju apa yang akan dibutuhkan Indonesia di
                    masa depan."
                </p>

                <div class="hero-ctas">
                    <a class="btn btn-primary" href="#portfolio">"Lihat Portofolio"</a>
                    <a class="btn btn-ghost" href="#top">"Kembali ke Atas"</a>
                </div>

                <div class="closing-contact">
                    <div>
                        <b>"N-1 Labs"</b>
                        <span>"Artificial Intelligence & Technology Company"</span>
                    </div>
                    <div>
                        <b>"Semarang, Jawa Tengah"</b>
                        <span>"Indonesia"</span>
                    </div>
                    <div>
                        <b>"Founded"</b>
                        <span>"2 September 2026"</span>
                    </div>
                </div>
            </div>
        </section>
    }
}
