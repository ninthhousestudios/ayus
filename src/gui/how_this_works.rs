use iced::widget::{Column, column, container, rich_text, scrollable, span, text};
use iced::{Element, Fill};

use super::Message;
use super::theme;

const CH26_PROMPT: &str = include_str!("../../extraction/extract-charaka-ss-26.md");
const CH27_PROMPT: &str = include_str!("../../extraction/extract-charaka-ss-27.md");

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

fn prompt_block<'a>(title: &'a str, content: &'a str) -> Element<'a, Message> {
    container(
        column![
            text(title)
                .size(14)
                .font(theme::latin())
                .color(theme::ACCENT),
            container(
                scrollable(
                    text(content)
                        .size(11)
                        .font(iced::Font::MONOSPACE)
                        .color(theme::TEXT_SECONDARY),
                )
                .height(300),
            )
            .style(theme::provenance_drawer)
            .padding(12)
            .width(Fill),
        ]
        .spacing(8),
    )
    .style(theme::card)
    .padding([12, 16])
    .width(Fill)
    .into()
}

pub(super) fn view<'a>() -> Element<'a, Message> {
    let vidya_section: Element<'_, Message> = container(
        column![
            text("What is Vidya")
                .size(16)
                .font(theme::latin())
                .color(theme::ACCENT),
            rich_text![
                span(
                    "Vidya is a domain-agnostic knowledge store built on RDF/Turtle and \
                      Oxigraph. It provides both a human interface (a CLI) and an agent \
                      interface (MCP server) for browsing, querying, and reasoning over \
                      structured knowledge from traditional texts. Vidya is not tied to \
                      Ayurveda \u{2014} the same infrastructure can host Jyotish, Vastu, or any \
                      domain where source texts encode structured relationships. "
                )
                .size(13)
                .font(theme::latin())
                .color(theme::TEXT_COLOR),
                span("github.com/ninthhousestudios/vidya")
                    .link("https://github.com/ninthhousestudios/vidya".to_string())
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

    let ayus_section: Element<'_, Message> = container(
        column![
            text("What is Ayus")
                .size(16)
                .font(theme::latin())
                .color(theme::ACCENT),
            rich_text![
                span(
                    "Ayus is a demonstration of Vidya applied to Ayurvedic pharmacology. \
                     It contains dravya (substance) data extracted from the Charaka Samhita, \
                     Sutra Sthana — chapter 26 (rasa theory: the six tastes, dosha mappings, \
                     veerya, vipaka, and viruddha ahara) and chapter 27 (the dravya catalog: \
                     grains, pulses, meats, vegetables, fruits, wines, waters, dairy, and more).\n\n\
                     Every extracted fact carries verse-level provenance — you can \
                     trace any property back to the specific verse it came from.\n\n\
                     This is not expert-curated data. It was extracted using Claude (Anthropic's \
                     LLM) to demonstrate what a non-specialist with AI access can produce from \
                     classical texts. The knowledge would need to be verified and curated by \
                     domain experts before any scholarly or clinical use.",
                )
                    .size(13)
                    .font(theme::latin())
                    .color(theme::TEXT_COLOR),
                span("github.com/ninthhousestudios/ayus")
                    .link("https://github.com/ninthhousestudios/ayus".to_string())
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
        vidya_section,
        ayus_section,
        section(
            "Limitations",
            "\u{2022} Small dataset: ~95 dravyas from two chapters of one text. \
             Projects like GRAYU have 157K nodes.\n\
             \u{2022} Single source: only the Charaka Samhita SS ch.26\u{2013}27 so far.\n\
             \u{2022} LLM-extracted: Claude did the extraction, not a vaidya or \
             Ayurveda scholar. Errors are likely.\n\
             \u{2022} No clinical applicability: this is a knowledge exploration tool, \
             not medical guidance.\n\
             \u{2022} Verse-Substance Attribution (VSA) is approximate: the OCR text \
             doesn't always have clean verse boundaries, so some citations may be \
             off by a verse.",
        ),
        container(
            column![
                text("For Researchers")
                    .size(16)
                    .font(theme::latin())
                    .color(theme::ACCENT),
                rich_text![
                    span(
                        "\u{2022} Load your own TTL: use the \u{201c}Load TTL\u{201d} button to bring in \
                         your own RDF data. The viewer works with any domain. Try it with ",
                    )
                    .size(13)
                    .font(theme::latin())
                    .color(theme::TEXT_COLOR),
                    span("jyotish.ttl")
                        .link("https://github.com/ninthhousestudios/vidya/blob/main/seeds/jyotish.ttl".to_string())
                        .size(13)
                        .font(theme::latin())
                        .color(theme::ACCENT)
                        .underline(true),
                    span(
                        " from the Vidya repo \u{2014} the Vidya README has example queries \
                         you can try in the Explorer tab once the jyotish data is loaded.\n\
                         \u{2022} Curate extractions: the JSON output is human-editable. Fix \
                         errors, add missing substances, refine citations, then regenerate TTL.\n\
                         \u{2022} MCP for agent workflows: Vidya exposes an MCP server so AI \
                         agents can query the knowledge graph programmatically. Ayus itself \
                         currently does not expose an MCP, but that is easy to add.\n\
                         \u{2022} Repeatable methodology: the extraction prompt structure works \
                         with other chapters and texts. Adapt the prompts, run extraction, \
                         validate, convert.",
                    )
                    .size(13)
                    .font(theme::latin())
                    .color(theme::TEXT_COLOR),
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
            "Extraction Pipeline",
            "Source text (OCR'd public-domain translation)\n\
             \u{2192} LLM extraction prompt (structured task with JSON schema)\n\
             \u{2192} JSON output (substances, properties, verse citations)\n\
             \u{2192} Validation (validate.py checks schema, cross-references)\n\
             \u{2192} TTL generation (json2ttl converts to RDF/Turtle)\n\
             \u{2192} Oxigraph (loaded into the knowledge store at startup)\n\n\
             The actual extraction prompts given to Claude are shown below. \
             The methodology is designed to be repeatable — give the same prompt \
             structure to a different chapter or text and produce comparable output.",
        ),
        prompt_block("Extraction prompt: Ch.26 (Rasa Theory)", CH26_PROMPT),
        prompt_block("Extraction prompt: Ch.27 (Dravya Catalog)", CH27_PROMPT),
    ];

    let content = Column::from_vec(sections)
        .spacing(12)
        .padding([16, 24])
        .width(Fill)
        .max_width(900);

    let centered_layout = container(content).center_x(Fill);

    scrollable(centered_layout).height(Fill).into()
}
