# Extraction Task: Charaka Samhita, Sutra Sthana, Chapter 27

You are extracting structured pharmacological data from the Kaviratna (1890-1908) English translation of the Charaka Samhita. This is a public domain OCR'd text.

## Your task

Read the source file at `sources/charaka-ss-27.txt` and extract every identifiable substance (dravya) into the JSON schema defined in `extraction/schema.json`. Write the output to `extraction/output/charaka-ss-27.json`.

## How to read the source text

The text is OCR from a 19th-century printed book. Expect:
- Extra spaces between characters (e.g., "C H A R A K A" for "CHARAKA")
- Ligature artifacts: `¬` at line breaks means a hyphenated word
- Page numbers appear as bare numbers on their own line (e.g., "333")
- Headers like "CHARAKA-SAMHITA." or "CHA RA KA-SA MHITA." are page headers — skip them
- Footnotes are marked with `*`, `†`, `‡` and appear after the main text of a verse
- Verse numbers appear as superscript in the original, rendered as bare numbers after punctuation (e.g., "...and heavy.23" means verse 23)
- Translator notes in parentheses marked with "— T." are Kaviratna's commentary

## Substance identification

Ch.27 (Annapanavidhi — "Ordinances on Food and Drink") organizes substances into groups:
1. **Shuka-dhanya** (grains/cereals): Shali rice varieties, Shastika, Yava (barley), Godhuma (wheat)
2. **Shami-dhanya** (pulses): Mudga, Masha, Rajamasha, Chanaka, etc.
3. **Mamsa varga** (meats): grouped by habitat — Prasaha, Bhumishaya, Anupa, Audaka, etc.
4. **Shaka varga** (vegetables/greens)
5. **Phala varga** (fruits)
6. **Harita varga** (salad herbs/greens)
7. **Madya varga** (wines/alcoholic preparations)
8. **Jala varga** (waters)
9. **Gorasa varga** (milk and dairy): Kshira (milk), Dadhi (curds), Takra (buttermilk), Ghrita (ghee)
10. **Ikshu varga** (sugarcane products)
11. **Kritanna varga** (prepared foods)
12. **Ahara varga** (miscellaneous dietary substances)

Extract each distinct substance that has at least one pharmacological property stated (rasa, guna, veerya, vipaka, karma, or dosha effect).

## Property mapping

The Kaviratna translation uses English terms. Map them to Sanskrit:

### Rasa (taste)
| English | Sanskrit |
|---------|----------|
| sweet | madhura |
| sour | amla |
| salty, saltish, saline | lavana |
| pungent | katu |
| bitter | tikta |
| astringent | kashaya |

### Guna (quality)
| English | Sanskrit |
|---------|----------|
| heavy | guru |
| light | laghu |
| cold, cooling | sheeta |
| hot, warm, heating | ushna |
| oily, unctuous | snigdha |
| dry | ruksha |
| sharp, penetrating | tikshna |
| slow, dull | manda |
| stable, static | sthira |
| mobile, flowing | sara |
| soft | mridu |
| hard | kathina |
| clear, non-slimy | vishada |
| slimy, sticky | picchila |
| smooth | shlakshna |
| rough | khara |
| subtle, minute | sukshma |
| gross, bulky | sthula |
| dense, solid | sandra |
| liquid | drava |

### Veerya (potency)
Only two values: `sheeta` (cooling/cold) or `ushna` (warm/hot).
Kaviratna writes "cooling in potency" or just "cooling" when describing the overall nature.

### Vipaka (post-digestive effect / assimilation)
Kaviratna calls this "assimilation." Only three values: `madhura`, `amla`, `katu`.
Look for phrases like "sweet on assimilation", "in assimilation it is pungent", "sour on assimilation."

### Dosha effects
| English pattern | Meaning |
|-----------------|---------|
| "destructive of wind/vata" | pacifies vata |
| "enhances/increases/provocative of wind" | aggravates vata |
| "destructive of bile/pitta" | pacifies pitta |
| "provocative of bile" | aggravates pitta |
| "destructive of phlegm/kapha" | pacifies kapha |
| "provocative of phlegm" | aggravates kapha |
| "destructive of all the faults" | pacifies vata, pitta, kapha |
| "enhances all the faults" | aggravates vata, pitta, kapha |

