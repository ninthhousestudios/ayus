use iced::widget::{
    Column, Row, button, column, container, row, scrollable, space, text, text_input,
};
use iced::{Center, Element, Fill};

use vidya_core::{
    DescribeResult, ProvenanceResult, ResolutionReport, ResolvedQuery, SearchResult,
    SimilarityResult, TraverseResult,
};

use super::Message;
use super::theme;
use super::widgets;
use crate::data::AppData;
use crate::query::{PRESETS, QueryOutcome};

pub struct ExplorerState {
    pub query_input: String,
    pub result: Option<QueryOutcome>,
    pub expanded_predicate: Option<String>,
    pub expanded_search_hit: Option<ExpandedHit>,
    pub show_trace: bool,
}

pub struct ExpandedHit {
    pub index: usize,
    pub describe: DescribeResult,
}

impl ExplorerState {
    pub fn new() -> Self {
        Self {
            query_input: String::new(),
            result: None,
            expanded_predicate: None,
            expanded_search_hit: None,
            show_trace: false,
        }
    }
}

pub(super) fn view<'a>(state: &'a ExplorerState, _data: &'a AppData) -> Element<'a, Message> {
    let input = text_input(
        "Search dravyas, properties, or try a preset...",
        &state.query_input,
    )
    .on_input(Message::QueryInputChanged)
    .on_submit(Message::QuerySubmitted)
    .style(theme::input_style)
    .size(14)
    .padding(10);

    let search_btn = button(text("Search").size(13).font(theme::latin()))
        .on_press(Message::QuerySubmitted)
        .padding([10, 16])
        .style(theme::accent_btn);

    let input_row = row![input, search_btn].spacing(8).align_y(Center);

    let presets = Row::from_vec(
        PRESETS
            .iter()
            .enumerate()
            .map(|(i, p)| {
                button(text(p.label).size(12).font(theme::latin()))
                    .on_press(Message::PresetClicked(i))
                    .padding([4, 10])
                    .style(theme::preset_pill)
                    .into()
            })
            .collect(),
    )
    .spacing(6);

    let mut sections: Vec<Element<'_, Message>> = vec![input_row.into(), presets.into()];

    if let Some(outcome) = &state.result {
        sections.push(render_outcome(outcome, state));
    }

    sections.push(guide_card());

    let content = Column::from_vec(sections)
        .spacing(12)
        .padding([16, 24])
        .width(Fill)
        .max_width(900);

    let centered_layout = container(content).center_x(Fill);

    scrollable(centered_layout).height(Fill).into()
}

fn render_outcome<'a>(outcome: &'a QueryOutcome, state: &'a ExplorerState) -> Element<'a, Message> {
    match outcome {
        QueryOutcome::Describe { result, report } => column![
            widgets::render_describe(result, &state.expanded_predicate),
            render_trace(report, state.show_trace),
        ]
        .spacing(12)
        .into(),
        QueryOutcome::Search { result, report } => column![
            render_search(
                result,
                &state.expanded_search_hit,
                &state.expanded_predicate
            ),
            render_trace(report, state.show_trace),
        ]
        .spacing(12)
        .into(),
        QueryOutcome::Traverse { result, report } => column![
            render_traverse(result),
            render_trace(report, state.show_trace),
        ]
        .spacing(12)
        .into(),
        QueryOutcome::Provenance { result, report } => column![
            render_provenance_result(result),
            render_trace(report, state.show_trace),
        ]
        .spacing(12)
        .into(),
        QueryOutcome::Similar { result, report } | QueryOutcome::Unbind { result, report } => {
            column![
                render_similarity(result),
                render_trace(report, state.show_trace),
            ]
            .spacing(12)
            .into()
        }
        QueryOutcome::NoMatch { unknown_tokens, .. } => render_no_match(unknown_tokens),
    }
}

// ---- Search results ----

