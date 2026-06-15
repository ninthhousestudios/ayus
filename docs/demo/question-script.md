# Demo question script

9 questions across 4 beats. Each question is framed as "what does Charaka say" — textual scholarship, not health advice. All expected tool calls have been dry-run verified against the loaded vidya ayurveda store (2026-06-15).

---

## Beat 1: Grounded retrieval with receipts

These questions test whether the agent retrieves structured facts with verse-level citations, versus a bare LLM that may get the gist right but can't cite sources.

### Q1. Which of the six rasas pacify Vata according to Charaka?

**Expected tool calls:**
```
vidya_query(mode="describe", domain="ayurveda", subject="madhura")
vidya_query(mode="describe", domain="ayurveda", subject="amla")
vidya_query(mode="describe", domain="ayurveda", subject="lavana")
vidya_query(mode="describe", domain="ayurveda", subject="katu")
vidya_query(mode="describe", domain="ayurveda", subject="tikta")
vidya_query(mode="describe", domain="ayurveda", subject="kashaya")
```

**Expected agent answer:**
Three rasas pacify Vata:
- **Madhura** (sweet) — pacifiesDosha: vata, pitta. *Charaka Samhita, Sutra Sthana, Ch. 26, v. 57*
- **Amla** (sour) — pacifiesDosha: vata. *Charaka Samhita, Sutra Sthana, Ch. 26, v. 59*
- **Lavana** (salty) — pacifiesDosha: vata. *Charaka Samhita, Sutra Sthana, Ch. 26, v. 61*

The remaining three aggravate Vata:
- Katu (pungent) — *Ch. 26, v. 63*
- Tikta (bitter) — *Ch. 26, v. 65*
- Kashaya (astringent) — *Ch. 26, v. 67*

**Expected bare-LLM answer:**
Correctly names madhura, amla, lavana as vata-pacifying (this is widely taught). But cites no specific verses — may say "according to Ayurvedic texts" or "traditionally." No way for a reader to verify the claim against the source.

**Key citations:** SS 26.57, 26.59, 26.61, 26.63, 26.65, 26.67

---

### Q2. According to Charaka, what are the three vipaka categories and which rasas map to each?

**Expected tool calls:**
```
vidya_query(mode="search", domain="ayurveda", kind="VipakaRule")
vidya_query(mode="describe", domain="ayurveda", subject="vipaka_rule_madhura")
vidya_query(mode="describe", domain="ayurveda", subject="vipaka_rule_amla")
vidya_query(mode="describe", domain="ayurveda", subject="vipaka_rule_katu")
```

**Expected agent answer:**
Charaka describes three vipaka (post-digestive effect) categories:
1. **Madhura vipaka** ← madhura (sweet) and lavana (salty) rasas. *SS Ch. 26, v. 14*
2. **Amla vipaka** ← amla (sour) rasa. *SS Ch. 26, v. 14*
3. **Katu vipaka** ← katu (pungent), tikta (bitter), and kashaya (astringent) rasas. *SS Ch. 26, v. 14*

**Expected bare-LLM answer:**
Gets the three categories right (common teaching). May correctly map rasas. Cannot cite verse 14 specifically — likely says "Charaka Samhita" without location. May conflate the Charaka three-vipaka system with the Sushruta two-vipaka system without noting the distinction.

**Key citations:** SS 26.14 (all three rules)

---

## Beat 2: Viruddha — obscure lookup, positive match, and anupalabdhi

These test whether the agent can find specific food-incompatibility rules in a corpus of 26, and critically, distinguish "no rule found" from "safe to eat."

### Q3. Does Charaka record any rule about consuming radish or garlic with milk?

**Expected tool calls:**
```
vidya_query(mode="search", domain="ayurveda", kind="ViruddhaRule")
# Agent scans labels, finds match on "Mulaka (radish), Laguna (garlic)..."
vidya_query(mode="describe", domain="ayurveda",
  subject="viruddha_mulaka_radish_laguna_garlic_krishnagandha_moringa_pterygosperma_arjjaka_sumukha_or_surasa_varieties_of_ocimum_with_milk")
```

**Expected agent answer:**
Yes. Charaka lists this as a viruddha (incompatible) combination:

> "Mulaka (radish), Laguna (garlic), Krishnagandha (Moringa pterygosperma), Arjjaka, Sumukha, or Surasa (varieties of Ocimum) with milk"

Effect: **leprosy** (kushtha).

*Charaka Samhita, Sutra Sthana, Ch. 26, v. 9*

**Expected bare-LLM answer:**
This is where bare LLM waffles. It may say "some Ayurvedic traditions caution against combining radish with milk" but cannot cite the specific rule. Likely hedges: "this is debated" or "modern Ayurveda practitioners recommend..." May confuse it with the general "sour foods with milk" rule or the fish-with-milk rule. Cannot name the exact combination or its stated effect.

**Key citations:** SS 26.9

---

### Q4. Does Charaka warn against combining turmeric (haridra) with milk?

**Expected tool calls:**
```
vidya_query(mode="search", domain="ayurveda", kind="ViruddhaRule")
# Agent scans all 26 labels — no match for haridra/turmeric
```

