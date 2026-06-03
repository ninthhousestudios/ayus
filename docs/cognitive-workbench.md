# cognitive workbench architecture

A general framework for AI-assisted expert work in tradition-rich,
knowledge-dense domains.

## Origin

This architecture emerged from the design of
[aion](https://github.com/ninthhousestudios/aion), a desktop application
for professional astrologers. A key decision in that design was that the
core application would have zero domain knowledge — it would be a
workspace shell, with all domain functionality provided by MCP servers
(plugins). That decision, combined with subsystems that were already
shared across projects, revealed a pattern that applies well beyond
astrology.

## The pattern

A human expert works in a domain where knowledge is structured,
traditions matter, provenance matters, and understanding accumulates over
time. They need:

- Verified domain facts that an AI cannot hallucinate, with citations
  back to source texts and traditions.
- Their own research notes, observations, and hypotheses — searchable,
  cross-referenceable, persistent across sessions.
- A reference library of primary texts, indexed for semantic search and
  citation anchoring.
- An AI agent that can draw on all three simultaneously in a single
  interaction.

The architecture separates these concerns into independent subsystems,
each accessible over MCP, each domain-agnostic.

## Cognitive infrastructure

Three subsystems form the core. Each is a standalone Rust project with
its own storage, embedding pipeline, and MCP server. They share no
state and communicate only through the MCP host.

### Vidya — domain knowledge

[github.com/ninthhousestudios/vidya](https://github.com/ninthhousestudios/vidya)

Structured knowledge graph backed by Oxigraph (RDF/Turtle). Every
assertion carries provenance: which tradition, which source text, what
epistemological basis (pramana), what confidence. Supports multiple
traditions that overlap but diverge — the same entity can have different
properties depending on who you ask, and vidya tracks that rather than
flattening it.

Natural language resolution maps casual queries to graph operations.
VSA/HRR encodes entities as high-dimensional vectors for structural
similarity search.

Domain knowledge is loaded as seed files (.ttl). Switching domains
means loading a different seed. The engine doesn't change.

**What belongs in vidya:** facts about the domain — entities, their
properties, their relationships, as asserted by authoritative sources.
Not personal notes, not project tasks, not source text content.

### Chitta — practitioner cognition

[github.com/ninthhousestudios/chitta](https://github.com/ninthhousestudios/chitta)

Personal cognition store for practitioners and the agents that work with
them. Two complementary roles: a place where practitioners record their
own thinking (research notes, session observations, hypotheses), and a
place where agents accumulate a working model of the person (preferences,
patterns, values).

Bi-temporal storage (event time and record time), profile-isolated,
semantically searchable via hybrid dense+sparse retrieval (BGE-M3,
pgvector). Memory types are free text — each deployment defines its own
vocabulary.

**What belongs in chitta:** what is known about or by the person. Their
observations, their decisions, their patterns. Not domain facts (vidya),
not source text content (kosha), not project artifacts (yojana).

### Kosha — library perception

[github.com/ninthhousestudios/kosha](https://github.com/ninthhousestudios/kosha)

Filesystem and content perception. Indexes a library of reference
material — PDFs, classical texts, research papers — with semantic search
and citation anchoring down to the page or verse.

Kosha owns the source. Chitta owns the practitioner's relationship to
the source. A research note in chitta cites a passage by reference; the
actual content lives in kosha and is fetched on demand.

**What belongs in kosha:** the reference material itself — books, papers,
texts. Not the practitioner's notes about those texts (chitta), not
structured domain facts extracted from those texts (vidya).

### The separation matters

These three subsystems model genuinely different things:

| Subsystem | Holds | Example |
|---|---|---|
| Vidya | What the domain knows | "Ashwagandha has ushna veerya, per Charaka" |
| Chitta | What the person thinks | "I suspect ashwagandha dose matters more than duration" |
| Kosha | What the sources say | The actual text of Charaka Samhita, ch.4, v.12 |

Most systems conflate these — notes, facts, and source text all land in
the same RAG store. The result is an AI that cannot distinguish between
a verified domain fact, a researcher's speculative hypothesis, and a
passage from a book. Separating them means the AI knows what it's
drawing on and can cite accordingly.

## Workspace shell

The cognitive infrastructure is headless — three MCP servers that any
client can connect to. A desktop workspace shell brings them together
as a visual, interactive environment for the practitioner.

### What the shell provides

The shell is an interaction *framework*. It provides structure and
surfaces, but never hardcodes what appears in them. Plugins populate
every domain-specific element at runtime.

**Canvas and cards.** An infinite canvas with a card system. Each card
binds to a data slot and a renderer. Cards have no domain knowledge —
they say "show slot X using renderer Y with config Z." The card system
handles position, size, z-order, constraints, snap physics, and
animation. What a card *displays* is entirely determined by the
renderer plugin it's bound to.

**Layout presets.** Saved as JSON. Reference slots, not specific data.
Shareable, exportable. A clinician's layout ("patient profile left,
search results center, source text right") works for any domain that
provides those card types.

**MCP host.** Discovers, launches, and manages plugin lifecycles. The
shell is the single host; all tool calls — from the AI or from UI
interactions — route through it.

### Three input modes

The shell is designed to be human-native and agent-native
simultaneously. Every action that can be performed manually can also be
performed by voice or by the AI agent.

**Manual.** Mouse, keyboard, gesture. Context menus, command palette
(ctrl+k or /), drag-and-drop from a card palette. The traditional
desktop interaction model.

**Voice.** Local speech-to-text (whisper.cpp or equivalent) captures
the practitioner's speech and routes it. Two modes:

- *Command*: "show me all ushna veerya dravyas" — transcribed text goes
  to the AI, which interprets it as tool calls against the loaded
  plugins.
- *Dictation*: "note: patient showed improved digestion after two
  weeks on the modified formulation" — transcribed text goes directly
  to chitta as a new memory, tagged with the current workspace context
  (which patient, which session, which domain).

The distinction can be explicit (push-to-talk for command, always-on
for dictation) or inferred by the AI from context.

**Agent.** The AI can perform any action the human can — open cards,
run queries, create notes, rearrange the workspace. It operates
through the same MCP tool calls that back the manual UI, so there is
no separate "agent API." When the AI opens a card or runs a search,
the practitioner sees it happen on the canvas in real time.

### Domain-aware interactions

Context menus, command palette entries, and card types are all
domain-specific — but the shell doesn't know what domain it's in. The
solution: plugins *declare* their UI contributions, and the shell
*discovers* them.

**Context menus.** When the user right-clicks a card, the shell asks
loaded plugins: "what actions do you offer for an entity of this type
in this context?" Each plugin responds with menu items. The shell
composes them into a single menu. An ayurveda plugin might offer
"show interactions" and "compare with similar dravyas." An astrology
plugin might offer "cast transit chart" and "show dasha timeline."
The shell renders whatever it receives.

**Command palette.** Plugins register commands with labels and
categories. The palette aggregates them. The shell provides fuzzy
search and keyboard shortcut binding; plugins provide the commands
themselves.

**Card types.** Plugins register renderable card types with metadata
(name, icon, supported slot types, default size). The card palette
shows whatever types are available from loaded plugins. Generic types
(property table, relationship graph, text viewer, note card) come
from the shell's built-in renderers. Domain-specific types (chart
wheel, herb profile, meridian diagram) come from domain plugins.

**Settings panels.** Plugins can contribute settings sections. The
shell provides the settings UI framework; plugins populate it.

This means installing a domain package changes everything the user
sees — menus, commands, available card types, settings — without the
shell's code changing at all.

### Built-in renderers

The shell ships with generic renderers that work for any vidya domain:

- **Property table** — entity attributes, values, provenance badges.
- **Relationship graph** — interactive node-link visualization of
  entity relationships from the knowledge graph.
- **Search results** — list view with type icons, attribute previews,
  similarity scores.
- **Source text** — kosha content with citation highlighting and
  navigation.
- **Note card** — chitta memory display and inline editing.
- **Provenance drawer** — expandable detail showing tradition, source,
  pramana, and confidence for any assertion.

These cover the common case. A researcher exploring a new domain
through vidya gets a functional workspace immediately, before any
domain-specific renderers exist.

### Current state

A canvas interaction prototype exists
([aion](https://github.com/ninthhousestudios/aion),
Flutter desktop) covering card physics, gesture handling, and layout
animation. The domain-aware plugin contribution system described above
is design-level — not yet implemented. The cognitive subsystems
underneath are built and working.

## Domain packages

Each domain provides the pieces that make the framework specific:

- **Vidya seed** (.ttl) — the knowledge graph for the domain.
- **Synonym table** — English-to-domain vocabulary mapping for natural
  language resolution.
- **Computation engine** (if applicable) — domain-specific calculation
  that goes beyond graph lookup. Ephemeris calculations for astrology,
  pharmacological interaction modeling for medicine, etc.
- **Renderers** — domain-specific visualizations. Chart wheels, herb
  profiles, meridian diagrams, case timelines.
- **Vector schema** — what dimensions to encode for similarity search
  over domain objects. Chart vectors for astrology, compound profiles
  for pharmacology, patient symptom vectors for clinical work.
- **Chitta vocabulary** — what memory types practitioners use in this
  domain. An astrologer records `transit_observation` and
  `client_session`. A researcher records `case_note` and
  `lab_observation`.
- **Import/export handlers** — domain-specific file formats.

## AI integration

The architecture's value for AI is that it constrains the agent to
verified knowledge while preserving the ability to synthesize across
sources.

```
user query
    |
    v
LLM (local or cloud)
    |-- tool calls via MCP
    v
+------------------------------------------+
| vidya    (domain facts with provenance)  |
| chitta   (practitioner's notes/model)    |
| kosha    (source texts, citations)       |
| domain plugins (computation, rendering)  |
+------------------------------------------+
    |
    v
LLM synthesizes response with citations
```

A single interaction can cross-reference all three subsystems:

> "Which of my patients might benefit from ashwagandha based on their
> symptom profiles, and what did I note about contraindications last
> month?"
>
> vidya: ashwagandha properties, interactions, contraindication rules
> chitta: the clinician's notes from last month about contraindications
> kosha: the relevant passage from Charaka Samhita
> domain plugin: patient similarity search over symptom vectors
> synthesis: specific patients, cited reasoning, the clinician's own
> prior observations woven in

The LLM cannot invent domain facts because it must call through vidya,
which returns only assertions that exist in the knowledge graph with
their provenance. It can still reason, synthesize, and suggest — but
its factual claims are grounded.

## Where this pattern fits

The framework fits domains with these characteristics:

| Domain | Traditions diverge? | Structured knowledge? | Authoritative texts? | Expert accumulation? |
|---|---|---|---|---|
| Ayurveda / dravyaguna | Yes (Charaka, Sushruta, Vagbhata) | Deeply (rasa, guna, veerya, vipaka) | Yes (Samhitas, Nighantus) | Yes (clinical cases, research notes) |
| Jyotish (Vedic astrology) | Yes (Parashara, Jaimini, Tajika) | Deeply (grahas, rashis, bhavas, yogas) | Yes (BPHS, Jataka Parijata) | Yes (chart notes, client sessions) |
| Traditional Chinese Medicine | Yes (schools, historical eras) | Deeply (meridians, herbs, formulas) | Yes (Huang Di Nei Jing, Shang Han Lun) | Yes (clinical cases) |
| Vastu Shastra | Yes (regional, textual) | Yes (directions, elements, ratios) | Yes (Manasara, Mayamatam) | Moderate |
| Homeopathy | Yes (classical vs. modern) | Yes (repertory, materia medica) | Yes (provings, Kent, Boericke) | Yes (case records) |
| Classical Indian music | Yes (gharanas) | Yes (ragas, talas, rules) | Yes (Sangita Ratnakara) | Yes (compositions, pedagogy) |
| Herbalism / ethnobotany | Yes (regional traditions) | Yes (preparations, interactions) | Mixed (pharmacopeias, field literature) | Yes (field notes, clinical) |

Common characteristics: multiple legitimate traditions that overlap but
diverge, structured relational knowledge (graphs not documents),
authoritative source texts, and practitioners who build personal
understanding over time.

The pattern is a poor fit for domains with a single canonical truth
(physics), domains where knowledge changes too rapidly (news), purely
procedural domains (manufacturing), or domains that are fundamentally
computational rather than interpretive.

## Current state

| Component | Status |
|---|---|
| Vidya | Working. Two active domain examples (jyotish, ayurveda). CLI + MCP server. |
| Chitta | Working. Running in production for manas. MCP server + reflect pipeline. |
| Kosha | Working. Foundation implementation. |
| Workspace shell | Prototype only (Flutter canvas spike). |
| Ayurveda domain package | Demonstrated by [ayus](https://github.com/ninthhousestudios/ayus). Seed data from Charaka Samhita SS ch.26-27. |
| Jyotish domain package | Vidya seed (~1029 triples). No workspace integration. |

The cognitive infrastructure is built. The workspace shell and full
domain packages are future work.
