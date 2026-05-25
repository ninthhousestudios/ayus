use std::collections::HashSet;

use iced::widget::{
    button, column, container, row, scrollable, space, text, text_input, Column, Row,
};
use iced::{Center, Element, Fill};

use vidya_core::query::AnnotatedTriple;
use vidya_core::{
    DescribeResult, ProvenanceResult, ResolvedQuery, ResolutionReport, SearchResult, TraverseResult,
};

use super::theme;
use super::Message;
use crate::data::AppData;
use crate::query::{QueryOutcome, PRESETS};

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

const PREDICATE_ORDER: &[(&str, &str)] = &[
    ("hasRasa", "RASA"),
    ("hasGuna", "GUNA"),
    ("hasVeerya", "VEERYA"),
    ("hasVipaka", "VIPAKA"),
    ("hasKarma", "KARMA"),
    ("pacifiesDosha", "PACIFIES"),
    ("aggravatesDosha", "AGGRAVATES"),
];

pub(super) fn view<'a>(state: &'a ExplorerState, _data: &'a AppData) -> Element<'a, Message> {
    let input =
        text_input("Search dravyas, properties, or try a preset...", &state.query_input)
            .on_input(Message::QueryInputChanged)
            .on_submit(Message::QuerySubmitted)
            .style(theme::input_style)
            .size(16)
            .padding(12);

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

    let result_view = match &state.result {
        Some(outcome) => render_outcome(outcome, state),
        None => empty_state(),
    };

    let content = column![input, presets, result_view]
        .spacing(12)
        .padding([16, 24]);

    scrollable(content).height(Fill).into()
}

fn render_outcome<'a>(
    outcome: &'a QueryOutcome,
    state: &'a ExplorerState,
) -> Element<'a, Message> {
    match outcome {
        QueryOutcome::Describe { result, report } => column![
            render_describe(result, &state.expanded_predicate),
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
        QueryOutcome::NoMatch {
            unknown_tokens, ..
        } => render_no_match(unknown_tokens),
    }
}

// ---- Describe card ----

fn render_describe<'a>(
    result: &'a DescribeResult,
    expanded_pred: &Option<String>,
) -> Element<'a, Message> {
    let label_text = text(result.label.as_deref().unwrap_or("unknown"))
        .size(22)
        .color(theme::ACCENT);

    let types_row = Row::from_vec(
        result
            .types
            .iter()
            .map(|t| {
                container(
                    text(t.as_str())
                        .size(11)
                        .font(theme::latin())
                        .color(theme::TEXT_COLOR),
                )
                .padding([2, 8])
                .style(theme::type_badge)
                .into()
            })
            .collect(),
    )
    .spacing(4);

    let header = column![label_text, types_row].spacing(6);

    let mut prop_rows: Vec<Element<'_, Message>> = Vec::new();
    let ordered_preds: Vec<&str> = PREDICATE_ORDER.iter().map(|&(p, _)| p).collect();

    for &(pred, display_name) in PREDICATE_ORDER {
        let values: Vec<&str> = result
            .properties
            .iter()
            .filter(|p| p.predicate == pred)
            .map(|p| p.value.as_str())
            .collect();
        if values.is_empty() {
            continue;
        }

        let has_prov = result.annotated_triples.iter().any(|t| t.predicate == pred);
        let is_expanded = expanded_pred.as_deref() == Some(pred);
        prop_rows.push(property_row(display_name, &values, has_prov, is_expanded, pred));

        if is_expanded {
            let prov_triples: Vec<&AnnotatedTriple> = result
                .annotated_triples
                .iter()
                .filter(|t| t.predicate == pred)
                .collect();
            if !prov_triples.is_empty() {
                prop_rows.push(render_provenance_drawer(&prov_triples));
            }
        }
    }

    let mut seen = HashSet::new();
    for pv in &result.properties {
        if ordered_preds.contains(&pv.predicate.as_str()) {
            continue;
        }
        if !seen.insert(pv.predicate.as_str()) {
            continue;
        }

        let values: Vec<&str> = result
            .properties
            .iter()
            .filter(|p| p.predicate == pv.predicate)
            .map(|p| p.value.as_str())
            .collect();

        let has_prov = result
            .annotated_triples
            .iter()
            .any(|t| t.predicate == pv.predicate);
        let is_expanded = expanded_pred.as_deref() == Some(pv.predicate.as_str());
        let display = pv.predicate.to_uppercase();
        prop_rows.push(property_row(
            &display, &values, has_prov, is_expanded, &pv.predicate,
        ));

        if is_expanded {
            let prov_triples: Vec<&AnnotatedTriple> = result
                .annotated_triples
                .iter()
                .filter(|t| t.predicate == pv.predicate)
                .collect();
            if !prov_triples.is_empty() {
                prop_rows.push(render_provenance_drawer(&prov_triples));
            }
        }
    }

    container(
        column![header, Column::from_vec(prop_rows).spacing(2)].spacing(12),
    )
    .style(theme::card)
    .padding(16)
    .width(Fill)
    .into()
}