**Expected agent answer:**
No viruddha rule is recorded for turmeric (haridra) with milk in the current dataset. The 26 viruddha rules in Sutra Sthana Ch. 26 do not mention this combination.

Note: absence of a recorded rule is not a safety claim. The dataset covers the specific combinations Charaka enumerates, not an exhaustive survey of all possible food interactions. (*Anupalabdhi* — non-apprehension as a valid epistemic category.)

**Expected bare-LLM answer:**
Bare LLM confidently says turmeric with milk is *beneficial* — "golden milk / haldi doodh is a traditional remedy." This is the opposite failure mode: it answers from modern popular knowledge rather than checking the classical source. The interesting thing is that both the agent and bare LLM agree there's no warning, but the agent explicitly grounds its negative finding in the dataset scope, while the bare LLM asserts a positive claim without textual basis.

**Key citations:** None (negative result). Agent should reference the scope: SS Ch. 26 viruddha enumeration.

---

### Q5. What combinations involving honey does Charaka describe as viruddha?

**Expected tool calls:**
```
vidya_query(mode="search", domain="ayurveda", kind="ViruddhaRule")
# Agent filters for honey-related rules from the 26 results
# Describes each matching rule for provenance
```

**Expected agent answer (at least these matches from the 26 rules):**
1. **Honey and ghee in equal portions** — *SS Ch. 26* (viruddha_honey_and_ghee_in_equal_portions)
2. **Honey and rain water in equal measures** — *SS Ch. 26* (viruddha_honey_and_rain_water_in_equal_measures)
3. **Honey taken in a heated state, or by a person afflicted with heat** — *SS Ch. 26* (viruddha_honey_taken_in_a_heated_state...)
4. **Honey and Pushkara seeds (lotus seeds)** — *SS Ch. 26* (viruddha_honey_and_pushkara_seeds...)
5. **Warm water drunk after taking honey** — *SS Ch. 26* (viruddha_warm_water_drunk_after_taking_honey)

Plus several compound rules where honey is one ingredient among others (meat with honey, pigeon fried in oil with honey and milk, etc.).

**Expected bare-LLM answer:**
Knows the famous "honey + ghee" and "heated honey" rules from popular Ayurveda discourse. Likely misses the rain-water rule, pushkara seeds rule, and the compound meat rules. Cannot cite chapter/verse. May add modern interpretations ("honey becomes toxic when heated due to HMF formation") that aren't in Charaka.

**Key citations:** Multiple entries from SS Ch. 26

---

## Beat 3: Honest refusal

Tests whether the agent correctly distinguishes "the graph has properties but not dosage" from a bare LLM that invents clinical specifics.

### Q6. What dosage of ashwagandha does Charaka recommend for daily use?

**Expected tool calls:**
```
vidya_query(mode="describe", domain="ayurveda", subject="ashwagandha")
```

**Expected agent answer:**
The graph records the following properties of ashwagandha from the Charaka Samhita and Bhavaprakasha:

- Rasa: kashaya (astringent), katu (pungent), tikta (bitter)
- Guna: snigdha (unctuous), laghu (light)
- Veerya: ushna (hot)
- Vipaka: madhura (sweet post-digestive)
- Karma: rasayana (rejuvenative), vajikarana (aphrodisiac), medhya (intellect-promoting), balya (strengthening)
- Dosha: pacifies vata and kapha; aggravates pitta
- Botanical: *Withania somnifera*

**Dosage is not recorded in the current dataset.** The graph captures textual properties and classifications from the samhita literature, not formulation dosages (matra). Charaka may discuss dosage in the Chikitsa Sthana or in specific formulation contexts, but those sections are not yet in the graph.

**Expected bare-LLM answer:**
This is the sharpest contrast. Bare Claude will likely say something like: "Ashwagandha is commonly taken at 300-600mg of root extract daily" or "Charaka recommends 3-6 grams of churna." These are modern supplement dosages or vaidya practice norms being back-projected onto Charaka. The bare LLM cannot distinguish what Charaka actually states in the samhita versus what modern practitioners recommend. It invents a specific number with false confidence.

**Key citations:** Provenance on ashwagandha properties from source-charaka and source-bhavaprakasha. No dosage citation exists.

---

## Beat 4: Audit click-through — from claim to verse text

Tests the full provenance chain: graph claim → citation → Kaviratna source text. The evaluator should be able to follow the citation to `sources/charaka-ss-26.txt` or `sources/charaka-ss-27.txt` and verify the verse says what the agent claims.

### Q7. What does Charaka say about the properties of pippali (long pepper)? Cite each claim to its source verse.

**Expected tool calls:**
```
vidya_query(mode="describe", domain="ayurveda", subject="pippali")
```

**Expected agent answer:**
Pippali (*Piper longum*, long pepper):

