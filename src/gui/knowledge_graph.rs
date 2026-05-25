use iced::widget::{button, column, container, row, scrollable, text, Column, Row};
use iced::{Element, Fill, Length, Theme};

use vidya_core::query::SearchHit;
use vidya_core::{DescribeResult, ProvenanceFilter, TypeSummary};

use super::theme;
use super::widgets;
use super::Message;
use crate::data::AppData;

pub struct KnowledgeGraphState {
    pub types: Vec<TypeSummary>,
    pub active_type: usize,
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
            active_type: 0,
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
        self.active_type = 0;
        self.load_entities(data);
    }

    pub fn load_entities(&mut self, data: &AppData) {
        self.entities = if let Some(ty) = self.types.get(self.active_type) {
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
        };
        self.expanded_entity = None;
        self.expanded_predicate = None;
    }
}

pub(super) fn view<'a>(
    state: &'a KnowledgeGraphState,
    data: &'a AppData,
) -> Element<'a, Message> {
    let total_entities: usize = state.types.iter().map(|t| t.count).sum();
    let type_count = state.types.len();
    let triple_count = data.store.triple_count().unwrap_or(0);
    let coverage_pct = data
        .store
        .provenance_coverage(&data.active_domain)
        .map(|c| (c.coverage * 100.0).round() as u32)
        .unwrap_or(0);

    let stats = container(
        text(format!(
            "{total_entities} entities \u{00b7} {type_count} types \u{00b7} {triple_count} triples \u{00b7} {coverage_pct}% cited"
        ))
        .size(13)
        .font(theme::latin())
        .color(theme::TEXT_SECONDARY),
    )
    .style(theme::stats_bar)
    .padding([8, 16])
    .width(Fill);

    let type_buttons: Vec<Element<'_, Message>> = state
        .types
        .iter()
        .enumerate()
        .map(|(i, ty)| {
            let label = format!("{} ({})", ty.name, ty.count);
            let style: fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style =
                if i == state.active_type {
                    theme::category_btn_active
                } else {
                    theme::category_btn_inactive
                };
            button(text(label).size(13).font(theme::latin()))
                .on_press(Message::KgSelectCategory(i))
                .padding([6, 12])
                .style(style)
                .width(Fill)
                .into()
        })
        .collect();

    let sidebar = container(
        scrollable(Column::from_vec(type_buttons).spacing(2)).height(Fill),
    )
    .width(Length::Fixed(200.0))
    .height(Fill);

    let main_content: Element<'_, Message> = if state.entities.is_empty() {
        container(
            text("No entities")
                .size(14)
                .color(theme::TEXT_SECONDARY),
        )
        .center(Fill)
        .into()
    } else {
        let mut items: Vec<Element<'_, Message>> = Vec::new();

        let pills: Vec<Element<'_, Message>> = state
            .entities
            .iter()
            .enumerate()
            .map(|(i, hit)| {
                let is_expanded =
                    state.expanded_entity.as_ref().map(|e| e.index) == Some(i);
                let style: fn(
                    &Theme,
                    iced::widget::button::Status,
                ) -> iced::widget::button::Style = if is_expanded {
                    theme::category_btn_active
                } else {
                    theme::dravya_pill
                };
                button(text(hit.label.as_str()).size(13))
                    .on_press(Message::KgExpandEntity(i))
                    .padding([4, 10])
                    .style(style)
                    .into()
            })
            .collect();

        items.push(Row::from_vec(pills).spacing(6).wrap().into());

        if let Some(expanded) = &state.expanded_entity {
            let collapse_btn = button(
                text("Close")
                    .size(12)
                    .font(theme::latin())
                    .color(theme::TEXT_SECONDARY),
            )
            .on_press(Message::KgCollapseEntity)
            .style(theme::tab_inactive)
            .padding([4, 8]);

            items.push(collapse_btn.into());
            items.push(widgets::render_describe(
                &expanded.describe,
                &state.expanded_predicate,
            ));
        }

        Column::from_vec(items).spacing(12).width(Fill).into()
    };

    let body = row![
        sidebar,
        scrollable(container(main_content).padding([0, 16]))
            .height(Fill)
            .width(Fill)
    ]
    .spacing(16)
    .height(Fill);

    column![stats, body]
        .spacing(12)
        .padding([16, 24])
        .height(Fill)
        .into()
}
