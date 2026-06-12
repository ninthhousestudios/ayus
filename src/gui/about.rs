use iced::widget::{Column, column, container, rich_text, scrollable, span, text};
use iced::{Element, Fill};

use super::Message;
use super::theme;

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

    let about_me: Element<'_, Message> = container(
        column![
            text("About me")
                .size(16)
                .font(theme::latin())
                .color(theme::ACCENT),
            rich_text![
                span(
"My name is Josh Harper. I first met Amma in 2019 in Seattle. I was a driver for the \
summer tour in 2024 and I enjoy visiting Chicago and Ann Arbor satsangs when I can. I \
live in Indianapolis, so it isn't too far, but it also isn't too close.

When I was last at the Chicago ashram for the Spring Renewal retreat with Swamiji and \
Swamini I heard about the Amrita Institute for Advanced Research. I remembered that I \
was actually at the program in 2024 when it was announced, but I didn't really \
understand what it was about. I was curious after hearing about it again so I looked on \
the website to understand more about the Institute and what it is doing.

I have had many interests in life but somehow I ended up with a BS in Mathematics and a \
minor in Computer Science, graduating in 2012. About two years ago I started studying \
astrology. At the time I was not able to purchase the astrology software I wanted, so I \
wrote my own by hand in Python. I had always been interested in programming but never \
had a real idea of what I wanted to create, and this finally gave me something I was \
genuinely motivated to build. Since this February I have been writing software in \
collaboration with Claude. I now have ",
                    )
                    .size(13)
                    .font(theme::latin())
                    .color(theme::TEXT_COLOR),
                    span("four published apps")
                        .link("https://ninthhouse.studio/vendara/#market")
                        .size(13)
                        .font(theme::latin())
                        .color(theme::ACCENT)
                        .underline(true),
                    span(
" on Google Play, a desktop astrology application in progress, and a collection of bespoke
tools that support my daily workflows.

I was intrigued by the Agentic AI for Ayurveda Analytics project mentioned on \
the Projects page. I had been interested in having agents do astrology on some \
level. Claude has a great deal of astrological knowledge in its training data, \
but it isn't very reliable. This led to the idea of Vidya, a structured \
knowledge graph that gives an agent a canonical, verifiable source of domain \
knowledge instead of leaving it to rely on memory. I realized the same approach \
could apply to Ayurveda, so I built Ayus as a working demonstration of one way \
these ideas could serve the Institute's project.

Ayus was written in Rust, designed and directed by me in collaboration with \
Claude. I am currently pursuing a deeper study of Rust and machine learning. If \
the Institute has needs around software — prototypes, agentic systems, or \
knowledge infrastructure like Vidya — I would be glad to talk about how I could \
serve, whether as a volunteer or in a more formal capacity. Most of the \
conversation around AI right now is about using it to make more money faster; I \
think there are much better uses for this technology, and I would like to put my \
time toward them.

Om Namah Shivaya"
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
    .into();

    let sections: Vec<Element<'_, Message>> = vec![
        beyond_section,
        about_me,
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
        .width(Fill)
        .max_width(900);

    let centered_layout = container(content).center_x(Fill);

    scrollable(centered_layout).height(Fill).into()
}