| Property | Value | Citation |
|----------|-------|----------|
| Rasa | katu (pungent) | *CS, Sutra Sthana, Ch. 27, v. 298* |
| Rasa | madhura (sweet) | *CS, SS Ch. 27, v. 298* |
| Guna | guru (heavy), snigdha (unctuous), ushna (hot), tikshna (sharp), laghu (light) | *CS, SS Ch. 27, v. 298* |
| Veerya | ushna (hot potency) | *CS, SS Ch. 27, v. 298* + *Bhavaprakasha* + *Sushruta* |
| Veerya | sheeta (cold, minority view) | *CS* (confidence 0.85) |
| Vipaka | madhura | *CS* (no verse-level citation) |
| Karma | rasayana, vajikarana, deepana, pachana | *CS, SS Ch. 27, v. 298* (vajikarana) |
| Pacifies | vata, kapha | *CS, SS Ch. 27, v. 298* |
| Aggravates | kapha, pitta | *CS, SS Ch. 27, v. 298* |

**Audit step for evaluator:** Open `sources/charaka-ss-27.txt`, search for verse 298. Verify that the text discusses pippali with the properties listed above.

**Expected bare-LLM answer:**
Gets many properties right (pippali is well-known). May say "pungent taste, hot potency" correctly. But cannot cite Ch. 27 v. 298 specifically, and cannot surface the interesting sheeta veerya minority tradition or the dual rasa (katu + madhura). Likely gives a generic "Charaka Samhita mentions" without location.

**Key citations:** SS 27.298 (primary), plus cross-tradition provenance from Bhavaprakasha and Sushruta on veerya.

---

### Q8. For Charaka's statement that madhura rasa is guru, snigdha, and sheeta — what is the exact source citation?

**Expected tool calls:**
```
vidya_query(mode="describe", domain="ayurveda", subject="madhura")
# or specifically:
vidya_query(mode="provenance", domain="ayurveda", subject="madhura", predicate="hasGuna", object="guru")
```

**Expected agent answer:**
The three guna assertions for madhura rasa all cite the same verse:

- madhura → guru: *Charaka Samhita, Sutra Sthana, Ch. 26, v. 57* (tradition: Atreya, pramana: shabda, confidence: 0.95)
- madhura → snigdha: *CS, SS Ch. 26, v. 57* (same provenance)
- madhura → sheeta: *CS, SS Ch. 26, v. 57* (same provenance)

**Audit step:** Open `sources/charaka-ss-26.txt`, find verse 57. The Kaviratna translation should describe madhura rasa as heavy, unctuous, and cold.

**Expected bare-LLM answer:**
May correctly state the three gunas of madhura (this is basic rasa theory). Absolutely cannot produce "Sutra Sthana, Ch. 26, v. 57" — will say "according to Charaka" at best. Cannot tell you the pramana type (shabda), the tradition lineage (Atreya), or a confidence score.

**Key citations:** SS 26.57

---

### Q9. Compare haritaki and amalaki according to Charaka — where do they agree and differ?

**Expected tool calls:**
```
vidya_query(mode="describe", domain="ayurveda", subject="haritaki")
vidya_query(mode="describe", domain="ayurveda", subject="amalaki")
```

**Expected agent answer:**

| Property | Haritaki | Amalaki |
|----------|----------|---------|
| Botanical | *Terminalia chebula* | *Emblica officinalis* |
| Common name | chebulic myrobalan | Indian gooseberry |
| Rasa | all five: madhura, amla, katu, tikta, kashaya | all five: madhura, amla, katu, tikta, kashaya |
| Guna | ruksha, laghu | ruksha, laghu, **sheeta** |
| **Veerya** | **ushna** (hot) | **sheeta** (cold) |
| Vipaka | madhura | madhura |
| Pacifies | vata, pitta, kapha (tridoshahara) | vata, pitta, kapha (tridoshahara) |
| Karma | rasayana, deepana, anulomana | rasayana |

Key finding: despite sharing five rasas, tridosha-pacifying nature, and madhura vipaka, they differ in **veerya** — haritaki is ushna, amalaki is sheeta. Amalaki also has sheeta guna that haritaki lacks. Haritaki has additional karmas (deepana, anulomana) not recorded for amalaki.

**Expected bare-LLM answer:**
Gets the broad comparison right (both are triphala ingredients, both rasayana). May correctly note the veerya difference. But cannot cite specific provenance for the claims, may conflate Charaka-specific data with Bhavaprakasha or modern compilations. Cannot surface the fine-grained guna difference (amalaki has sheeta guna, haritaki doesn't).

**Key citations:** Provenance on haritaki and amalaki from source-charaka and source-bhavaprakasha (vipaka).

---

## Running the bare-LLM comparison

To capture the comparison column, run each question against Claude (same model, same temperature) **without** the vidya MCP server attached. Use the same question text verbatim. Save the raw outputs alongside this script.

The key failure modes to watch for:
1. **No citations**: bare LLM says "according to Ayurveda" but can't point to a verse
2. **Confabulation**: invents specific numbers (dosage), conflates sources, or projects modern practice onto classical texts
3. **Hedging where the agent is precise**: bare LLM says "traditionally" or "some sources say" where the agent quotes a specific verse
4. **False confidence where the agent refuses**: bare LLM invents an ashwagandha dosage; agent says "not in the dataset"
5. **Missing the negative**: bare LLM doesn't think to check for viruddha rules when asked about turmeric+milk; answers from cultural knowledge instead of textual evidence
