use iced::widget::{Column, Row, button, column, container, scrollable, text};
use iced::{Center, Element, Fill, Theme};

use vidya_core::query::SearchHit;
use vidya_core::{DescribeResult, ProvenanceFilter, TypeSummary};

use super::Message;
use super::theme;
use super::widgets;
use crate::data::AppData;

pub struct KnowledgeGraphState {
    pub types: Vec<TypeSummary>,
    pub expanded_type: Option<usize>,
    pub entities: Vec<SearchHit>,
    pub expanded_entity: Option<ExpandedEntity>,
    pub expanded_predicate: Option<String>,
}

pub struct ExpandedEntity {
    pub index: usize,
    pub describe: DescribeResult,
}

impl KnowledgeGraphState {
    pub fn new() -> Self {
        Self {
            types: Vec::new(),
            expanded_type: None,
            entities: Vec::new(),
            expanded_entity: None,
            expanded_predicate: None,
        }
    }

    pub fn load(&mut self, data: &AppData) {
        self.types = data
            .store
            .type_summary(&data.active_domain)
            .unwrap_or_default();
        self.expanded_type = None;
        self.entities.clear();
        self.expanded_entity = None;
        self.expanded_predicate = None;
    }

    pub fn load_entities(&mut self, data: &AppData) {
        self.entities = if let Some(idx) = self.expanded_type {
            if let Some(ty) = self.types.get(idx) {
                data.store
                    .search(
                        &data.active_domain,
                        &ty.name,
                        &[],
                        &ProvenanceFilter::default(),
                    )
                    .map(|r| r.entities)
                    .unwrap_or_default()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        self.expanded_entity = None;
        self.expanded_predicate = None;
    }
}

pub(super) fn view<'a>(state: &'a KnowledgeGraphState, data: &'a AppData) -> Element<'a, Message> {
    let total_entities: usize = state.types.iter().map(|t| t.count).sum();
    let type_count = state.types.len();
    let triple_count = data.store.triple_count().unwrap_or(0);
    let coverage_pct = data
        .store
        .provenance_coverage(&data.active_domain)
        .map(|c| (c.coverage * 100.0).round() as u32)
        .unwrap_or(0);

    let stats = container(
        column![
            text(format!(
                "{total_entities} substances \u{00b7} {type_count} categories \u{00b7} {triple_count} textual relationships \u{00b7} {coverage_pct}% verse-cited"
            ))
            .size(13)
            .font(theme::latin())
            .color(theme::TEXT_SECONDARY),
            text("Each \u{201c}relationship\u{201d} is a structured fact linking a substance to a property \u{2014} e.g., Pippali \u{2192} has rasa \u{2192} Katu")
                .size(11)
                .font(theme::latin())
                .color(theme::TEXT_SECONDARY),
        ]
        .spacing(4)
        .align_x(Center),
    )
    .style(theme::stats_bar)
    .padding([8, 16])
    .width(Fill);

    let intro = text("Expand a category to browse its substances, then click one to see its properties and provenance back to the source verse.")
        .size(13)
        .font(theme::latin())
        .color(theme::TEXT_SECONDARY);

    let mut sections: Vec<Element<'_, Message>> = Vec::new();

    for (i, ty) in state.types.iter().enumerate() {
        let is_expanded = state.expanded_type == Some(i);
        let arrow = if is_expanded { "\u{25be}" } else { "\u{25b8}" };
        let header_label = format!("{arrow}  {} ({})", ty.name, ty.count);

        let header_style: fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style =
            if is_expanded {
                theme::category_btn_active
            } else {
                theme::category_btn_inactive
            };

        let header = button(text(header_label).size(14).font(theme::latin()))
            .on_press(Message::KgToggleType(i))
            .padding([8, 12])
            .style(header_style)
            .width(Fill);

        if !is_expanded {
            sections.push(header.into());
            continue;
        }

        let mut section_items: Vec<Element<'_, Message>> = vec![header.into()];

        if state.entities.is_empty() {
            section_items.push(
                container(
                    text("No entities found")
                        .size(13)
                        .color(theme::TEXT_SECONDARY),
                )
                .padding([8, 24])
                .into(),
            );
        } else {
            let pills: Vec<Element<'_, Message>> = state
                .entities
                .iter()
                .enumerate()
                .map(|(j, hit)| {
                    let is_entity_expanded =
                        state.expanded_entity.as_ref().map(|e| e.index) == Some(j);
                    let style: fn(
                        &Theme,
                        iced::widget::button::Status,
                    ) -> iced::widget::button::Style = if is_entity_expanded {
                        theme::category_btn_active
                    } else {
                        theme::dravya_pill
                    };
                    button(text(hit.label.as_str()).size(13))
                        .on_press(Message::KgExpandEntity(j))
                        .padding([4, 10])
                        .style(style)
                        .into()
                })
                .collect();

            section_items.push(
                container(Row::from_vec(pills).spacing(6).wrap())
                    .padding([8, 24])
                    .width(Fill)
                    .into(),
            );
        }

        if let Some(expanded) = &state.expanded_entity {
            section_items.push(
                container(
                    column![
                        button(
                            text("Close")
                                .size(12)
                                .font(theme::latin())
                                .color(theme::TEXT_SECONDARY),
                        )
                        .on_press(Message::KgCollapseEntity)
                        .style(theme::tab_inactive)
                        .padding([4, 8]),
                        widgets::render_describe(&expanded.describe, &state.expanded_predicate,),
                    ]
                    .spacing(8),
                )
                .padding([0, 24])
                .width(Fill)
                .into(),
            );
        }

        sections.push(
            container(Column::from_vec(section_items).spacing(4).width(Fill))
                .style(theme::card)
                .padding(8)
                .width(Fill)
                .into(),
        );
    }

    if state.types.is_empty() {
        sections.push(
            container(
                text("No entity types found in this domain")
                    .size(14)
                    .color(theme::TEXT_SECONDARY),
            )
            .center(Fill)
            .into(),
        );
    }

    let content = column![
        stats,
        intro,
        Column::from_vec(sections).spacing(4).width(Fill),
    ]
    .spacing(12)
    .padding([16, 24])
    .width(Fill)
    .max_width(900);

    let centered_layout = container(content).center_x(Fill);

    scrollable(centered_layout).height(Fill).into()
}
