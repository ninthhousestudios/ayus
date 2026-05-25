use std::path::Path;
use std::sync::Arc;

use vidya_core::{KnowledgeStore, ProvenanceFilter, ResolveContext, Result};

const SEED_TTL: &str = include_str!("../seeds/ayurveda.ttl");
const CH27_TTL: &str = include_str!("../seeds/charaka-ss-27.ttl");
const CH26_TTL: &str = include_str!("../seeds/charaka-ss-26.ttl");

pub struct AppData {
    pub store: KnowledgeStore,
    pub resolve_ctx: Arc<ResolveContext>,
    pub catalog: Vec<CategoryGroup>,
    pub domains: Vec<DomainInfo>,
    pub active_domain: String,
}

pub struct CategoryGroup {
    pub name: String,
    pub dravyas: Vec<String>,
}

#[derive(Clone)]
pub struct DomainInfo {
    pub name: String,
    pub source: String,
    pub triple_count: usize,
    pub entity_count: usize,
}

pub fn init() -> Result<AppData> {
    let store = KnowledgeStore::new_memory()?;

    store.load_domain("ayurveda", SEED_TTL)?;
    store.load_domain("ayurveda", CH27_TTL)?;
    store.load_domain("ayurveda", CH26_TTL)?;

    let resolve_ctx = store.resolve_context("ayurveda");
    let catalog = build_catalog(&store, "ayurveda")?;
    let domains = vec![build_domain_info(&store, "ayurveda", "embedded")];

    Ok(AppData {
        store,
        resolve_ctx,
        catalog,
        domains,
        active_domain: "ayurveda".to_string(),
    })
}

pub fn load_custom(data: &mut AppData, path: &Path) -> Result<DomainInfo> {
    let name = path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("custom");
    data.store.load_domain_from_file(name, path)?;
    data.resolve_ctx = data.store.resolve_context(&data.active_domain);
    let info = build_domain_info(&data.store, name, &path.display().to_string());
    data.domains.push(info.clone());
    Ok(info)
}

fn build_catalog(store: &KnowledgeStore, domain: &str) -> Result<Vec<CategoryGroup>> {
    let no_filter = ProvenanceFilter::default();
    let vargas = store.search(domain, "Varga", &[], &no_filter)?;

    let mut catalog = Vec::new();
    for varga in &vargas.entities {
        let dravyas = store.search(
            domain,
            "Dravya",
            &[("hasVarga".into(), varga.name.clone())],
            &no_filter,
        )?;
        catalog.push(CategoryGroup {
            name: varga.label.clone(),
            dravyas: dravyas.entities.iter().map(|d| d.label.clone()).collect(),
        });
    }

    catalog.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(catalog)
}

fn build_domain_info(store: &KnowledgeStore, name: &str, source: &str) -> DomainInfo {
    let entity_count = store
        .search(name, "Dravya", &[], &ProvenanceFilter::default())
        .map(|r| r.entities.len())
        .unwrap_or(0);
    DomainInfo {
        name: name.to_string(),
        source: source.to_string(),
        triple_count: store.triple_count().unwrap_or(0),
        entity_count,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_ttl_loads() {
        let data = init().unwrap();
        assert!(data.store.triple_count().unwrap() > 0);
        assert!(!data.domains.is_empty());
    }

    #[test]
    fn catalog_has_correct_categories() {
        let data = init().unwrap();
        assert_eq!(data.catalog.len(), 12);
        let shuka = data
            .catalog
            .iter()
            .find(|g| g.name == "shuka-dhanya")
            .unwrap();
        assert!(shuka.dravyas.len() >= 6);
    }

    #[test]
    fn all_83_dravyas_present() {
        let data = init().unwrap();
        let total: usize = data.catalog.iter().map(|g| g.dravyas.len()).sum();
        assert_eq!(total, 83);
    }

    #[test]
    fn dravya_queryable_by_name() {
        let data = init().unwrap();
        let result = data
            .store
            .search("ayurveda", "Dravya", &[], &ProvenanceFilter::default())
            .unwrap();
        let names: Vec<&str> = result.entities.iter().map(|e| e.label.as_str()).collect();
        assert!(names.contains(&"pippali"));
    }

    #[test]
    fn custom_ttl_creates_separate_domain() {
        let mut data = init().unwrap();
        let dir = std::env::temp_dir().join("ayus-test-custom-domain");
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test-herbs.ttl");
        std::fs::write(
            &path,
            "@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .\n\
             @prefix ay: <http://vidya.ninthhouse.studio/domain/test-herbs/> .\n\
             ay:TestHerb a ay:Dravya ; rdfs:label \"test-herb\" .\n",
        )
        .unwrap();
        let info = load_custom(&mut data, &path).unwrap();
        assert_eq!(info.name, "test-herbs");
        assert_eq!(data.domains.len(), 2);
        std::fs::remove_dir_all(&dir).ok();
    }

    #[test]
    fn invalid_ttl_path_returns_error() {
        let mut data = init().unwrap();
        let result = load_custom(&mut data, Path::new("/nonexistent.ttl"));
        assert!(result.is_err());
    }
}
