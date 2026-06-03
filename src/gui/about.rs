use iced::widget::{Column, column, container, rich_text, scrollable, span, text};
use iced::{Element, Fill};

use super::Message;
use super::theme;

const ABOUT_ME: &str = include_str!("../../docs/about-me.md");

fn section<'a>(title: &'a str, body: &'a str) -> Element<'a, Message> {
    container(
        column![
            text(title)
                .size(16)
                .font(theme::latin())
                .color(theme::ACCENT),
            text(body)
                .size(13)
                .font(theme::latin())
                .color(theme::TEXT_COLOR),
        ]
        .spacing(8),
    )
    .style(theme::card)
    .padding([12, 16])
    .width(Fill)
    .into()
}

pub(super) fn view<'a>() -> Element<'a, Message> {
    let beyond_section: Element<'_, Message> = container(
        column![
            text("Beyond This Demo")
                .size(16)
                .font(theme::latin())
                .color(theme::ACCENT),
            rich_text![
                span(
                    "Vidya is one layer of a broader cognitive infrastructure \
                     designed for tradition-rich, knowledge-dense domains. Three \
                     independent subsystems form the core:\n\n\
                     \u{2022} Vidya \u{2014} structured domain knowledge with provenance \
                     and tradition-scoping. What the domain knows, with citations \
                     to who said it and from which school. The foundation \
                     demonstrated here.\n\n\
                     \u{2022} Chitta (\u{091A}\u{093F}\u{0924}\u{094D}\u{0924}) \u{2014} \
                     a practitioner\u{2019}s research memory. Session notes, clinical \
                     observations, evolving hypotheses \u{2014} bi-temporal and \
                     semantically searchable. Not the domain\u{2019}s knowledge, but \
                     the researcher\u{2019}s relationship to it. ",
                )
                .size(13)
                .font(theme::latin())
                .color(theme::TEXT_COLOR),
                span("github.com/ninthhousestudios/chitta")
                    .link("https://github.com/ninthhousestudios/chitta".to_string())
                    .size(13)
                    .font(theme::latin())
                    .color(theme::ACCENT)
                    .underline(true),
                span(
                    "\n\n\
                     \u{2022} Kosha (\u{0915}\u{094B}\u{0936}) \u{2014} \
                     library perception. Classical texts, research papers, \
                     pharmacopeias \u{2014} indexed with semantic search and citation \
                     anchoring down to the page or verse. ",
                )
                .size(13)
                .font(theme::latin())
                .color(theme::TEXT_COLOR),
                span("github.com/ninthhousestudios/kosha")
                    .link("https://github.com/ninthhousestudios/kosha".to_string())
                    .size(13)
                    .font(theme::latin())
                    .color(theme::ACCENT)
                    .underline(true),
                span(
                    "\n\n\
                     Each subsystem is domain-agnostic. Chitta holds whatever a \
                     practitioner writes, whether that practitioner is a vaidya or \
                     an astronomer. Kosha indexes whatever texts you point it at. \
                     Vidya hosts whatever domain you load.\n\n\
                     An AI agent with access to all three can cross-reference \
                     verified domain facts (vidya) with the researcher\u{2019}s own \
                     notes and hypotheses (chitta) and primary source texts (kosha) \
                     in a single query \u{2014} without hallucinating, because every \
                     assertion carries provenance. These subsystems have working \
                     implementations today. A desktop workspace shell to bring \
                     them together visually is the natural next step.\n\n\
                     A fuller design document for this cognitive workbench idea is available here: ",
                )
                .size(13)
                .font(theme::latin())
                .color(theme::TEXT_COLOR),
                span("cognitive-workbench.md")
                    .link("https://github.com/ninthhousestudios/ayus/blob/main/docs/cognitive-workbench.md".to_string())
                    .size(13)
                    .font(theme::latin())
                    .color(theme::ACCENT)
                    .underline(true),
            ]
            .on_link_click(Message::LinkClicked),
        ]
        .spacing(8),
    )
    .style(theme::card)
    .padding([12, 16])
    .width(Fill)
    .into();

    let sections: Vec<Element<'_, Message>> = vec![
        beyond_section,
        section("About Me", ABOUT_ME),
        container(
            column![
                text("Contact")
                    .size(16)
                    .font(theme::latin())
                    .color(theme::ACCENT),
                rich_text![
                    span("josh@ninthhouse.studio")
                        .link("mailto:josh@ninthhouse.studio".to_string())
                        .size(13)
                        .font(theme::latin())
                        .color(theme::ACCENT)
                        .underline(true),
                ]
                .on_link_click(Message::LinkClicked),
            ]
            .spacing(8),
        )
        .style(theme::card)
        .padding([12, 16])
        .width(Fill)
        .into(),
        section(
            "License",
            "Ayus is licensed under the AGPL-3.0. Most of my projects are \
             because I believe in open source and openness in general. However, \
             I would be willing to donate this code or any other code I have or \
             will write to Amma if that is what She desires.",
        ),
    ];

    let content = Column::from_vec(sections)
        .spacing(12)
        .padding([16, 24])
        .width(Fill);

    scrollable(content).height(Fill).into()
}
