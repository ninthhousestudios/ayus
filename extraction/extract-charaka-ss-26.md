# Extraction Task: Charaka Samhita, Sutra Sthana, Chapter 26

You are extracting theoretical pharmacological framework data from the Kaviratna (1890-1908) English translation of the Charaka Samhita. This is a public domain OCR'd text.

## Your task

Read the source file at `sources/charaka-ss-26.txt` and extract the theoretical framework into `extraction/output/charaka-ss-26.json`.

Ch.26 (Atreya-Bhadrakapya) is the rasa theory chapter. It does NOT primarily list individual substances — it establishes the framework:
- The six rasas and their properties
- Rasa-to-dosha mappings (which tastes aggravate/pacify which doshas)
- The concept of veerya (potency) and its types
- The concept of vipaka (assimilation/post-digestive effect)
- The hierarchy: vipaka overrides rasa, veerya overrides vipaka, prabhava overrides all
- Incompatible food combinations (viruddha ahara)

## What to extract

### 1. Rasa-dosha theory triples

For each of the six rasas, extract which doshas it pacifies and aggravates, with the verse number. These should match the existing theory triples in `seeds/ayurveda.ttl` but now with real citations.

Output format:
```json
{
  "rasa_dosha_mappings": [
    {
      "rasa": "madhura",
      "pacifies": ["vata", "pitta"],
      "aggravates": ["kapha"],
      "verses": [28],
      "source_excerpt": "...",
      "confidence": 0.95
    }
  ]
}
```

### 2. Rasa-guna associations

What gunas (qualities) does each rasa have? E.g., "sweet is heavy, oily, cooling."

```json
{
  "rasa_guna_associations": [
    {
      "rasa": "madhura",
      "guna": ["guru", "snigdha", "sheeta"],
      "verses": [42],
      "source_excerpt": "..."
    }
  ]
}
```

### 3. Vipaka rules

The three vipakas and which rasas produce them.

### 4. Incompatible combinations (viruddha ahara)

List of food combinations declared incompatible, with verse citations. These are specific pairs/combinations, not general principles.

```json
{
  "viruddha": [
    {
      "combination": "honey and ghee in equal portions",
      "effect": "poisonous",
      "verses": [19],
      "source_excerpt": "..."
    }
  ]
}
```

## Mapping conventions

Same as in `extract-charaka-ss-27.md` — refer to that file for the full English-to-Sanskrit mapping tables.

## Process

1. Read the full chapter text
2. Extract the four categories above
3. Write the complete JSON output
4. Report what was found and any ambiguities

This chapter is more about establishing the theoretical framework than listing substances. The key deliverable is real verse-level citations for the rasa-dosha theory that the existing seed declares without citation.
