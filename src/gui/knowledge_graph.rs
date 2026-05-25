use iced::widget::{button, column, container, row, scrollable, text, Column, Row};
use iced::{Element, Fill, Theme};

use vidya_core::DescribeResult;

use super::theme;
use super::widgets;
use super::Message;
use crate::data::AppData;

pub struct KnowledgeGraphState {
    pub active_category: usize,
    pub expanded_dravya: Option<ExpandedDravya>,
    pub expanded_predicate: Option<String>,
}

pub struct ExpandedDravya {
    pub index: usize,
    pub describe: DescribeResult,
}

impl KnowledgeGraphState {
    pub fn new() -> Self {
        Self {
            active_category: 0,
            expanded_dravya: None,
            expanded_predicate: None,
        }
    }
}

pub(super) fn view<'a>(state: &'a KnowledgeGraphState, data: &'a AppData) -> Element<'a, Message> {
    let total_dravyas: usize = data.catalog.iter().map(|c| c.dravyas.len()).sum();
    let triple_count = data.store.triple_count().unwrap_or(0);
    let domain_count = data.domains.len();
    let coverage_pct = data
        .store
        .provenance_coverage(&data.active_domain)
        .map(|c| (c.coverage * 100.0).round() as u32)
        .unwrap_or(0);

    let stats = container(
        text(format!(
            "{total_dravyas} dravyas \u{00b7} {triple_count} triples \u{00b7} {domain_count} domain(s) \u{00b7} {coverage_pct}% cited"
        ))
        .size(13)
        .font(theme::latin())
        .color(theme::TEXT_SECONDARY),
    )
    .style(theme::stats_bar)
    .padding([8, 16])
    .width(Fill);

    let categories: Vec<Element<'_, Message>> = data
        .catalog
        .iter()
        .enumerate()
        .map(|(i, cat)| {
            let label = format!("{} ({})", cat.name, cat.dravyas.len());
            let style: fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style =
                if i == state.active_category {
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

    let sidebar = container(scrollable(Column::from_vec(categories).spacing(2)).height(Fill))
        .width(200);

    let main_content: Element<'_, Message> = if let Some(cat) = data.catalog.get(state.active_category) {
        let mut items: Vec<Element<'_, Message>> = Vec::new();

        let pills: Vec<Element<'_, Message>> = cat
            .dravyas
            .iter()
            .enumerate()
            .map(|(i, name)| {
                let is_expanded = state.expanded_dravya.as_ref().map(|e| e.index) == Some(i);
                let style: fn(&Theme, iced::widget::button::Status) -> iced::widget::button::Style =
                    if is_expanded {
                        theme::category_btn_active
                    } else {
                        theme::dravya_pill
                    };
                button(text(name.as_str()).size(13))
                    .on_press(Message::KgExpandDravya(i))
                    .padding([4, 10])
                    .style(style)
                    .into()
            })
            .collect();

        items.push(Row::from_vec(pills).spacing(6).wrap().into());

        if let Some(expanded) = &state.expanded_dravya {
            let collapse_btn = button(
                text("Close")
                    .size(12)
                    .font(theme::latin())
                    .color(theme::TEXT_SECONDARY),
            )
            .on_press(Message::KgCollapseDravya)
            .style(theme::tab_inactive)
            .padding([4, 8]);

            items.push(collapse_btn.into());
            items.push(widgets::render_describe(
                &expanded.describe,
                &state.expanded_predicate,
            ));
        }

        Column::from_vec(items).spacing(12).width(Fill).into()
    } else {
        container(
            text("No categories available")
                .size(14)
                .color(theme::TEXT_SECONDARY),
        )
        .center(Fill)
        .into()
    };

    let body = row![
        sidebar,
        scrollable(
            container(main_content).padding([0, 16])
        )
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
