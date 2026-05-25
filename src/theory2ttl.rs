use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct TheoryExtraction {
    source: Source,
    rasa_dosha_mappings: Vec<RasaDoshaMapping>,
    rasa_guna_associations: Vec<RasaGunaAssociation>,
    rasa_guna_rankings: std::collections::HashMap<String, GunaRanking>,
    vipaka_rules: VipakaRules,
    veerya_rules: VeeryaRules,
    hierarchy: Hierarchy,
    prabhava_definition: PrabhavaDefinition,
    viruddha: Vec<ViruddhaEntry>,
}

#[derive(Deserialize)]
struct Source {
    text: String,
    section: String,
}

#[derive(Deserialize)]
struct RasaDoshaMapping {
    rasa: String,
    pacifies: Vec<String>,
    aggravates: Vec<String>,
    verses: Vec<u32>,
    confidence: f64,
    #[allow(dead_code)]
    source_excerpt: String,
    #[allow(dead_code)]
    notes: Option<String>,
}

#[derive(Deserialize)]
struct RasaGunaAssociation {
    rasa: String,
    guna: Vec<String>,
    verses: Vec<u32>,
    #[allow(dead_code)]
    source_excerpt: String,
}

#[derive(Deserialize)]
struct GunaRanking {
    foremost: String,
    middling: String,
    inferior: String,
    verses: Vec<u32>,
    #[allow(dead_code)]
    source_excerpt: String,
    #[allow(dead_code)]
    notes: Option<String>,
}

#[derive(Deserialize)]
struct VipakaRules {
    #[allow(dead_code)]
    description: String,
    rules: Vec<VipakaRule>,
    vipaka_effects: Vec<VipakaEffect>,
}

#[derive(Deserialize)]
struct VipakaRule {
    input_rasas: Vec<String>,
    vipaka: String,
    verses: Vec<u32>,
    #[allow(dead_code)]
    source_excerpt: String,
}

#[derive(Deserialize)]
struct VipakaEffect {
    vipaka: String,
    effects: String,
    verses: Vec<u32>,
    #[allow(dead_code)]
    source_excerpt: String,
}

#[derive(Deserialize)]
struct VeeryaRules {
    #[allow(dead_code)]
    description: String,
    two_fold: VeeryaFold,
    eight_fold: VeeryaFold,
    #[allow(dead_code)]
    default_veerya_by_rasa: serde_json::Value,
}

#[derive(Deserialize)]
struct VeeryaFold {
    types: Vec<String>,
    #[allow(dead_code)]
    verses: Vec<u32>,
    #[allow(dead_code)]
    source_excerpt: String,
}

#[derive(Deserialize)]
struct Hierarchy {
    order: Vec<String>,
    #[allow(dead_code)]
    strongest: String,
    #[allow(dead_code)]
    weakest: String,
    verses: Vec<u32>,
    #[allow(dead_code)]
    source_excerpt: String,
    #[allow(dead_code)]
    verses_note: Option<String>,
}

#[derive(Deserialize)]
struct PrabhavaDefinition {
    #[allow(dead_code)]
    description: String,
    #[allow(dead_code)]
    verses: Vec<u32>,
    #[allow(dead_code)]
    source_excerpt: String,
    example: PrabhavaExample,
    #[allow(dead_code)]
    verses_note: Option<String>,
}

#[derive(Deserialize)]
struct PrabhavaExample {
    substance_1: String,
    substance_2: String,
    shared: String,
    difference: String,
    explanation: String,
}

#[derive(Deserialize)]
struct ViruddhaEntry {
    combination: String,
    effect: String,
    verses: Vec<u32>,
    #[allow(dead_code)]
    source_excerpt: String,
    confidence: f64,
    #[allow(dead_code)]
    notes: Option<String>,
}

