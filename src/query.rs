use vidya_core::resolve::assemble;
use vidya_core::resolve::matcher;
use vidya_core::resolve::{QueryMode, ResolvedQuery, ResolvedToken, ResolutionReport};
use vidya_core::{KnowledgeStore, ProvenanceFilter, ResolveContext};
use vidya_core::{DescribeResult, ProvenanceResult, SearchResult, TraverseResult};

#[derive(Debug)]
pub enum QueryOutcome {
    Describe {
        result: DescribeResult,
        report: ResolutionReport,
    },
    Search {
        result: SearchResult,
        report: ResolutionReport,
    },
    Traverse {
        result: TraverseResult,
        report: ResolutionReport,
    },
    Provenance {
        result: ProvenanceResult,
        report: ResolutionReport,
    },
    NoMatch {
        unknown_tokens: Vec<String>,
        error: String,
    },
}

pub struct Preset {
    pub label: &'static str,
    pub input: &'static str,
}

pub const PRESETS: &[Preset] = &[
    Preset {
        label: "Pippali",
        input: "pippali",
    },
    Preset {
        label: "Katu rasa dravyas",
        input: "dravya katu",
    },
    Preset {
        label: "Pacifies vata",
        input: "dravya vata",
    },
    Preset {
        label: "Pippali karma",
        input: "pippali haskarma",
    },
    Preset {
        label: "Haritaki",
        input: "haritaki",
    },
];

pub fn execute(
    input: &str,
    store: &KnowledgeStore,
    resolve_ctx: &ResolveContext,
    domain: &str,
) -> QueryOutcome {
    let tokens = matcher::tokenize(input);
    let matched = matcher::match_tokens(&tokens, &resolve_ctx.vocab, Some(&resolve_ctx.vsa), domain);

    if matched.is_empty() {
        return QueryOutcome::NoMatch {
            unknown_tokens: tokens,
            error: "no tokens to resolve".into(),
        };
    }

    let mode = infer_mode(&matched);

    for m in fallback_order(mode) {
        match assemble::assemble(m, &matched) {
            Ok(report) => match dispatch(report, store, domain, &matched) {
                Ok(outcome) => return outcome,
                Err(_) => continue,
            },
            Err(_) => continue,
        }
    }

    let unknowns: Vec<String> = matched
        .iter()
        .filter_map(|t| match t {
            ResolvedToken::Unknown(s) => Some(s.clone()),
            _ => None,
        })
        .collect();

    QueryOutcome::NoMatch {
        unknown_tokens: unknowns,
        error: "could not resolve query in any mode".into(),
    }
}

fn infer_mode(tokens: &[ResolvedToken]) -> QueryMode {
    let has_type = tokens
        .iter()
        .any(|t| matches!(t, ResolvedToken::Type { .. }));
    let has_entity = tokens
        .iter()
        .any(|t| matches!(t, ResolvedToken::Entity { .. }));
    let has_predicate = tokens
        .iter()
        .any(|t| matches!(t, ResolvedToken::Predicate { .. }));
    let has_prop_value = tokens
        .iter()
        .any(|t| matches!(t, ResolvedToken::PropertyValue { .. }));

    if has_type && (has_entity || has_prop_value) {
        QueryMode::Search
    } else if has_prop_value {
        QueryMode::Search
    } else if has_entity && has_predicate {
        QueryMode::Traverse
    } else if has_type && !has_entity {
        QueryMode::Search
    } else {
        QueryMode::Describe
    }
}

fn fallback_order(primary: QueryMode) -> Vec<QueryMode> {
    let all = [
        QueryMode::Describe,
        QueryMode::Search,
        QueryMode::Traverse,
        QueryMode::Provenance,
    ];
    let mut order = vec![primary];
    for m in all {
        if !matches!(
            (&primary, &m),
            (QueryMode::Describe, QueryMode::Describe)
                | (QueryMode::Search, QueryMode::Search)
                | (QueryMode::Traverse, QueryMode::Traverse)
                | (QueryMode::Provenance, QueryMode::Provenance)
        ) {
            order.push(m);
        }
    }
    order
}

