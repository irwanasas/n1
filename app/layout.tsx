import type { Metadata } from "next";
import "./globals.css";
import { withBasePath } from "./base-path";

export const metadata: Metadata = {
  title: "N-1 Labs — Building Indonesia's Own AI Technology Capability",
  description:
    "N-1 Labs adalah perusahaan teknologi AI asal Semarang, Indonesia yang membangun AI Agent, software systems, automation, dan digital products.",
  icons: { icon: withBasePath("/logo.png") },
};

const THEME_INIT = `
(function () {
  try {
    var stored = localStorage.getItem("n1-theme");
    var theme = stored === "light" || stored === "dark"
      ? stored
      : (window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light");
    document.documentElement.setAttribute("data-theme", theme);
  } catch (e) {}
})();
`;

export default function RootLayout({ children }: LayoutProps<"/">) {
  return (
    <html lang="id" suppressHydrationWarning>
      <head>
        <script dangerouslySetInnerHTML={{ __html: THEME_INIT }} />
      </head>
      <body>{children}</body>
    </html>
  );
}
