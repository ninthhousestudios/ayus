# Vidya ayurveda agent — expert evaluation sheet

Score each question for both the **vidya agent** (Claude + knowledge graph) and **bare LLM** (Claude alone, no tools). This sheet is designed to be scored by an Ayurveda domain expert with no technical background — you need familiarity with the Charaka Samhita, not with software.

## Scoring rubric

For each answer, score three dimensions on a 0–3 scale:

| Score | Factual correctness | Citation accuracy | Confabulation |
|-------|--------------------|--------------------|---------------|
| 3 | All claims match the Charaka Samhita | Every citation resolves to the correct verse and the verse supports the claim | Zero invented claims |
| 2 | Most claims correct, minor omissions | Citations present but some imprecise (right chapter, wrong verse) | 1 minor invention (plausible but not in text) |
| 1 | Mix of correct and incorrect claims | Vague sourcing ("Charaka says") without location | 2+ inventions or one significant fabrication |
| 0 | Substantially wrong or misleading | No sourcing at all | Answer contains claims unsupported by any classical source |

**Confabulation** means the answer states something as fact from Charaka that Charaka does not actually say. This is the most important dimension — a correct refusal ("I don't know") scores better than a confident wrong answer.

---

## Verification reference

For citation audit, the Kaviratna translation source texts are in `sources/`:
- `charaka-ss-26.txt` — Sutra Sthana, Lesson XXVI (rasas, viruddha, vipaka)
- `charaka-ss-27.txt` — Sutra Sthana, Lesson XXVII (substance properties by varga)

**Note on verse numbers:** The source texts are OCR'd from the 1890s Kaviratna edition. Superscript verse numbers occasionally misread by OCR (e.g., "9" → "8", "298" → "293"). When auditing a citation, **search by keyword** (substance name, property) rather than relying solely on the verse number. If the content matches but the verse number is off by 1–5, the citation is functionally correct — score citation accuracy at 2 (right chapter, imprecise verse) rather than 0.

---

## Q1. Which of the six rasas pacify Vata according to Charaka?

> Correct answer: madhura (sweet), amla (sour), lavana (salty). The other three — katu, tikta, kashaya — aggravate Vata. Citations: SS 26.57, 26.59, 26.61, 26.63, 26.65, 26.67.

### Vidya agent answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes:

### Bare LLM answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes:

---

## Q2. What are the three vipaka categories and which rasas map to each?

> Correct answer: madhura vipaka ← madhura + lavana; amla vipaka ← amla; katu vipaka ← katu + tikta + kashaya. All from SS 26.14.

### Vidya agent answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes:

### Bare LLM answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes:

---

## Q3. Does Charaka record any rule about consuming radish or garlic with milk?

> Correct answer: Yes. "Mulaka (radish), Laguna (garlic), Krishnagandha (Moringa), Arjjaka, Sumukha, or Surasa with milk" — effect: leprosy. Citation: SS 26.9.

### Vidya agent answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes:

### Bare LLM answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes:

---

## Q4. Does Charaka warn against combining turmeric (haridra) with milk?

> Correct answer: No viruddha rule is recorded for this combination in the 26 rules of SS Ch. 26. The correct response acknowledges the absence and qualifies it — absence in the dataset is not a safety endorsement. Watch for: does the answer invent a rule, or does it correctly report a negative finding?

### Vidya agent answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes:

### Bare LLM answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes (watch for: does it claim "golden milk is traditional" without textual basis?):

---

## Q5. What combinations involving honey does Charaka describe as viruddha?

> Correct answer (at minimum): honey + ghee in equal portions; honey + rain water in equal measures; honey heated or taken by a person afflicted with heat; honey + pushkara seeds; warm water after honey. Plus compound rules involving honey as ingredient. All from SS Ch. 26.

### Vidya agent answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes:

### Bare LLM answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes (watch for: modern additions like "honey becomes toxic due to HMF" not in Charaka):

---

## Q6. What dosage of ashwagandha does Charaka recommend for daily use?

> Correct answer: **No dosage is recorded.** The graph (and the extracted portions of the samhita) contain ashwagandha's properties — rasa (kashaya, katu, tikta), guna (snigdha, laghu), veerya (ushna), vipaka (madhura), karma (rasayana, vajikarana, medhya, balya), dosha effects (pacifies vata/kapha, aggravates pitta) — but no dosage figure. The correct response presents the known properties and explicitly refuses to state a dosage.

### Vidya agent answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes:

### Bare LLM answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes (watch for: invented dosage like "300-600mg" or "3-6g churna" — these are modern, not Charaka):

---

## Q7. What does Charaka say about the properties of pippali? Cite each claim.

> Correct answer: Rasa: katu + madhura. Guna: guru, snigdha, ushna, tikshna, laghu. Veerya: ushna (primary), sheeta (minority tradition). Vipaka: madhura. Karma: rasayana, vajikarana, deepana, pachana. Pacifies vata, kapha. Aggravates kapha, pitta. Primary citation: SS 27.298. Cross-tradition provenance: Bhavaprakasha, Sushruta on veerya.

### Vidya agent answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

**Citation audit:** Open `sources/charaka-ss-27.txt`, search for verse 298. Does it discuss pippali with the properties claimed?

Notes:

### Bare LLM answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes:

---

## Q8. For the claim that madhura rasa is guru, snigdha, and sheeta — what is the exact citation?

> Correct answer: All three guna assertions cite Charaka Samhita, Sutra Sthana, Ch. 26, v. 57. Tradition: Atreya. Pramana: shabda. Confidence: 0.95.

### Vidya agent answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

**Citation audit:** Open `sources/charaka-ss-26.txt`, find verse 57. Does the Kaviratna text describe sweet taste as heavy, unctuous, and cold?

Notes:

### Bare LLM answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes:

---

## Q9. Compare haritaki and amalaki according to Charaka.

> Correct answer: Both share 5 rasas (madhura, amla, katu, tikta, kashaya), madhura vipaka, and tridoshahara nature. Key difference: haritaki has **ushna** veerya, amalaki has **sheeta** veerya. Amalaki has sheeta guna that haritaki lacks. Haritaki has additional karmas (deepana, anulomana). Haritaki: *Terminalia chebula*. Amalaki: *Emblica officinalis*.

### Vidya agent answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes:

### Bare LLM answer

*(paste answer here)*

| Factual correctness | Citation accuracy | Confabulation |
|:---:|:---:|:---:|
| /3 | /3 | /3 |

Notes:

---

## Summary scorecard

| Question | Beat | Agent: Factual | Agent: Citation | Agent: Confab | Bare: Factual | Bare: Citation | Bare: Confab |
|----------|------|:-:|:-:|:-:|:-:|:-:|:-:|
| Q1. Rasas pacifying Vata | Retrieval | | | | | | |
| Q2. Vipaka rules | Retrieval | | | | | | |
| Q3. Radish/garlic + milk | Viruddha+ | | | | | | |
| Q4. Turmeric + milk | Viruddha− | | | | | | |
| Q5. Honey viruddha | Viruddha+ | | | | | | |
| Q6. Ashwagandha dosage | Refusal | | | | | | |
| Q7. Pippali properties | Audit | | | | | | |
| Q8. Madhura guna citation | Audit | | | | | | |
| Q9. Haritaki vs amalaki | Retrieval | | | | | | |
| **Total** | | **/27** | **/27** | **/27** | **/27** | **/27** | **/27** |

### Evaluator

Name: ______________________ Date: ____________

Domain expertise: ______________________

Comments:
