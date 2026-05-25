use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Deserialize)]
struct Extraction {
    source: Source,
    dravyas: Vec<Dravya>,
}

#[derive(Deserialize)]
struct Source {
    text: String,
    sthana: String,
    chapter: u32,
}

#[derive(Deserialize)]
struct Dravya {
    id: String,
    sanskrit_name: String,
    botanical_name: Option<String>,
    common_name: Option<String>,
    category: String,
    rasa: Vec<String>,
    guna: Vec<String>,
    veerya: Option<String>,
    vipaka: Option<String>,
    karma: Vec<String>,
    dosha_effects: DoshaEffects,
    verses: Vec<u32>,
    confidence: f64,
    #[allow(dead_code)]
    notes: Option<String>,
}

#[derive(Deserialize)]
struct DoshaEffects {
    pacifies: Vec<String>,
    aggravates: Vec<String>,
}

fn source_iri(text: &str) -> &str {
    match text {
        "Charaka Samhita" => "ayurveda:source-charaka",
        "Sushruta Samhita" => "ayurveda:source-sushruta",
        "Bhavaprakasha Nighantu" => "ayurveda:source-bhavaprakasha",
        _ => "ayurveda:source-unknown",
    }
}

fn ttl_id(id: &str) -> String {
    id.replace('-', "_")
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

fn write_new_karma_entities(out: &mut String, dravyas: &[Dravya]) {
    let known: BTreeSet<&str> = [
        "deepana", "pachana", "rasayana", "anulomana", "medhya", "balya",
        "shothahara", "krimighna", "jwaraghna", "vajikarana", "virechana",
        "medoghna", "vatanulomana",
    ]
    .into_iter()
    .collect();

    let mut new_karmas = BTreeSet::new();
    for d in dravyas {
        for k in &d.karma {
            if !known.contains(k.as_str()) {
                new_karmas.insert(k.as_str());
            }
        }
    }

    if new_karmas.is_empty() {
        return;
    }

    let common_names: std::collections::HashMap<&str, &str> = [
        ("brimhana", "nourishing"),
        ("grahi", "absorbent"),
        ("mutrala", "diuretic"),
        ("swedana", "diaphoretic"),
        ("vranaropana", "wound-healing"),
        ("stambhana", "astringent-action"),
        ("vishahara", "anti-poison"),
        ("hridya", "cardiotonic"),
        ("shukrala", "spermatogenic"),
        ("karshana", "emaciating"),
        ("vamana", "emetic"),
    ]
    .into_iter()
    .collect();

    out.push_str("# New karma entities discovered in extraction\n");
    for k in &new_karmas {
        let cn = common_names.get(k).unwrap_or(k);
        let _ = writeln!(
            out,
            "ayurveda:{id}  a ayurveda:Karma ; rdfs:label \"{id}\" ; ayurveda:commonName \"{cn}\" .",
            id = k,
            cn = cn,
        );
    }
    out.push('\n');
}

fn varga_id(category: &str) -> String {
    format!("varga_{}", category.replace('-', "_"))
}

fn write_varga_entities(out: &mut String, dravyas: &[Dravya]) {
    let vargas: BTreeSet<&str> = dravyas.iter().map(|d| d.category.as_str()).collect();
    out.push_str("# Varga (category) entities\n");
    for cat in &vargas {
        let id = varga_id(cat);
        let _ = writeln!(out, "ayurveda:{id}  a ayurveda:Varga ; rdfs:label \"{cat}\" .");
    }
    out.push('\n');
}

fn write_dravya(out: &mut String, d: &Dravya) {
    let id = ttl_id(&d.id);
    let vid = varga_id(&d.category);
    let _ = writeln!(out, "# -- {} ({}) --", d.sanskrit_name, d.category);
    let _ = writeln!(out, "ayurveda:{id}  a ayurveda:Dravya ;");
    let _ = writeln!(out, "    rdfs:label              \"{}\" ;", d.id);
    let _ = writeln!(out, "    ayurveda:hasVarga       ayurveda:{vid} ;");

    if let Some(cn) = &d.common_name {
        let _ = writeln!(out, "    ayurveda:commonName     \"{}\" ;", escape_ttl(cn));
    }
    if let Some(bn) = &d.botanical_name {
        let _ = writeln!(out, "    ayurveda:botanicalName  \"{}\" ;", escape_ttl(bn));
    }

    if !d.rasa.is_empty() {
        let vals: Vec<String> = d.rasa.iter().map(|r| format!("ayurveda:{r}")).collect();
        let _ = writeln!(out, "    ayurveda:hasRasa        {} ;", vals.join(", "));
    }
    if !d.guna.is_empty() {
        let vals: Vec<String> = d.guna.iter().map(|g| format!("ayurveda:{g}")).collect();
        let _ = writeln!(out, "    ayurveda:hasGuna        {} ;", vals.join(", "));
    }
    if let Some(v) = &d.veerya {
        let _ = writeln!(out, "    ayurveda:hasVeerya      ayurveda:{v} ;");
    }
    if let Some(v) = &d.vipaka {
        let _ = writeln!(out, "    ayurveda:hasVipaka      ayurveda:{v} ;");
    }
    if !d.karma.is_empty() {
        let vals: Vec<String> = d.karma.iter().map(|k| format!("ayurveda:{k}")).collect();
        let _ = writeln!(out, "    ayurveda:hasKarma       {} ;", vals.join(", "));
    }
    if !d.dosha_effects.pacifies.is_empty() {
        let vals: Vec<String> = d.dosha_effects.pacifies.iter().map(|x| format!("ayurveda:{x}")).collect();
        let _ = writeln!(out, "    ayurveda:pacifiesDosha  {} ;", vals.join(", "));
    }
    if !d.dosha_effects.aggravates.is_empty() {
        let vals: Vec<String> = d.dosha_effects.aggravates.iter().map(|x| format!("ayurveda:{x}")).collect();
        let _ = writeln!(out, "    ayurveda:aggravatesDosha {} ;", vals.join(", "));
    }

    // Remove trailing " ;\n" and replace with " .\n"
    if out.ends_with(" ;\n") {
        out.truncate(out.len() - 2);
        out.push_str(".\n");
    }
    out.push('\n');
}

fn write_provenance(out: &mut String, d: &Dravya, src: &Source) {
    let id = ttl_id(&d.id);
    let source_iri = source_iri(&src.text);

    struct Triple<'a> {
        pred: &'a str,
        obj: String,
    }

    let mut triples: Vec<Triple> = Vec::new();

    for r in &d.rasa {
        triples.push(Triple { pred: "ayurveda:hasRasa", obj: format!("ayurveda:{r}") });
    }
    for g in &d.guna {
        triples.push(Triple { pred: "ayurveda:hasGuna", obj: format!("ayurveda:{g}") });
    }
    if let Some(v) = &d.veerya {
        triples.push(Triple { pred: "ayurveda:hasVeerya", obj: format!("ayurveda:{v}") });
    }
    if let Some(v) = &d.vipaka {
        triples.push(Triple { pred: "ayurveda:hasVipaka", obj: format!("ayurveda:{v}") });
    }
    for k in &d.karma {
        triples.push(Triple { pred: "ayurveda:hasKarma", obj: format!("ayurveda:{k}") });
    }
    for p in &d.dosha_effects.pacifies {
        triples.push(Triple { pred: "ayurveda:pacifiesDosha", obj: format!("ayurveda:{p}") });
    }
    for a in &d.dosha_effects.aggravates {
        triples.push(Triple { pred: "ayurveda:aggravatesDosha", obj: format!("ayurveda:{a}") });
    }

    let verse_str = if d.verses.len() == 1 {
        format!("\"{}\"^^xsd:integer", d.verses[0])
    } else {
        format!("\"{}\"^^xsd:integer", d.verses[0])
    };

    for t in &triples {
        let _ = writeln!(out, "<< ayurveda:{id} {} {} >>", t.pred, t.obj);
        let _ = writeln!(out, "    vidya:assertedBy [");
        let _ = writeln!(out, "        vidya:tradition  ayurveda:tradition-atreya ;");
        let _ = writeln!(out, "        vidya:source     {} ;", source_iri);
        let _ = writeln!(out, "        vidya:sthana     \"{}\" ;", src.sthana);
        let _ = writeln!(out, "        vidya:chapter    \"{}\"^^xsd:integer ;", src.chapter);
        let _ = writeln!(out, "        vidya:verse      {} ;", verse_str);
        let _ = writeln!(out, "        vidya:pramana    vidya:shabda ;");
        let _ = writeln!(out, "        vidya:confidence \"{}\"^^xsd:float", d.confidence);
        let _ = writeln!(out, "    ] .\n");
    }
}

