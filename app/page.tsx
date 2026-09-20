import Image from "next/image";
import Header from "./Header";
import {
  identity,
  stats,
  missionPillars,
  techDomains,
  buildAreas,
  projects,
  journey,
  visionPoints,
  positioningFlow,
  conceptualRoadmap,
  llmStages,
  llmReasons,
  securityAreas,
  values,
  roadmapPhases,
  positioningCards,
} from "./content";

export default function Home() {
  return (
    <>
      <div id="top" />
      <Header />

      <section className="hero">
        <div className="container">
          <div className="hero-eyebrow">Company Profile · 2026</div>
          <h1>
            Building Indonesia&apos;s Own <span>AI Technology</span> Capability
          </h1>
          <p>
            N-1 Labs adalah perusahaan teknologi AI asal Indonesia yang membangun AI Agent, software
            systems, automation, dan digital products — dengan arah jangka panjang menjadi AI technology
            company yang membangun teknologinya sendiri.
          </p>
          <div className="hero-ctas">
            <a className="btn btn-primary" href="#portfolio">
              Lihat Portofolio
            </a>
            <a className="btn btn-ghost" href="#vision">
              Pelajari Visi Kami
            </a>
          </div>
          <div className="hero-stats">
            {stats.map((stat) => (
              <div className="stat" key={stat.label}>
                <b>{stat.value}</b>
                <span>{stat.label}</span>
              </div>
            ))}
          </div>
        </div>
      </section>

      <section id="about">
        <div className="container">
          <div className="kicker">01 · Tentang Kami</div>
          <h2 className="section-title">About N-1 Labs</h2>
          <div className="divider" />

          <p className="section-lead" style={{ maxWidth: 720 }}>
            N-1 Labs adalah perusahaan teknologi asal Semarang, Jawa Tengah, yang berfokus pada
            pengembangan Artificial Intelligence, AI Agent, software systems, automation, digital
            products, dan teknologi komputasi.
          </p>
          <p style={{ maxWidth: 720 }}>
            Didirikan pada 2 September 2026, N-1 Labs hadir bukan sebagai software house biasa,
            melainkan sebagai perusahaan yang membangun kapabilitasnya secara bertahap — dimulai dari
            software engineering, menuju sistem bisnis dan automation, hingga akhirnya pada pengembangan
            AI Agent dan AI Infrastructure. Setiap proyek yang dikerjakan N-1 Labs dirancang untuk
            memperkuat fondasi teknis menuju arah tersebut.
          </p>
          <p style={{ maxWidth: 720 }}>
            Dalam jangka panjang, N-1 Labs memiliki tujuan untuk berkembang menjadi{" "}
            <b>AI technology company</b> yang mampu membangun teknologi AI-nya sendiri — termasuk
            kontribusi terhadap kemampuan Indonesia dalam mengembangkan Large Language Model (LLM)
            secara mandiri.
          </p>

          <div className="grid grid-3" style={{ marginTop: 24 }}>
            {stats.map((stat) => (
              <div className="panel stat-card" key={stat.label}>
                <b>{stat.value}</b>
                <span>{stat.label}</span>
              </div>
            ))}
          </div>

          <div className="subhead" style={{ marginTop: 32 }}>
            Ringkasan Identitas
          </div>
          <div className="panel">
            <table className="identity-table">
              <tbody>
                {identity.map((row) => (
                  <tr key={row.label}>
                    <td>{row.label}</td>
                    <td>{row.value}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </section>

      <section id="mission">
        <div className="container">
          <div className="kicker">02 · Cara Kami Mewujudkannya</div>
          <h2 className="section-title">Mission</h2>
          <div className="divider" />
          <p className="section-lead">
            Misi N-1 Labs disusun berdasarkan tujuh prinsip kerja: Develop, Integrate, Automate,
            Research, Build, Scale, dan Protect.
          </p>

          <div className="grid grid-2" style={{ marginTop: 16 }}>
            {missionPillars.map((pillar) => (
              <div
                className="panel"
                key={pillar.title}
                style={pillar.wide ? { gridColumn: "1 / -1" } : undefined}
              >
                <h4 style={{ margin: "0 0 8px 0", fontSize: 14.5 }}>{pillar.title}</h4>
                <p style={{ margin: 0, fontSize: 13.5, color: "var(--muted)" }}>{pillar.body}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      <section id="technology">
        <div className="container">
          <div className="kicker">03 · Kapabilitas Teknologi</div>
          <h2 className="section-title">Technology Focus</h2>
          <div className="divider" />
          <p className="section-lead">
            Kapabilitas N-1 Labs dikelompokkan ke dalam enam domain teknologi, dari sistem yang telah
            dibangun hingga arah riset masa depan.
          </p>

          <div className="grid grid-2" style={{ marginTop: 16 }}>
            {techDomains.map((domain) => (
              <div className={`panel domain-card${domain.vision ? " vision" : ""}`} key={domain.name}>
                <h3>
                  <span className="dot" />
                  {domain.name}
                  {domain.vision && <span className="tag vision" style={{ marginLeft: 6 }}>Vision</span>}
                </h3>
                <ul>
                  {domain.items.map((item) => (
                    <li key={item}>{item}</li>
                  ))}
                </ul>
              </div>
            ))}
          </div>
        </div>
      </section>

      <section id="what-we-build">
        <div className="container">
          <div className="kicker">04 · Output Nyata</div>
          <h2 className="section-title">What We Build</h2>
          <div className="divider" />
          <p className="section-lead">
            N-1 Labs membangun sistem yang digunakan secara nyata — bukan sekadar prototipe atau konsep.
            Setiap produk dirancang untuk menjadi bagian dari infrastruktur bisnis penggunanya.
          </p>

          <div className="grid grid-2" style={{ marginTop: 16 }}>
            {buildAreas.map((area) => (
              <div className="panel" key={area.title}>
                <h4 style={{ margin: "0 0 8px 0", fontSize: 14.5 }}>{area.title}</h4>
                <p style={{ margin: 0, fontSize: 13.5, color: "var(--muted)" }}>{area.body}</p>
              </div>
            ))}
          </div>

          <div className="subhead" style={{ marginTop: 28 }}>
            Enam Proyek yang Telah Dibangun
          </div>
          <div className="panel">
            <ul className="clean" style={{ columns: 2, columnGap: 24 }}>
              {projects.map((project) => (
                <li key={project.num}>{project.title}</li>
              ))}
            </ul>
          </div>
        </div>
      </section>

      <section id="portfolio">
        <div className="container">
          <div className="kicker">05 · Rekam Jejak</div>
          <h2 className="section-title">Project Portfolio</h2>
          <div className="divider" />
          <p className="section-lead">
            Enam proyek berikut menggambarkan perjalanan N-1 Labs dalam membangun kapabilitas — dari
            software engineering, sistem bisnis, automation, hingga AI Agent. Setiap proyek dijelaskan
            berdasarkan permasalahan, solusi, fungsi, dampak, dan hal yang dipelajari N-1 Labs.
          </p>
          <span className="tag existing">Existing Capability</span>

          <div style={{ marginTop: 24 }}>
            {projects.map((project) => (
              <details className="project" key={project.num}>
                <summary>
                  <div className="heading">
                    <span className="num">PROJECT {project.num}</span>
                    <span className="title">{project.title}</span>
                  </div>
                  <span className="chev" aria-hidden="true">
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                      <path d="M12 5v14M5 12h14" />
                    </svg>
                  </span>
                </summary>
                <div className="project-body">
                  <div className="project-position">&quot;{project.position}&quot;</div>

                  <div className="subhead">Problem</div>
                  <p style={{ fontSize: 13.5, margin: 0 }}>{project.problem}</p>

                  <div className="subhead">Solution</div>
                  <p style={{ fontSize: 13.5, margin: 0 }}>{project.solution}</p>

                  {project.flow && (
                    <div className="panel" style={{ textAlign: "center", marginTop: 12, fontSize: 12.5, fontWeight: 700 }}>
                      {project.flow}
                    </div>
                  )}

                  <div className="grid grid-2" style={{ marginTop: 12 }}>
                    <div className="panel">
                      <h4 style={{ margin: "0 0 8px 0", fontSize: 11.5, textTransform: "uppercase", letterSpacing: 0.5 }}>
                        Function
                      </h4>
                      <ul className="clean">
                        {project.function.map((item) => (
                          <li key={item}>{item}</li>
                        ))}
                      </ul>
                    </div>
                    <div className="panel">
                      <h4 style={{ margin: "0 0 8px 0", fontSize: 11.5, textTransform: "uppercase", letterSpacing: 0.5 }}>
                        Impact
                      </h4>
                      <ul className="clean">
                        {project.impact.map((item) => (
                          <li key={item}>{item}</li>
                        ))}
                      </ul>
                    </div>
                  </div>

                  <div className="subhead">What N-1 Labs Learned / Built</div>
                  <p style={{ fontSize: 13.5, margin: 0 }}>{project.learned}</p>
                </div>
              </details>
            ))}
          </div>
        </div>
      </section>

      <section id="journey">
        <div className="container">
          <div className="kicker">06 · Perjalanan Kapabilitas</div>
          <h2 className="section-title">Technology Journey</h2>
          <div className="divider" />
          <p className="section-lead">
            Proyek-proyek yang telah dikerjakan N-1 Labs bukan kumpulan proyek yang berdiri sendiri,
            melainkan tahapan yang membangun kapabilitas perusahaan secara bertahap menuju teknologi AI
            berskala nasional.
          </p>

          <div className="timeline">
            {journey.map((step, i) => (
              <div className="tstep" key={step.title}>
                <div className="tline">
                  <div className={`tdot${step.vision ? " vision" : ""}`} />
                  {i < journey.length - 1 && <div className="tbar" />}
                </div>
                <div className="tbody">
                  <h4>
                    {step.title}
                    {step.vision && <span className="tag vision">Vision</span>}
                  </h4>
                  <p>{step.body}</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      <section id="vision">
        <div className="container">
          <div className="kicker">07 · Arah Perusahaan</div>
          <h2 className="section-title">Vision</h2>
          <div className="divider" />

          <div className="quote-block">
            <p>
              &ldquo;Membangun teknologi Artificial Intelligence Indonesia yang mandiri, berdaulat, dan
              mampu digunakan untuk kebutuhan masyarakat, industri, serta kepentingan strategis
              nasional.&rdquo;
            </p>
          </div>

          <p>
            Visi ini menjadi arah jangka panjang N-1 Labs, yang secara bertahap dijabarkan ke dalam
            beberapa fokus pengembangan:
          </p>
          <ul className="clean">
            {visionPoints.map((point) => (
              <li key={point}>{point}</li>
            ))}
          </ul>

          <div className="subhead">Positioning Arah Perusahaan</div>
          <div className="panel flow-chain">
            {positioningFlow.map((step, i) => (
              <div className="step" key={step} style={{ display: "flex", alignItems: "center", gap: 10, flex: "1 1 auto" }}>
                <span>{step}</span>
                {i < positioningFlow.length - 1 && <span className="arrow">→</span>}
              </div>
            ))}
          </div>
          <p style={{ fontSize: 12, color: "var(--muted)", marginTop: 12 }}>
            N-1 Labs tidak memposisikan dirinya sebagai penyedia chatbot maupun software development
            semata, melainkan sebagai perusahaan yang membangun jenjang kapabilitas menuju teknologi AI
            berskala nasional.
          </p>

          <div className="kicker" style={{ marginTop: 48 }}>
            08 · Masa Depan
          </div>
          <h2 className="section-title">Long-Term Vision</h2>
          <div className="divider" />
          <span className="tag vision">Future Vision / Aspiration</span>

          <div className="quote-block" style={{ marginTop: 16 }}>
            <p>
              &ldquo;Today we build AI applications. Tomorrow we build AI infrastructure. Eventually, we
              aim to contribute to Indonesia&apos;s own AI foundation models.&rdquo;
            </p>
          </div>

          <p>
            N-1 Labs saat ini fokus membangun AI Agent dan sistem berbasis AI untuk kebutuhan bisnis.
            Namun, arah jangka panjang perusahaan mengarah pada kontribusi terhadap kapabilitas Indonesia
            dalam membangun teknologi AI-nya sendiri — termasuk Large Language Model (LLM).
          </p>

          <div className="subhead">Roadmap Kapabilitas Konseptual</div>
          <div className="panel flow-chain">
            {conceptualRoadmap.map((step, i) => (
              <div className="step" key={step} style={{ display: "flex", alignItems: "center", gap: 10, flex: "1 1 auto" }}>
                <span>{step}</span>
                {i < conceptualRoadmap.length - 1 && <span className="arrow">→</span>}
              </div>
            ))}
          </div>
          <p style={{ fontSize: 12, color: "var(--muted)", marginTop: 12 }}>
            Roadmap ini bersifat konseptual dan aspirasional. N-1 Labs saat ini belum memiliki foundation
            model atau LLM sendiri; tahapan di atas menggambarkan arah pengembangan kapabilitas jangka
            panjang perusahaan.
          </p>

          <div className="kicker" style={{ marginTop: 48 }}>
            09 · Kedaulatan Teknologi
          </div>
          <h2 className="section-title">Building Indonesia&apos;s Own LLM</h2>
          <div className="divider" />
          <span className="tag vision">Future Vision / Aspiration</span>

          <p style={{ marginTop: 16 }}>
            Salah satu tujuan jangka panjang N-1 Labs adalah berkontribusi terhadap kemampuan Indonesia
            dalam membangun Large Language Model (LLM) sendiri — sebagai bagian dari kedaulatan teknologi
            bangsa di bidang kecerdasan buatan.
          </p>
          <p>
            <b>N-1 Labs saat ini belum memiliki LLM sendiri.</b> Kapabilitas yang dimiliki saat ini
            berada pada tahap membangun AI Agent dan aplikasi berbasis AI. Pengembangan menuju model
            engineering dan foundation model merupakan arah jangka panjang yang akan dibangun secara
            bertahap.
          </p>

          <div className="subhead">Mengapa Ini Penting</div>
          <ul className="clean">
            {llmReasons.map((reason) => (
              <li key={reason}>{reason}</li>
            ))}
          </ul>

          <div className="subhead">Tahapan Menuju Kemampuan LLM</div>
          <div className="grid grid-2">
            {llmStages.map((stage) => (
              <div className="panel" key={stage.title}>
                <h4 style={{ margin: "0 0 6px 0", fontSize: 13.5 }}>{stage.title}</h4>
                <p style={{ margin: 0, fontSize: 12.5, color: "var(--muted)" }}>{stage.body}</p>
              </div>
            ))}
          </div>

          <div className="kicker" style={{ marginTop: 48 }}>
            10 · Kontribusi Strategis
          </div>
          <h2 className="section-title">National Security &amp; Defense Technology</h2>
          <div className="divider" />
          <span className="tag vision">Future Vision / Aspiration</span>

          <p style={{ marginTop: 16 }}>
            N-1 Labs memiliki aspirasi jangka panjang untuk bekerja sama dengan institusi pemerintah,
            pertahanan, dan militer Indonesia dalam mengembangkan teknologi AI yang dapat mendukung
            kebutuhan strategis nasional, di antaranya:
          </p>

          <div className="grid grid-2">
            {securityAreas.map((area) => (
              <div className="panel" key={area}>
                <p style={{ margin: 0, fontSize: 13.5 }}>{area}</p>
              </div>
            ))}
          </div>

          <div className="quote-block" style={{ marginTop: 20 }}>
            <p>
              &ldquo;Mengembangkan teknologi AI yang dapat menjadi bagian dari technological capability
              Indonesia dalam menjaga keamanan dan kedaulatan negara.&rdquo;
            </p>
          </div>

          <p style={{ fontSize: 12.5, color: "var(--muted)", marginTop: 12 }}>
            <b style={{ color: "var(--ink)" }}>Catatan penting:</b> N-1 Labs bukan perusahaan militer, dan
            saat ini belum memiliki kerja sama resmi dengan institusi pertahanan atau militer mana pun.
            Bagian ini menggambarkan arah dan aspirasi jangka panjang perusahaan, bukan kemitraan yang
            sudah berjalan.
          </p>
        </div>
      </section>

      <section id="values">
        <div className="container">
          <div className="kicker">11 · Prinsip Kerja</div>
          <h2 className="section-title">Company Values</h2>
          <div className="divider" />

          <div className="panel">
            {values.map((value, i) => (
              <div className="value-row" key={value.title}>
                <div className="value-num">{String(i + 1).padStart(2, "0")}</div>
                <div className="value-body">
                  <h4>{value.title}</h4>
                  <p>{value.body}</p>
                </div>
              </div>
            ))}
          </div>
        </div>
      </section>

      <section id="roadmap">
        <div className="container">
          <div className="kicker">12 · Jalan ke Depan</div>
          <h2 className="section-title">Development Roadmap</h2>
          <div className="divider" />
          <p style={{ fontSize: 12.5, color: "var(--muted)", marginBottom: 20 }}>
            Roadmap berikut bersifat aspirasional — menggambarkan arah pengembangan, bukan janji atau
            kepastian waktu.
          </p>

          {roadmapPhases.map((phase) => (
            <div className="phase" key={phase.phase}>
              <div className="phase-head">
                <h4>{phase.phase}</h4>
                <span className="yr">{phase.year}</span>
              </div>
              <div className="phase-body">
                <ul>
                  {phase.items.map((item) => (
                    <li key={item}>{item}</li>
                  ))}
                </ul>
              </div>
            </div>
          ))}
        </div>
      </section>

      <section id="positioning">
        <div className="container">
          <div className="kicker">13 · Cara Kami Diposisikan</div>
          <h2 className="section-title">Company Positioning</h2>
          <div className="divider" />

          <div className="quote-block">
            <p>
              &ldquo;An Indonesian AI Technology Company Building Intelligent Systems for Business,
              Industry, and the Future of National Technology.&rdquo;
            </p>
          </div>

          <p className="section-lead">
            Positioning inti ini konsisten di seluruh konteks komunikasi, dengan penekanan yang
            disesuaikan menurut audiens:
          </p>

          <div className="grid grid-2" style={{ marginTop: 8 }}>
            {positioningCards.map((card) => (
              <div className="pos-card" key={card.title}>
                <h4>{card.title}</h4>
                <p>{card.body}</p>
              </div>
            ))}
          </div>
        </div>
      </section>

      <section id="contact" className="closing">
        <div className="container">
          <div className="kicker">14 · Closing Statement</div>
          <h2>
            Dari sistem yang dibangun hari ini,
            <br />
            menuju teknologi bangsa di masa depan.
          </h2>
          <p>
            N-1 Labs adalah perusahaan teknologi AI asal Semarang yang membangun AI Agent, sistem bisnis,
            dan produk digital sebagai fondasi menuju kapabilitas yang lebih besar: AI infrastructure, AI
            research, dan pada akhirnya, teknologi AI Indonesia yang mandiri.
          </p>
          <p>
            Setiap proyek yang kami kerjakan adalah satu langkah dalam perjalanan panjang — bukan sekadar
            produk, tetapi bagian dari upaya membangun kedaulatan teknologi bangsa. Kami membangun apa
            yang dibutuhkan hari ini, sambil terus bergerak menuju apa yang akan dibutuhkan Indonesia di
            masa depan.
          </p>

          <div className="hero-ctas">
            <a className="btn btn-primary" href="#portfolio">
              Lihat Portofolio Kami
            </a>
            <a className="btn btn-ghost" href="#top">
              Kembali ke Atas
            </a>
          </div>

          <div className="closing-contact">
            <div>
              <b>N-1 Labs</b>
              <span>Artificial Intelligence &amp; Technology Company</span>
            </div>
            <div>
              <b>Semarang, Jawa Tengah</b>
              <span>Indonesia</span>
            </div>
            <div>
              <b>Founded</b>
              <span>2 September 2026</span>
            </div>
          </div>
        </div>
      </section>

      <footer className="site-footer">
        <div className="container">
          <a className="brand" href="#top">
            <Image src="/logo.png" alt="N-1 Labs" width={26} height={26} />
            N-1 LABS
          </a>
          <span>© 2026 N-1 Labs. All rights reserved.</span>
        </div>
      </footer>
    </>
  );
}
