use std::collections::HashSet;

use leptos::prelude::*;

use crate::content::{Project, PROJECTS};

#[component]
pub fn Portfolio() -> impl IntoView {
    let open_nums = RwSignal::new(HashSet::<&'static str>::new());

    let items = PROJECTS
        .iter()
        .map(|project: &'static Project| {
            let num = project.num;
            let body_id = format!("project-body-{num}");
            let body_id_for_attr = body_id.clone();
            let is_open = move || open_nums.get().contains(num);

            view! {
                <div class="project" class:open=is_open>
                    <button
                        type="button"
                        class="project-trigger"
                        aria-expanded=move || is_open().to_string()
                        aria-controls=body_id_for_attr
                        on:click=move |_| {
                            open_nums.update(|set| {
                                if set.contains(num) {
                                    set.remove(num);
                                } else {
                                    set.insert(num);
                                }
                            });
                        }
                    >
                        <div class="heading">
                            <span class="num">"PROJECT "{num}</span>
                            <span class="title">{project.title}</span>
                        </div>
                        <span class="chev" aria-hidden="true">
                            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M12 5v14M5 12h14" />
                            </svg>
                        </span>
                    </button>
                    <div class="project-body-wrap" id=body_id>
                        <div class="project-body">
                            <div class="project-position">"\""{project.position}"\""</div>

                            <div class="subhead">"Problem"</div>
                            <p style="font-size:13.5px;margin:0">{project.problem}</p>

                            <div class="subhead">"Solution"</div>
                            <p style="font-size:13.5px;margin:0">{project.solution}</p>

                            {project.flow.map(|flow| view! {
                                <div class="panel" style="text-align:center;margin-top:12px;font-size:12.5px;font-weight:700">
                                    {flow}
                                </div>
                            })}

                            <div class="grid grid-2" style="margin-top:12px">
                                <div class="panel">
                                    <h4 style="margin:0 0 8px 0;font-size:11.5px;text-transform:uppercase;letter-spacing:0.5px">
                                        "Function"
                                    </h4>
                                    <ul class="clean">
                                        {project.function.iter().map(|item| view! { <li>{*item}</li> }).collect::<Vec<_>>()}
                                    </ul>
                                </div>
                                <div class="panel">
                                    <h4 style="margin:0 0 8px 0;font-size:11.5px;text-transform:uppercase;letter-spacing:0.5px">
                                        "Impact"
                                    </h4>
                                    <ul class="clean">
                                        {project.impact.iter().map(|item| view! { <li>{*item}</li> }).collect::<Vec<_>>()}
                                    </ul>
                                </div>
                            </div>

                            <div class="subhead">"What N-1 Labs Learned / Built"</div>
                            <p style="font-size:13.5px;margin:0">{project.learned}</p>
                        </div>
                    </div>
                </div>
            }
        })
        .collect::<Vec<_>>();

    view! { <div style="margin-top:24px">{items}</div> }
}