fn escape_ttl(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: json2ttl <input.json> [output.ttl]");
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
    let extraction: Extraction = serde_json::from_str(&json_str)
        .with_context(|| "parsing JSON")?;

    let mut ttl = String::with_capacity(64 * 1024);

    let _ = writeln!(
        ttl,
        "# Extracted from {}, {}, Ch.{} (Kaviratna translation, public domain).\n\
         # Generated by json2ttl from extraction JSON. {} dravyas.\n",
        extraction.source.text,
        extraction.source.sthana,
        extraction.source.chapter,
        extraction.dravyas.len(),
    );

    write_prefixes(&mut ttl);
    write_new_karma_entities(&mut ttl, &extraction.dravyas);
    write_varga_entities(&mut ttl, &extraction.dravyas);

    ttl.push_str("# ══════════════════════════════════════════════════\n");
    ttl.push_str("# Dravya Entities\n");
    ttl.push_str("# ══════════════════════════════════════════════════\n\n");

    for d in &extraction.dravyas {
        write_dravya(&mut ttl, d);
    }

    ttl.push_str("# ══════════════════════════════════════════════════\n");
    ttl.push_str("# RDF-star Provenance\n");
    ttl.push_str("# ══════════════════════════════════════════════════\n\n");

    for d in &extraction.dravyas {
        write_provenance(&mut ttl, d, &extraction.source);
    }

    fs::write(&output, &ttl)
        .with_context(|| format!("writing {}", output.display()))?;

    eprintln!(
        "Wrote {} dravyas ({} triples with provenance) to {}",
        extraction.dravyas.len(),
        extraction.dravyas.iter().map(|d| {
            d.rasa.len() + d.guna.len()
                + d.veerya.as_ref().map_or(0, |_| 1)
                + d.vipaka.as_ref().map_or(0, |_| 1)
                + d.karma.len()
                + d.dosha_effects.pacifies.len()
                + d.dosha_effects.aggravates.len()
        }).sum::<usize>(),
        output.display(),
    );

    Ok(())
}