fn property_row(
    label: &str,
    values: &[&str],
    has_prov: bool,
    is_expanded: bool,
    pred: &str,
) -> Element<'static, Message> {
    let label_w = text(label.to_string())
        .size(11)
        .font(theme::latin())
        .color(theme::TEXT_SECONDARY);
    let value_w = text(values.join(", ")).size(14).color(theme::TEXT_COLOR);
    let mut r = row![label_w, value_w].spacing(12).align_y(Center);
    if has_prov {
        let arrow = if is_expanded { "▾" } else { "▸" };
        r = r.push(
            button(text(arrow).size(12))
                .on_press(Message::ToggleProvenance(pred.to_string()))
                .style(theme::tab_inactive)
                .padding([2, 6]),
        );
    }
    r.into()
}

fn render_provenance_drawer(triples: &[&AnnotatedTriple]) -> Element<'static, Message> {
    let mut rows: Vec<Element<'static, Message>> = Vec::new();
    for t in triples {
        for p in &t.provenance {
            rows.push(
                row![
                    text(format!("{} → {}", t.object, p.source))
                        .size(12)
                        .color(theme::TEXT_SECONDARY),
                    text(p.pramana.clone())
                        .size(11)
                        .color(theme::TEXT_SECONDARY),
                    text(format!("conf: {}", p.confidence))
                        .size(11)
                        .color(theme::TEXT_SECONDARY),
                ]
                .spacing(16)
                .into(),
            );
        }
    }

    container(Column::from_vec(rows).spacing(4))
        .style(theme::provenance_drawer)
        .padding([8, 24])
        .width(Fill)
        .into()
}

// ---- Search results ----

fn render_search<'a>(
    result: &'a SearchResult,
    expanded: &'a Option<ExpandedHit>,
    expanded_pred: &'a Option<String>,
) -> Element<'a, Message> {
    let header = text(format!("Found {} results", result.entities.len()))
        .size(14)
        .color(theme::TEXT_SECONDARY);

    let mut items: Vec<Element<'_, Message>> = Vec::new();
    for (i, hit) in result.entities.iter().enumerate() {
        let is_expanded = expanded.as_ref().map(|h| h.index) == Some(i);
        let arrow = if is_expanded { "▾" } else { "▸" };

        let hit_row = button(
            row![
                text(&hit.label).size(14).color(theme::TEXT_COLOR),
                space::horizontal(),
                text(arrow).size(12).color(theme::TEXT_SECONDARY),
            ]
            .align_y(Center)
            .width(Fill),
        )
        .on_press(Message::ToggleSearchHit(i))
        .style(theme::tab_inactive)
        .padding([6, 12])
        .width(Fill);

        items.push(hit_row.into());

        if is_expanded {
            if let Some(exp) = expanded {
                items.push(render_describe(&exp.describe, expanded_pred));
            }
        }
    }

    column![header, Column::from_vec(items).spacing(2)]
        .width(Fill)
        .into()
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

    container(
        column![header, Column::from_vec(items).spacing(2)].spacing(8),
    )
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

    container(
        column![header, Column::from_vec(assertions).spacing(4)].spacing(8),
    )
    .style(theme::card)
    .padding(16)
    .width(Fill)
    .into()
}

// ---- Empty / no-match states ----

fn empty_state<'a>() -> Element<'a, Message> {
    container(
        column![
            text("Search the knowledge graph")
                .size(18)
                .color(theme::TEXT_SECONDARY),
            text("Try typing a dravya name or click a preset above")
                .size(14)
                .color(theme::TEXT_SECONDARY),
        ]
        .spacing(8)
        .align_x(Center),
    )
    .center(Fill)
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

    let mut trace_rows: Vec<Element<'_, Message>> = vec![text(format!("Query mode: {mode}"))
        .size(11)
        .color(theme::TEXT_SECONDARY)
        .into()];
    trace_rows.extend(details);

    if !report.unknown_tokens.is_empty() {
        trace_rows.push(
            text(format!(
                "Unknown: {}",
                report.unknown_tokens.join(", ")
            ))
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
