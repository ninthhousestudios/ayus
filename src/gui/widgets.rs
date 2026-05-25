use std::collections::HashSet;

use iced::widget::{button, column, container, row, text, Column, Row};
use iced::{Center, Element, Fill};

use vidya_core::query::AnnotatedTriple;
use vidya_core::DescribeResult;

use super::theme;
use super::Message;

pub(super) const PREDICATE_ORDER: &[(&str, &str)] = &[
    ("hasRasa", "RASA"),
    ("hasGuna", "GUNA"),
    ("hasVeerya", "VEERYA"),
    ("hasVipaka", "VIPAKA"),
    ("hasKarma", "KARMA"),
    ("pacifiesDosha", "PACIFIES"),
    ("aggravatesDosha", "AGGRAVATES"),
];

pub(super) fn render_describe<'a>(
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

pub(super) fn property_row(
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

pub(super) fn render_provenance_drawer(triples: &[&AnnotatedTriple]) -> Element<'static, Message> {
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
