pub mod about;
pub mod explorer;
pub mod how_this_works;
pub mod knowledge_graph;
pub mod theme;
pub mod widgets;

use std::path::PathBuf;
use std::sync::Arc;

use iced::widget::{Row, button, column, container, pick_list, row, space, text};
use iced::window::icon;
use iced::{Center, Element, Fill, Size, Task, Theme};

use vidya_core;

use crate::data;
use crate::query::{self, QueryOutcome};

const NOTO_DEVA: &[u8] = include_bytes!("../../fonts/NotoSansDevanagari-Regular.ttf");
const NOTO_SANS: &[u8] = include_bytes!("../../fonts/NotoSans-Regular.ttf");

const SCALE_STEP: f32 = 0.1;
const SCALE_MIN: f32 = 0.6;
const SCALE_MAX: f32 = 2.0;
const SCALE_DEFAULT: f32 = 1.1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tab {
    Explorer,
    KnowledgeGraph,
    HowThisWorks,
    About,
}

impl Tab {
    const ALL: [Tab; 4] = [
        Tab::Explorer,
        Tab::KnowledgeGraph,
        Tab::HowThisWorks,
        Tab::About,
    ];

    fn label(self) -> &'static str {
        match self {
            Tab::Explorer => "Explorer",
            Tab::KnowledgeGraph => "Knowledge Graph",
            Tab::HowThisWorks => "How This Works",
            Tab::About => "About",
        }
    }
}

struct State {
    data: data::AppData,
    active_tab: Tab,
    scale: f32,
    explorer: explorer::ExplorerState,
    knowledge_graph: knowledge_graph::KnowledgeGraphState,
}

#[derive(Debug, Clone)]
enum Message {
    TabSelected(Tab),
    ZoomIn,
    ZoomOut,
    ZoomReset,
    DomainSelected(String),
    LoadTtl,
    TtlLoaded(Option<PathBuf>),
    QueryInputChanged(String),
    QuerySubmitted,
    PresetClicked(usize),
    ToggleSearchHit(usize),
    ToggleProvenance(String),
    ToggleTrace,
    KgToggleType(usize),
    KgExpandEntity(usize),
    KgCollapseEntity,
    LinkClicked(String),
    LinkOpened,
}