fn render_search<'a>(
    result: &'a SearchResult,
    expanded: &'a Option<ExpandedHit>,
    expanded_pred: &'a Option<String>,
) -> Element<'a, Message> {
    let mut items: Vec<Element<'_, Message>> = Vec::new();

    items.push(
        text(format!("Found {} results", result.entities.len()))
            .size(14)
            .color(theme::TEXT_SECONDARY)
            .into(),
    );

    for (i, hit) in result.entities.iter().enumerate() {
        let is_expanded = expanded.as_ref().map(|h| h.index) == Some(i);
        let arrow = if is_expanded { "▾" } else { "▸" };

        items.push(
            button(
                row![
                    text(hit.label.clone()).size(14).color(theme::TEXT_COLOR),
                    space::horizontal(),
                    text(arrow).size(12).color(theme::TEXT_SECONDARY),
                ]
                .align_y(Center)
                .width(Fill),
            )
            .on_press(Message::ToggleSearchHit(i))
            .style(theme::tab_inactive)
            .padding([6, 12])
            .width(Fill)
            .into(),
        );

        if is_expanded {
            if let Some(exp) = expanded {
                items.push(widgets::render_describe(&exp.describe, expanded_pred));
            }
        }
    }

    Column::from_vec(items).spacing(2).width(Fill).into()
}

// ---- Traverse result ----

fn render_traverse(result: &TraverseResult) -> Element<'_, Message> {
    let header = column![
        text(format!("{} → {}", result.origin, result.predicate))
            .size(16)
            .color(theme::ACCENT),
        text(format!("max depth: {}", result.max_depth))
            .size(12)
            .color(theme::TEXT_SECONDARY),
    ]
    .spacing(4);

    let items: Vec<Element<'_, Message>> = result
        .entities
        .iter()
        .map(|hit| {
            let label = hit.label.as_deref().unwrap_or(&hit.iri);
            let indent = "  ".repeat(hit.depth as usize);
            text(format!("{indent}{label}"))
                .size(14)
                .color(theme::TEXT_COLOR)
                .into()
        })
        .collect();

    container(column![header, Column::from_vec(items).spacing(2)].spacing(8))
        .style(theme::card)
        .padding(16)
        .width(Fill)
        .into()
}

// ---- Provenance result ----

fn render_provenance_result(result: &ProvenanceResult) -> Element<'_, Message> {
    let header = text(format!(
        "{} → {} → {}",
        result.subject, result.predicate, result.object
    ))
    .size(16)
    .color(theme::ACCENT);

    let assertions: Vec<Element<'_, Message>> = result
        .assertions
        .iter()
        .map(|p| {
            row![
                text(p.source.clone()).size(13).color(theme::TEXT_COLOR),
                text(p.pramana.clone())
                    .size(12)
                    .color(theme::TEXT_SECONDARY),
                text(format!("conf: {}", p.confidence))
                    .size(12)
                    .color(theme::TEXT_SECONDARY),
            ]
            .spacing(16)
            .into()
        })
        .collect();

    container(column![header, Column::from_vec(assertions).spacing(4)].spacing(8))
        .style(theme::card)
        .padding(16)
        .width(Fill)
        .into()
}

// ---- Similarity results ----

fn render_similarity(result: &SimilarityResult) -> Element<'_, Message> {
    let header = column![
        text(&result.query).size(16).color(theme::ACCENT),
        text(format!("{} matches", result.matches.len()))
            .size(12)
            .color(theme::TEXT_SECONDARY),
    ]
    .spacing(4);

    let items: Vec<Element<'_, Message>> = result
        .matches
        .iter()
        .map(|m| {
            text(format!("{}  ({:.3})", m.label, m.score))
                .size(14)
                .color(theme::TEXT_COLOR)
                .into()
        })
        .collect();

    container(column![header, Column::from_vec(items).spacing(4).width(Fill)].spacing(8))
        .style(theme::card)
        .padding(16)
        .width(Fill)
        .into()
}

// ---- Empty / no-match states ----