fn escape_ttl(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn viruddha_id(combination: &str) -> String {
    combination
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

fn write_prefixes(out: &mut String) {
    out.push_str(
        "@prefix rdf:      <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .\n\
         @prefix rdfs:     <http://www.w3.org/2000/01/rdf-schema#> .\n\
         @prefix xsd:      <http://www.w3.org/2001/XMLSchema#> .\n\
         @prefix vidya:    <http://vidya.ninthhouse.studio/ontology/> .\n\
         @prefix ayurveda: <http://vidya.ninthhouse.studio/domain/ayurveda/> .\n\n",
    );
}

fn write_provenance(out: &mut String, subj: &str, pred: &str, obj: &str, verses: &[u32], confidence: f64) {
    let verse = verses.first().copied().unwrap_or(0);
    let _ = writeln!(out, "<< {subj} {pred} {obj} >>");
    let _ = writeln!(out, "    vidya:assertedBy [");
    let _ = writeln!(out, "        vidya:tradition  ayurveda:tradition-atreya ;");
    let _ = writeln!(out, "        vidya:source     ayurveda:source-charaka ;");
    let _ = writeln!(out, "        vidya:sthana     \"Sutra Sthana\" ;");
    let _ = writeln!(out, "        vidya:chapter    \"26\"^^xsd:integer ;");
    let _ = writeln!(out, "        vidya:verse      \"{}\"^^xsd:integer ;", verse);
    let _ = writeln!(out, "        vidya:pramana    vidya:shabda ;");
    let _ = writeln!(out, "        vidya:confidence \"{}\"^^xsd:float", confidence);
    let _ = writeln!(out, "    ] .\n");
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: theory2ttl <input.json> [output.ttl]");
        std::process::exit(1);
    }

    let input = PathBuf::from(&args[1]);
    let output = if args.len() >= 3 {
        PathBuf::from(&args[2])
    } else {
        input.with_extension("ttl")
    };

    let json_str = fs::read_to_string(&input)
        .with_context(|| format!("reading {}", input.display()))?;
    let extraction: TheoryExtraction = serde_json::from_str(&json_str)
        .with_context(|| "parsing JSON")?;

    let mut ttl = String::with_capacity(64 * 1024);
    let mut triple_count: usize = 0;

    let _ = writeln!(
        ttl,
        "# Theory data extracted from {}, {} (Kaviratna translation, public domain).\n\
         # Generated by theory2ttl from extraction JSON.\n",
        extraction.source.text, extraction.source.section,
    );

    write_prefixes(&mut ttl);

    // -- New classes and properties for ch.26 theory --

    ttl.push_str("# ══════════════════════════════════════════════════\n");
    ttl.push_str("# Theory Classes and Properties\n");
    ttl.push_str("# ══════════════════════════════════════════════════\n\n");

    ttl.push_str("ayurveda:VipakaRule   a rdfs:Class ; rdfs:label \"vipaka-rule\" .\n");
    ttl.push_str("ayurveda:ViruddhaRule a rdfs:Class ; rdfs:label \"viruddha-rule\" .\n\n");

    ttl.push_str("ayurveda:inputRasa     a rdf:Property ; rdfs:domain ayurveda:VipakaRule ; rdfs:range ayurveda:Rasa .\n");
    ttl.push_str("ayurveda:resultVipaka  a rdf:Property ; rdfs:domain ayurveda:VipakaRule ; rdfs:range ayurveda:Rasa .\n");
    ttl.push_str("ayurveda:vipakaEffect  a rdf:Property ; rdfs:domain ayurveda:Rasa .\n");
    ttl.push_str("ayurveda:combination   a rdf:Property ; rdfs:domain ayurveda:ViruddhaRule .\n");
    ttl.push_str("ayurveda:effect        a rdf:Property ; rdfs:domain ayurveda:ViruddhaRule .\n");
    ttl.push_str("ayurveda:gunaRank      a rdf:Property .\n");
    ttl.push_str("ayurveda:hierarchyRank a rdf:Property .\n\n");

    // -- Rasa-dosha mappings --

    ttl.push_str("# ══════════════════════════════════════════════════\n");
    ttl.push_str("# Rasa-Dosha Mappings\n");
    ttl.push_str("# ══════════════════════════════════════════════════\n\n");

    for mapping in &extraction.rasa_dosha_mappings {
        let _ = writeln!(ttl, "# {} ({})", mapping.rasa, if mapping.pacifies.is_empty() { "" } else { "rasa-dosha theory" });
        for d in &mapping.pacifies {
            let _ = writeln!(ttl, "ayurveda:{}  ayurveda:pacifiesDosha   ayurveda:{} .", mapping.rasa, d);
            triple_count += 1;
        }
        for d in &mapping.aggravates {
            let _ = writeln!(ttl, "ayurveda:{}  ayurveda:aggravatesDosha ayurveda:{} .", mapping.rasa, d);
            triple_count += 1;
        }
        ttl.push('\n');
    }

    // -- Rasa-guna associations --

    ttl.push_str("# ══════════════════════════════════════════════════\n");
    ttl.push_str("# Rasa-Guna Associations\n");
    ttl.push_str("# ══════════════════════════════════════════════════\n\n");

    for assoc in &extraction.rasa_guna_associations {
        let gunas: Vec<String> = assoc.guna.iter().map(|g| format!("ayurveda:{g}")).collect();
        let _ = writeln!(ttl, "ayurveda:{}  ayurveda:hasGuna  {} .", assoc.rasa, gunas.join(", "));
        triple_count += assoc.guna.len();
    }
    ttl.push('\n');

    // -- Rasa-guna rankings --

    ttl.push_str("# ══════════════════════════════════════════════════\n");
    ttl.push_str("# Rasa-Guna Rankings (foremost/middling/inferior)\n");
    ttl.push_str("# ══════════════════════════════════════════════════\n\n");

    for (guna, ranking) in &extraction.rasa_guna_rankings {
        let _ = writeln!(ttl, "ayurveda:{guna}  ayurveda:gunaRank [");
        let _ = writeln!(ttl, "    rdfs:label   \"foremost\" ;");
        let _ = writeln!(ttl, "    rdf:value    ayurveda:{}", ranking.foremost);
        let _ = writeln!(ttl, "] , [");
        let _ = writeln!(ttl, "    rdfs:label   \"middling\" ;");
        let _ = writeln!(ttl, "    rdf:value    ayurveda:{}", ranking.middling);
        let _ = writeln!(ttl, "] , [");
        let _ = writeln!(ttl, "    rdfs:label   \"inferior\" ;");
        let _ = writeln!(ttl, "    rdf:value    ayurveda:{}", ranking.inferior);
        let _ = writeln!(ttl, "] .\n");
        triple_count += 3;
    }

    // -- Vipaka rules --

    ttl.push_str("# ══════════════════════════════════════════════════\n");
    ttl.push_str("# Vipaka Rules\n");
    ttl.push_str("# ══════════════════════════════════════════════════\n\n");

    for rule in &extraction.vipaka_rules.rules {
        let id = format!("vipaka_rule_{}", rule.vipaka);
        let inputs: Vec<String> = rule.input_rasas.iter().map(|r| format!("ayurveda:{r}")).collect();
        let _ = writeln!(ttl, "ayurveda:{id}  a ayurveda:VipakaRule ;");
        let _ = writeln!(ttl, "    rdfs:label          \"{} vipaka rule\" ;", rule.vipaka);
        let _ = writeln!(ttl, "    ayurveda:inputRasa   {} ;", inputs.join(", "));
        let _ = writeln!(ttl, "    ayurveda:resultVipaka ayurveda:{} .", rule.vipaka);
        ttl.push('\n');
        triple_count += 3 + rule.input_rasas.len();
    }

    for effect in &extraction.vipaka_rules.vipaka_effects {
        let _ = writeln!(
            ttl,
            "ayurveda:{}  ayurveda:vipakaEffect  \"{}\" .",
            effect.vipaka,
            escape_ttl(&effect.effects),
        );
        triple_count += 1;
    }
    ttl.push('\n');

    // -- Veerya classification --

    ttl.push_str("# ══════════════════════════════════════════════════\n");
    ttl.push_str("# Veerya Classification\n");
    ttl.push_str("# ══════════════════════════════════════════════════\n\n");

    let two_types: Vec<String> = extraction.veerya_rules.two_fold.types.iter().map(|t| format!("ayurveda:{t}")).collect();
    let _ = writeln!(ttl, "ayurveda:veerya_twofold  rdfs:label \"two-fold veerya\" ;");
    let _ = writeln!(ttl, "    rdf:value  {} .\n", two_types.join(", "));
    triple_count += 2;

    let eight_types: Vec<String> = extraction.veerya_rules.eight_fold.types.iter().map(|t| format!("ayurveda:{t}")).collect();
    let _ = writeln!(ttl, "ayurveda:veerya_eightfold  rdfs:label \"eight-fold veerya\" ;");
    let _ = writeln!(ttl, "    rdf:value  {} .\n", eight_types.join(", "));
    triple_count += 2;

    // -- Hierarchy --

    ttl.push_str("# ══════════════════════════════════════════════════\n");
    ttl.push_str("# Hierarchy: prabhava > veerya > vipaka > rasa\n");
    ttl.push_str("# ══════════════════════════════════════════════════\n\n");

    for (i, concept) in extraction.hierarchy.order.iter().enumerate() {
        let _ = writeln!(
            ttl,
            "ayurveda:{concept}  ayurveda:hierarchyRank  \"{}\"^^xsd:integer .",
            i + 1,
        );
        triple_count += 1;
    }
    ttl.push('\n');

    // -- Prabhava definition --

    ttl.push_str("# ══════════════════════════════════════════════════\n");
    ttl.push_str("# Prabhava (specific potency) — definition and example\n");
    ttl.push_str("# ══════════════════════════════════════════════════\n\n");

    let ex = &extraction.prabhava_definition.example;
    let _ = writeln!(ttl, "ayurveda:prabhava_example  rdfs:label \"prabhava example\" ;");
    let _ = writeln!(ttl, "    rdfs:comment \"{}\" ;", escape_ttl(&format!(
        "{} and {} — {}. {} {}",
        ex.substance_1, ex.substance_2, ex.shared, ex.difference, ex.explanation
    )));
    let _ = writeln!(ttl, "    rdf:value    \"{}\" .", escape_ttl(&ex.explanation));
    ttl.push('\n');
    triple_count += 3;

    // -- Viruddha ahara (incompatible combinations) --

    ttl.push_str("# ══════════════════════════════════════════════════\n");
    ttl.push_str("# Viruddha Ahara (Incompatible Food Combinations)\n");
    ttl.push_str("# ══════════════════════════════════════════════════\n\n");

    for entry in &extraction.viruddha {
        let id = viruddha_id(&entry.combination);
        let _ = writeln!(ttl, "ayurveda:viruddha_{id}  a ayurveda:ViruddhaRule ;");
        let _ = writeln!(ttl, "    rdfs:label          \"{}\" ;", escape_ttl(&entry.combination));
        let _ = writeln!(ttl, "    ayurveda:combination \"{}\" ;", escape_ttl(&entry.combination));
        let _ = writeln!(ttl, "    ayurveda:effect      \"{}\" .", escape_ttl(&entry.effect));
        ttl.push('\n');
        triple_count += 4;
    }

    // -- Provenance --

    ttl.push_str("# ══════════════════════════════════════════════════\n");
    ttl.push_str("# RDF-star Provenance\n");
    ttl.push_str("# ══════════════════════════════════════════════════\n\n");

    // Rasa-dosha provenance
    for mapping in &extraction.rasa_dosha_mappings {
        for d in &mapping.pacifies {
            write_provenance(
                &mut ttl,
                &format!("ayurveda:{}", mapping.rasa),
                "ayurveda:pacifiesDosha",
                &format!("ayurveda:{d}"),
                &mapping.verses,
                mapping.confidence,
            );
        }
        for d in &mapping.aggravates {
            write_provenance(
                &mut ttl,
                &format!("ayurveda:{}", mapping.rasa),
                "ayurveda:aggravatesDosha",
                &format!("ayurveda:{d}"),
                &mapping.verses,
                mapping.confidence,
            );
        }
    }

    // Rasa-guna provenance
    for assoc in &extraction.rasa_guna_associations {
        for g in &assoc.guna {
            write_provenance(
                &mut ttl,
                &format!("ayurveda:{}", assoc.rasa),
                "ayurveda:hasGuna",
                &format!("ayurveda:{g}"),
                &assoc.verses,
                0.95,
            );
        }
    }

    // Vipaka rule provenance
    for rule in &extraction.vipaka_rules.rules {
        let id = format!("vipaka_rule_{}", rule.vipaka);
        write_provenance(
            &mut ttl,
            &format!("ayurveda:{id}"),
            "ayurveda:resultVipaka",
            &format!("ayurveda:{}", rule.vipaka),
            &rule.verses,
            0.95,
        );
    }

    // Vipaka effect provenance
    for effect in &extraction.vipaka_rules.vipaka_effects {
        write_provenance(
            &mut ttl,
            &format!("ayurveda:{}", effect.vipaka),
            "ayurveda:vipakaEffect",
            &format!("\"{}\"", escape_ttl(&effect.effects)),
            &effect.verses,
            0.90,
        );
    }

    // Viruddha provenance
    for entry in &extraction.viruddha {
        let id = viruddha_id(&entry.combination);
        write_provenance(
            &mut ttl,
            &format!("ayurveda:viruddha_{id}"),
            "ayurveda:combination",
            &format!("\"{}\"", escape_ttl(&entry.combination)),
            &entry.verses,
            entry.confidence,
        );
    }

    // Ranking provenance
    for (guna, ranking) in &extraction.rasa_guna_rankings {
        write_provenance(
            &mut ttl,
            &format!("ayurveda:{guna}"),
            "ayurveda:gunaRank",
            &format!("_:rank_{guna}_foremost"),
            &ranking.verses,
            0.90,
        );
    }

    // Hierarchy provenance
    for concept in &extraction.hierarchy.order {
        write_provenance(
            &mut ttl,
            &format!("ayurveda:{concept}"),
            "ayurveda:hierarchyRank",
            &format!("\"{}\"^^xsd:integer", extraction.hierarchy.order.iter().position(|c| c == concept).unwrap() + 1),
            &extraction.hierarchy.verses,
            0.95,
        );
    }

    fs::write(&output, &ttl)
        .with_context(|| format!("writing {}", output.display()))?;

    eprintln!(
        "Wrote ch.26 theory ({} triples with provenance, {} viruddha entries) to {}",
        triple_count,
        extraction.viruddha.len(),
        output.display(),
    );

    Ok(())
}