fn dispatch(
    report: ResolutionReport,
    store: &KnowledgeStore,
    domain: &str,
    matched: &[ResolvedToken],
) -> vidya_core::Result<QueryOutcome> {
    let no_filter = ProvenanceFilter::default();
    match &report.query {
        ResolvedQuery::Describe { subject_iri } => {
            let local = local_name(subject_iri);
            let result = store.describe(domain, &local, &no_filter)?;
            Ok(QueryOutcome::Describe { result, report })
        }
        ResolvedQuery::Search { type_iri, filters } => {
            let kind = local_name(type_iri);
            let effective_filters = if filters.is_empty() {
                entity_to_filters(matched, store, domain)
            } else {
                filters.clone()
            };
            let result = store.search(domain, &kind, &effective_filters, &no_filter)?;
            Ok(QueryOutcome::Search { result, report })
        }
        ResolvedQuery::Traverse {
            subject_iri,
            predicate_iri,
        } => {
            let subject = local_name(subject_iri);
            let predicate = local_name(predicate_iri);
            let result = store.traverse(domain, &subject, &predicate, 3, &no_filter)?;
            Ok(QueryOutcome::Traverse { result, report })
        }
        ResolvedQuery::Provenance {
            subject_iri,
            predicate_iri,
            object,
            ..
        } => {
            let subject = local_name(subject_iri);
            let predicate = local_name(predicate_iri);
            let result = store.provenance(domain, &subject, &predicate, object, &no_filter)?;
            Ok(QueryOutcome::Provenance { result, report })
        }
    }
}

fn local_name(iri: &str) -> String {
    iri.rsplit_once('/')
        .map(|(_, l)| l)
        .unwrap_or(iri)
        .to_string()
}

fn entity_to_filters(
    matched: &[ResolvedToken],
    store: &KnowledgeStore,
    domain: &str,
) -> Vec<(String, String)> {
    let no_filter = ProvenanceFilter::default();
    matched
        .iter()
        .filter_map(|t| {
            let iri = match t {
                ResolvedToken::Entity { iri, .. } => iri,
                _ => return None,
            };
            let local = local_name(iri);
            let desc = store.describe(domain, &local, &no_filter).ok()?;
            for ty in &desc.types {
                let predicate = match ty.as_str() {
                    "Rasa" => "hasRasa",
                    "Guna" => "hasGuna",
                    "Dosha" => "pacifiesDosha",
                    "Karma" => "hasKarma",
                    "Varga" => "hasVarga",
                    _ => continue,
                };
                return Some((predicate.to_string(), local.clone()));
            }
            None
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data;

    fn setup() -> data::AppData {
        data::init().unwrap()
    }

    #[test]
    fn describe_pippali() {
        let data = setup();
        let outcome = execute("pippali", &data.store, &data.resolve_ctx, &data.active_domain);
        match outcome {
            QueryOutcome::Describe { result, .. } => {
                assert_eq!(result.label.as_deref(), Some("pippali"));
                let preds: Vec<&str> = result.properties.iter().map(|p| p.predicate.as_str()).collect();
                assert!(preds.iter().any(|p| p.contains("hasRasa")));
                assert!(preds.iter().any(|p| p.contains("hasGuna")));
            }
            other => panic!("expected Describe, got {other:?}"),
        }
    }

    #[test]
    fn search_dravya_katu() {
        let data = setup();
        let outcome = execute("dravya katu", &data.store, &data.resolve_ctx, &data.active_domain);
        match outcome {
            QueryOutcome::Search { result, .. } => {
                let names: Vec<&str> = result.entities.iter().map(|e| e.label.as_str()).collect();
                assert!(names.contains(&"pippali"), "expected pippali in {names:?}");
            }
            other => panic!("expected Search, got {other:?}"),
        }
    }

    #[test]
    fn unknown_input_no_match() {
        let data = setup();
        let outcome = execute("xyzzyplugh", &data.store, &data.resolve_ctx, &data.active_domain);
        match outcome {
            QueryOutcome::NoMatch { unknown_tokens, .. } => {
                assert!(unknown_tokens.contains(&"xyzzyplugh".to_string()));
            }
            other => panic!("expected NoMatch, got {other:?}"),
        }
    }

    #[test]
    fn presets_all_resolve() {
        let data = setup();
        for preset in PRESETS {
            let outcome = execute(preset.input, &data.store, &data.resolve_ctx, &data.active_domain);
            assert!(
                !matches!(outcome, QueryOutcome::NoMatch { .. }),
                "preset '{}' (input: '{}') returned NoMatch",
                preset.label,
                preset.input,
            );
        }
    }
}