fn boot() -> (State, Task<Message>) {
    let data = data::init().expect("failed to initialize knowledge store");
    let mut kg = knowledge_graph::KnowledgeGraphState::new();
    kg.load(&data);
    let state = State {
        data,
        active_tab: Tab::Explorer,
        scale: SCALE_DEFAULT,
        explorer: explorer::ExplorerState::new(),
        knowledge_graph: kg,
    };
    (state, Task::none())
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::TabSelected(tab) => {
            state.active_tab = tab;
        }
        Message::ZoomIn => {
            state.scale = (state.scale + SCALE_STEP).min(SCALE_MAX);
        }
        Message::ZoomOut => {
            state.scale = (state.scale - SCALE_STEP).max(SCALE_MIN);
        }
        Message::ZoomReset => {
            state.scale = SCALE_DEFAULT;
        }
        Message::DomainSelected(name) => {
            state.data.active_domain = name;
            state.data.resolve_ctx = state.data.store.resolve_context(&state.data.active_domain);
            if let Ok(catalog) = data::build_catalog(&state.data.store, &state.data.active_domain) {
                state.data.catalog = catalog;
            }
            state.explorer.result = None;
            state.explorer.expanded_predicate = None;
            state.explorer.expanded_search_hit = None;
            state.knowledge_graph.load(&state.data);
        }
        Message::LoadTtl => {
            return Task::perform(
                async {
                    let handle = rfd::AsyncFileDialog::new()
                        .add_filter("Turtle", &["ttl"])
                        .set_title("Load TTL knowledge file")
                        .pick_file()
                        .await;
                    handle.map(|h| h.path().to_path_buf())
                },
                Message::TtlLoaded,
            );
        }
        Message::TtlLoaded(Some(path)) => match data::load_custom(&mut state.data, &path) {
            Ok(info) => {
                tracing::info!(domain = %info.name, "loaded custom TTL");
                state.knowledge_graph.load(&state.data);
            }
            Err(e) => {
                tracing::error!(path = %path.display(), err = %e, "failed to load TTL");
            }
        },
        Message::TtlLoaded(None) => {}
        Message::QueryInputChanged(s) => {
            state.explorer.query_input = s;
        }
        Message::QuerySubmitted => {
            let input = state.explorer.query_input.trim().to_string();
            if !input.is_empty() {
                state.explorer.result = Some(query::execute(
                    &input,
                    &state.data.store,
                    &state.data.resolve_ctx,
                    &state.data.active_domain,
                ));
                state.explorer.expanded_predicate = None;
                state.explorer.expanded_search_hit = None;
                state.explorer.show_trace = false;
            }
        }
        Message::PresetClicked(idx) => {
            state.explorer.query_input = query::PRESETS[idx].input.to_string();
            state.explorer.result = Some(query::execute(
                query::PRESETS[idx].input,
                &state.data.store,
                &state.data.resolve_ctx,
                &state.data.active_domain,
            ));
            state.explorer.expanded_predicate = None;
            state.explorer.expanded_search_hit = None;
            state.explorer.show_trace = false;
        }
        Message::ToggleSearchHit(idx) => {
            if state.explorer.expanded_search_hit.as_ref().map(|h| h.index) == Some(idx) {
                state.explorer.expanded_search_hit = None;
            } else {
                let hit_name = state.explorer.result.as_ref().and_then(|outcome| {
                    if let QueryOutcome::Search { result, .. } = outcome {
                        result.entities.get(idx).map(|h| h.name.clone())
                    } else {
                        None
                    }
                });
                if let Some(name) = hit_name {
                    if let Ok(desc) = state.data.store.describe(
                        &state.data.active_domain,
                        &name,
                        &vidya_core::ProvenanceFilter::default(),
                    ) {
                        state.explorer.expanded_search_hit = Some(explorer::ExpandedHit {
                            index: idx,
                            describe: desc,
                        });
                        state.explorer.expanded_predicate = None;
                    }
                }
            }
        }
        Message::ToggleProvenance(pred) => {
            let expanded = match state.active_tab {
                Tab::KnowledgeGraph => &mut state.knowledge_graph.expanded_predicate,
                _ => &mut state.explorer.expanded_predicate,
            };
            if expanded.as_deref() == Some(pred.as_str()) {
                *expanded = None;
            } else {
                *expanded = Some(pred);
            }
        }
        Message::ToggleTrace => {
            state.explorer.show_trace = !state.explorer.show_trace;
        }
        Message::KgToggleType(idx) => {
            if state.knowledge_graph.expanded_type == Some(idx) {
                state.knowledge_graph.expanded_type = None;
                state.knowledge_graph.entities.clear();
                state.knowledge_graph.expanded_entity = None;
                state.knowledge_graph.expanded_predicate = None;
            } else {
                state.knowledge_graph.expanded_type = Some(idx);
                state.knowledge_graph.load_entities(&state.data);
            }
        }
        Message::KgExpandEntity(idx) => {
            if state
                .knowledge_graph
                .expanded_entity
                .as_ref()
                .map(|e| e.index)
                == Some(idx)
            {
                state.knowledge_graph.expanded_entity = None;
            } else if let Some(hit) = state.knowledge_graph.entities.get(idx) {
                if let Ok(desc) = state.data.store.describe(
                    &state.data.active_domain,
                    &hit.name,
                    &vidya_core::ProvenanceFilter::default(),
                ) {
                    state.knowledge_graph.expanded_entity = Some(knowledge_graph::ExpandedEntity {
                        index: idx,
                        describe: desc,
                    });
                    state.knowledge_graph.expanded_predicate = None;
                }
            }
        }
        Message::KgCollapseEntity => {
            state.knowledge_graph.expanded_entity = None;
            state.knowledge_graph.expanded_predicate = None;
        }
        Message::LinkClicked(url) => {
            return Task::perform(
                async move {
                    let _ = open::that_detached(&url);
                },
                |_| Message::LinkOpened,
            );
        }
        Message::LinkOpened => {}
    }
    Task::none()
}