Note: Kaviratna uses "wind" for vata, "bile" for pitta, "phlegm" for kapha.

### Karma (therapeutic actions)
Map from English descriptions to Sanskrit terms:

| English | Sanskrit |
|---------|----------|
| operates as a Rasayana / prolongs life | rasayana |
| appetizer / creates relish for food | deepana |
| digestive / assists at digestion | pachana |
| carminative / alleviates flatulence | anulomana |
| brain tonic / promotes intelligence | medhya |
| strength-giving / invigorating | balya |
| aphrodisiac / increases semen | vajikarana |
| purgative / purging | virechana |
| emetic / induces vomiting | vamana |
| reduces fat / destructive of fat | medoghna |
| anti-inflammatory / reduces swelling | shothahara |
| anthelmintic / destructive of worms | krimighna |
| antipyretic / alleviative of fever | jwaraghna |
| diuretic / increases urine | mutrala |
| wound-healing / unites fractures | vranaropana |
| nourishing / promotes nutrition | brimhana |
| reduces corpulency / emaciating | karshana |

Add new karma terms as needed — use the standard Sanskrit term, document it in the notes field.

## Confidence scoring

- **0.9-1.0**: All extracted properties are explicitly stated in the text. Clear verse boundary.
- **0.7-0.89**: Most properties explicit, but some inferred. E.g., dosha effect inferred from stated rasa using standard theory, or OCR slightly damaged but readable.
- **0.5-0.69**: Significant inference needed, or OCR is damaged enough to create ambiguity. Substance may be mentioned briefly with few properties.
- **Below 0.5**: Do not include. Too uncertain.

## Distinguishing veerya from guna

The text often says things like "cooling" or "warm" without specifying whether this is guna or veerya. Rules:
- If the text says "cooling in potency" or "hot in potency" → that's veerya
- If "cooling" appears alongside other gunas (heavy, light, dry, oily) → it's guna AND likely veerya too
- When in doubt, record it as guna. Only set veerya when the text is clear about potency, or when the cooling/heating nature is the dominant characterization.

## Output format

Write a single JSON file following the schema. Include the `source_excerpt` for each dravya — copy the relevant passage verbatim (you may clean up OCR spacing artifacts like extra spaces and `¬` line-break hyphens, but preserve the original wording).

## Worked example

For this passage:
> Yava (Hordeum kexastichum, Linn.) or barley, is dry, cooling, heavy, agreeable, and enhances the wind and the faeces. It makes the body consistent and hard, is of astringent taste, invigorating, and destructive of all affections due to phlegm.21

Extract:
```json
{
  "id": "yava",
  "sanskrit_name": "Yava",
  "botanical_name": "Hordeum hexastichum",
  "common_name": "barley",
  "category": "shuka-dhanya",
  "rasa": ["kashaya"],
  "guna": ["ruksha", "sheeta", "guru"],
  "veerya": "sheeta",
  "vipaka": null,
  "karma": ["balya"],
  "dosha_effects": {
    "pacifies": ["kapha"],
    "aggravates": ["vata"]
  },
  "verses": [21],
  "source_excerpt": "Yava (Hordeum hexastichum, Linn.) or barley, is dry, cooling, heavy, agreeable, and enhances the wind and the faeces. It makes the body consistent and hard, is of astringent taste, invigorating, and destructive of all affections due to phlegm.",
  "confidence": 0.9,
  "notes": "OCR has 'kexastichum' — corrected to 'hexastichum'. Vipaka not stated. 'Enhances wind' = aggravates vata. 'Destructive of phlegm' = pacifies kapha."
}
```

## Process

1. Read the full chapter text
2. Work through it sequentially, group by group
3. For each identifiable substance, extract all stated properties
4. Write the complete JSON output
5. At the end, report: total dravyas extracted, any substances skipped and why, any systematic issues encountered

Do NOT extract:
- Groups where only the group name is given with no individual substance properties
- Prepared dishes or recipes (these are combinations, not single dravyas)
- Substances mentioned only in footnotes/translator commentary with no properties from the original text