fn guide_row<'a>(preset_idx: usize, desc: &'a str) -> Element<'a, Message> {
    button(
        text(desc)
            .size(13)
            .font(theme::latin())
            .color(theme::TEXT_COLOR),
    )
    .on_press(Message::PresetClicked(preset_idx))
    .style(theme::tab_inactive)
    .padding([6, 12])
    .width(Fill)
    .into()
}

fn guide_card<'a>() -> Element<'a, Message> {
    container(
        column![
            text("What you can discover here")
                .size(16)
                .font(theme::latin())
                .color(theme::ACCENT),
            guide_row(0, "\u{25b8} Look up a substance \u{2014} click to see Pippali\u{2019}s tastes, energies, and effects"),
            guide_row(2, "\u{25b8} Query by property \u{2014} find every substance that pacifies Vata dosha"),
            guide_row(3, "\u{25b8} Explore relationships \u{2014} see Pippali\u{2019}s therapeutic actions (karma)"),
            guide_row(5, "\u{25b8} Ask a question \u{2014} try \u{201c}What is haritaki?\u{201d} for a natural-language lookup"),
            guide_row(4, "\u{25b8} Find similar substances \u{2014} uses vector similarity to find dravyas related to Pippali"),
            text("Click any example to try it, or type your own query above.")
                .size(12)
                .font(theme::latin())
                .color(theme::TEXT_SECONDARY),
        ]
        .spacing(4),
    )
    .style(theme::card)
    .padding([16, 20])
    .width(Fill)
    .into()
}

fn render_no_match(unknown_tokens: &[String]) -> Element<'_, Message> {
    let token_text = if unknown_tokens.is_empty() {
        "No tokens could be resolved.".to_string()
    } else {
        format!("Unknown terms: {}", unknown_tokens.join(", "))
    };
    container(
        column![
            text("No results found")
                .size(18)
                .color(theme::TEXT_SECONDARY),
            text(token_text).size(14).color(theme::TEXT_SECONDARY),
            text("Try a preset query or different search terms")
                .size(13)
                .color(theme::TEXT_SECONDARY),
        ]
        .spacing(8)
        .align_x(Center),
    )
    .center(Fill)
    .into()
}

// ---- Resolution trace ----

fn render_trace(report: &ResolutionReport, show: bool) -> Element<'_, Message> {
    let toggle_label = if show {
        "▾ Resolution trace"
    } else {
        "▸ Resolution trace"
    };
    let toggle = button(text(toggle_label).size(12).color(theme::TEXT_SECONDARY))
        .on_press(Message::ToggleTrace)
        .style(theme::tab_inactive)
        .padding([4, 8]);

    if !show {
        return toggle.into();
    }

    let mode = match &report.query {
        ResolvedQuery::Describe { .. } => "Describe",
        ResolvedQuery::Search { .. } => "Search",
        ResolvedQuery::Traverse { .. } => "Traverse",
        ResolvedQuery::Provenance { .. } => "Provenance",
        ResolvedQuery::Similar { .. } => "Similar",
        ResolvedQuery::Unbind { .. } => "Unbind",
    };

    let details: Vec<Element<'_, Message>> = report
        .resolution_details
        .iter()
        .map(|d| {
            text(d.as_str())
                .size(11)
                .font(theme::latin())
                .color(theme::TEXT_SECONDARY)
                .into()
        })
        .collect();

    let mut trace_rows: Vec<Element<'_, Message>> = vec![
        text(format!("Query mode: {mode}"))
            .size(11)
            .color(theme::TEXT_SECONDARY)
            .into(),
    ];
    trace_rows.extend(details);

    if !report.unknown_tokens.is_empty() {
        trace_rows.push(
            text(format!("Unknown: {}", report.unknown_tokens.join(", ")))
                .size(11)
                .color(theme::ACCENT)
                .into(),
        );
    }

    column![
        toggle,
        container(Column::from_vec(trace_rows).spacing(2))
            .style(theme::provenance_drawer)
            .padding([8, 12]),
    ]
    .spacing(4)
    .into()
}