fn view(state: &State) -> Element<'_, Message> {
    let title = text("Āyus")
        .size(24)
        .font(theme::latin())
        .color(theme::ACCENT);

    let tabs = Row::from_vec(
        Tab::ALL
            .iter()
            .map(|&tab| {
                let style: fn(&Theme, button::Status) -> button::Style = if tab == state.active_tab
                {
                    theme::tab_active
                } else {
                    theme::tab_inactive
                };
                button(text(tab.label()).size(14).font(theme::latin()))
                    .on_press(Message::TabSelected(tab))
                    .padding([6, 14])
                    .style(style)
                    .into()
            })
            .collect(),
    )
    .spacing(4);

    let pct = format!("{}%", (state.scale * 100.0).round() as i32);
    let zoom_controls = row![
        button(text("\u{2212}").size(14))
            .on_press(Message::ZoomOut)
            .padding([2, 8])
            .style(theme::zoom_btn),
        button(
            text(pct)
                .size(11)
                .font(theme::latin())
                .color(theme::TEXT_SECONDARY),
        )
        .on_press(Message::ZoomReset)
        .padding([2, 6])
        .style(theme::zoom_btn),
        button(text("+").size(14))
            .on_press(Message::ZoomIn)
            .padding([2, 8])
            .style(theme::zoom_btn),
    ]
    .spacing(2)
    .align_y(Center);

    let mut right_controls: Vec<Element<'_, Message>> = Vec::new();

    right_controls.push(zoom_controls.into());

    if state.data.domains.len() > 1 {
        let domain_names: Vec<String> = state.data.domains.iter().map(|d| d.name.clone()).collect();
        let selected = Some(state.data.active_domain.clone());
        right_controls.push(
            pick_list(domain_names, selected, Message::DomainSelected)
                .style(theme::domain_pick_list)
                .text_size(13)
                .font(theme::latin())
                .into(),
        );
    }

    right_controls.push(
        button(text("Load TTL").size(13).font(theme::latin()))
            .on_press(Message::LoadTtl)
            .padding([6, 12])
            .style(theme::accent_btn)
            .into(),
    );

    let top_bar = row![
        title,
        space::horizontal(),
        tabs,
        space::horizontal(),
        Row::from_vec(right_controls).spacing(8).align_y(Center),
    ]
    .padding([8, 16])
    .align_y(Center);

    let separator = iced::widget::rule::horizontal(1).style(theme::tab_rule);

    let content = match state.active_tab {
        Tab::Explorer => explorer::view(&state.explorer, &state.data),
        Tab::KnowledgeGraph => knowledge_graph::view(&state.knowledge_graph, &state.data),
        Tab::HowThisWorks => how_this_works::view(),
        Tab::About => about::view(),
    };

    column![top_bar, separator, content].into()
}

fn tab_stub(name: &str) -> Element<'_, Message> {
    container(
        text(name)
            .size(18)
            .font(theme::latin())
            .color(theme::TEXT_SECONDARY),
    )
    .center(Fill)
    .into()
}

fn app_theme(_state: &State) -> Theme {
    theme::ayus_theme()
}

fn load_icon() -> Option<icon::Icon> {
    // Include the icon bytes at compile time
    let image_bytes = include_bytes!("../../assets/her-feet2.jpg");

    // Load the image from the text bytes
    let image = image::load_from_memory(image_bytes).ok()?.to_rgba8();

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    // Create the iced Icon
    icon::from_rgba(rgba, width, height).ok()
}

pub fn run() -> iced::Result {
    iced::application(boot, update, view)
        .title("Āyus")
        .theme(app_theme)
        .scale_factor(|state: &State| state.scale)
        .font(NOTO_DEVA)
        .font(NOTO_SANS)
        .default_font(theme::latin())
        .window_size(Size::new(1100.0, 800.0))
        .window(iced::window::Settings {
            icon: load_icon(),
            ..Default::default()
        })
        .centered()
        .antialiasing(true)
        .run()
}
