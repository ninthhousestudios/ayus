# Same model, same questions — with and without the knowledge graph

Two sessions of Claude (Sonnet 4.6), June 2026. Both were asked the same nine questions about the Charaka Samhita, in the same order. One session was Claude alone. The other had access to **vidya**, a structured knowledge graph extracted from the Kaviratna English translation of the Charaka Samhita (with supplementary data from the Bhavaprakasha Nighantu), and was instructed to answer only from the graph and cite every claim to source, sthana, chapter, and verse.

The answers below are reproduced verbatim from the two transcripts. Nothing was edited or regenerated. Full screen recordings of both sessions are available as an appendix.

## What to look for

Modern language models are honest about the limits of their memory. Asked for an obscure incompatibility rule or an exact verse citation, Claude alone does the right thing: it declines to guess, and it recommends consulting a translation — P.V. Sharma, the CCRAS corpus, a verse-numbered edition.

That refusal is the point of this comparison. **The bare model's best answer is a referral to the source text. The knowledge-graph agent *is* that consultation, automated.** Where the bare model says "check Sutrasthana 26," the agent returns the rule itself, the recorded effect, and the verse — as a claim that can be checked against the source in seconds, because every triple in the graph carries provenance: tradition, source text, sthana, chapter, verse, and a confidence score.

Three things this comparison shows:

1. **Long-tail retrieval.** On common doctrine (Q1, Q2) the bare model is accurate — this material saturates its training data. On specific enumerations (Q3, Q5) it either declines or gives a partial answer. The agent returns complete, verse-cited lists.
2. **Citations as data, not prose.** Asked for an exact citation (Q8), the bare model explicitly refuses to produce one — correctly, because for it a verse number would be a guess. The agent returns the citation with its epistemological metadata (pramana, tradition, confidence) because the citation is *stored*, not recalled.
3. **Auditable absence.** When no rule exists (Q4) or the data is out of scope (Q6), the agent's "not recorded" is a claim about a defined dataset — 26 viruddha rules, ~150 substances — that anyone can verify. The bare model's equivalent hedge is unfalsifiable.

The comparison also shows the graph's current limits honestly: extraction was done by an LLM and is unvalidated by domain experts. In Q7 the agent surfaces contradictory triples the extraction produced — and flags them as contradictions rather than smoothing them over. That visibility is deliberate: every extracted claim is verse-anchored precisely so that experts can adjudicate it. Building that review loop is the collaboration this demo argues for.

---

## Question 1 — According to Charaka, which of the six rasas (tastes) pacify Vata dosha, and which aggravate it?

### Claude alone

> According to the Charaka Samhita, the six rasas (shad rasa) relate to Vata dosha as follows:
>
> **Pacify (shamana) Vata — 3 rasas:**
>
> | Rasa | Taste | Reason |
> |------|-------|--------|
> | Madhura | Sweet | Heavy, unctuous, cool — opposes Vata's dry, light, mobile nature |
> | Amla | Sour | Unctuous, heating, slightly heavy — grounds and warms Vata |
> | Lavana | Salty | Unctuous, heating, penetrating — counters dryness and coldness |
>
> **Aggravate (kopana) Vata — 3 rasas:**
>
> | Rasa | Taste | Reason |
> |------|-------|--------|
> | Katu | Pungent | Dry, light, hot — shares and amplifies Vata's mobile/dry qualities |
> | Tikta | Bitter | Dry, light, cold — strongly increases Vata's dryness and lightness |
> | Kashaya | Astringent | Dry, rough, cold — most drying of all, strongly elevates Vata |
>
> **The underlying principle** is *samanya-vishesha* (like increases like, opposites balance). Vata's core qualities are *ruksha* (dry), *laghu* (light), *shita* (cold), *khara* (rough), and *chala* (mobile). Sweet, sour, and salty share opposite qualities (snigdha/unctuous, guru/heavy, ushna/warm), so they pacify it. Bitter and astringent most strongly share Vata's dryness, which is why Charaka singles out tikta as especially Vata-aggravating.

### Claude + vidya

> Here is what the graph records for all six rasas and their relationship to Vata dosha, drawn entirely from the Atreya tradition as recorded in the Charaka Samhita, Sutra Sthana, Ch. 26.
>
> #### Pacify Vata (shamana)
>
> | Rasa | Common name | Gunas | Source |
> |------|-------------|-------|--------|
> | **Madhura** | Sweet | guru, snigdha, sheeta | *CS, Su. St., Ch. 26, v. 57* |
> | **Amla** | Sour | laghu, snigdha, ushna | *CS, Su. St., Ch. 26, v. 59* |
> | **Lavana** | Salty | guru, snigdha, ushna | *CS, Su. St., Ch. 26, v. 61* |
>
> The logic is consistent with the guna principle: Vata is dry (ruksha), light (laghu), and cold (sheeta), so rasas whose gunas include snigdha (unctuous/oily) or ushna (hot) counteract it. Madhura is an exception — it is sheeta (cold) but its heaviness and unctuousness still counteract Vata's lightness and dryness.
>
> #### Aggravate Vata (kopana)
>
> | Rasa | Common name | Gunas | Source |
> |------|-------------|-------|--------|
> | **Katu** | Pungent | ushna, ruksha, laghu | *CS, Su. St., Ch. 26, v. 63* |
> | **Tikta** | Bitter | sheeta, ruksha, laghu | *CS, Su. St., Ch. 26, v. 65* |
> | **Kashaya** | Astringent | sheeta, ruksha, guru | *CS, Su. St., Ch. 26, v. 67* |
>
> These three share ruksha (dry) and/or laghu (light) — qualities that mirror and therefore increase Vata. Tikta and Kashaya also add sheeta (cold), compounding Vata's inherent coldness.
>
> #### Summary
>
> **Three pacify Vata:** madhura (sweet), amla (sour), lavana (salty).
> **Three aggravate Vata:** katu (pungent), tikta (bitter), kashaya (astringent).
>
> The pattern maps cleanly onto Vata's own gunas: the pacifying rasas bring snigdha and/or ushna to counteract dryness and cold; the aggravating rasas bring ruksha and/or sheeta, which amplify Vata's characteristics. Note that katu (pungent) is ushna but still aggravates Vata — the text records this; its ruksha and laghu qualities apparently dominate.

**What to notice.** Both answers agree on the doctrine — this is foundational material the bare model knows well. The difference is that every row of the agent's tables carries a verse number, and each one resolves to a specific location in the Kaviratna translation. The bare answer is correct but unauditable.

---

## Question 2 — According to Charaka, what are the three categories of vipaka (post-digestive effect), and which rasas map to each?

### Claude alone

> According to the Charaka Samhita, vipaka is the taste that emerges after *agni* (digestive fire) has fully transformed a substance — distinct from the initial rasa perceived on the tongue. Charaka uses a **tri-vipaka** system:
>
> | Vipaka | Post-digestive taste | Rasas that yield it |
> |--------|---------------------|---------------------|
> | **Madhura** | Sweet | Madhura (sweet) + **Lavana** (salty) |
> | **Amla** | Sour | Amla (sour) |
> | **Katu** | Pungent | Katu (pungent) + Tikta (bitter) + Kashaya (astringent) |
>
> **Key points:**
>
> - **Lavana → Madhura vipaka** is the notable surprise. Salt tastes salty initially but converts to a sweet post-digestive effect. This is specific to Charaka's scheme; Sushruta places lavana in amla vipaka.
> - **Amla** is the only rasa that maps to its own vipaka (sour in = sour out).
> - Three rasas (katu, tikta, kashaya) collapse into a single katu vipaka — they are already "similar" in their dry, light qualities.
>
> **Dosha implications of each vipaka:**
>
> | Vipaka | Net effect |
> |--------|-----------|
> | Madhura | Nourishes dhatus, promotes ojas, pacifies Vata + Pitta, increases Kapha |
> | Amla | Mildly heating, stimulates agni, can aggravate Pitta and Rakta |
> | Katu | Catabolic, drying, reduces Kapha and shukra (semen/reproductive tissue), aggravates Vata |
>
> Vipaka is considered more clinically significant than rasa alone because it reflects what actually reaches the tissues (*dhatus*) after digestion is complete.

### Claude + vidya

> The graph returns three VipakaRule entities, all sourced to *Charaka Samhita, Sutra Sthana, Ch. 26, v. 14* for the rasa-to-vipaka mappings, and the vipakaEffect descriptions from v. 17–18.
>
> #### 1. Madhura Vipaka (sweet post-digestive effect)
>
> **Input rasas:** madhura (sweet), lavana (salty)
>
> **Effect recorded in the graph:** *"easy and copious discharges of urine and stools; increases phlegm and semen; produces heaviness"*
>
> Both the sweet and salty tastes, despite differing initially, converge on a sweet post-digestive effect — building, moistening, and increasing Kapha and semen.
>
> *CS, Su. St., Ch. 26, v. 14 (rule); v. 17 (effect)*
>
> #### 2. Amla Vipaka (sour post-digestive effect)
>
> **Input rasa:** amla (sour)
>
> **Effect recorded in the graph:** *"increase of bile; evacuation of stools and urine; destruction of semen"*
>
> The sour taste is alone in its category — it digests into a sour vipaka, increasing Pitta and bile.
>
> *CS, Su. St., Ch. 26, v. 14 (rule); v. 18 (effect)*
>
> #### 3. Katu Vipaka (pungent post-digestive effect)
>
> **Input rasas:** katu (pungent), tikta (bitter), kashaya (astringent)
>
> **Effect recorded in the graph:** *"destructive of semen; leads to retention of faeces and urine, and to increase of wind"*
>
> Three tastes converge here. Despite their surface differences, pungent, bitter, and astringent all produce a drying, reducing, Vata-increasing post-digestive effect.
>
> *CS, Su. St., Ch. 26, v. 14 (rule); v. 17 (effect)*
>
> #### Summary table
>
> | Vipaka | Input rasas (6 → 3) | Broad effect |
> |--------|---------------------|-------------|
> | Madhura | madhura, lavana | Building; increases Kapha, semen |
> | Amla | amla | Increases bile/Pitta; reduces semen |
> | Katu | katu, tikta, kashaya | Drying; increases Vata; reduces semen, retains excreta |
>
> A notable pattern: six rasas collapse into three vipakas, and all three categories are described as destructive of semen (shukra) to varying degrees — a point the text emphasizes. The dataset records the rule mapping and the effects but does not include Charaka's explanatory rationale for *why* lavana digests madhura or why tikta/kashaya converge with katu; that reasoning is not in the current graph.

**What to notice.** Again both are correct, including the lavana → madhura subtlety. Two differences: the agent quotes the *recorded effects verbatim from the translation* rather than paraphrasing tradition, and it explicitly states what the graph does *not* contain (Charaka's rationale) — the boundary between data and interpretation stays visible.

---

## Question 3 — Does Charaka record any rule about consuming radish or garlic with milk? If so, what does he say?

### Claude alone

> This is a question where I should be transparent about the limits of my recall rather than risk fabricating a specific rule.
>
> **What I can say with confidence:**
>
> - Charaka Samhita's **Sutrasthana Chapter 26** is the primary locus for *viruddha ahara* (incompatible food combinations), and it lists numerous specific pairings.
> - Milk combinations feature prominently — fish with milk, sour substances with milk, and hot/cold pairings are among the classic examples.
> - Radish (*mulaka*) is discussed in the context of incompatibilities in the Ayurvedic literature broadly, and the pairing of radish with milk is widely cited across the tradition as problematic.
>
> **Where I'm uncertain:**
>
> I don't have sufficient confidence that I can accurately reproduce whether Charaka *specifically* names radish-with-milk or garlic-with-milk as a viruddha pair, versus those appearing in later commentators (like Chakrapanidatta) or in Sushruta, Ashtanga Hridayam, or the Nighantus. The risk of confusing sources or paraphrasing incorrectly is real for a question this textually precise.
>
> **My recommendation:**
>
> For a question at this level of specificity, consult the P.V. Sharma translation of Charaka Samhita or the Priya Vrat Sharma *Dravyaguna Vijnana* directly, checking Sutrasthana 26 for viruddha ahara and the *mulaka* and *lashuna* entries in the Dravyaguna sections.

### Claude + vidya

> Yes — the graph records two distinct viruddha rules that are relevant, both from *Charaka Samhita, Sutra Sthana, Ch. 26*.
>
> #### Rule 1 — The direct radish-and-garlic-with-milk rule
>
> **Combination:** Mulaka (radish), Laguna (garlic), Krishnagandha (*Moringa pterygosperma*), Arjjaka, Sumukha, or Surasa (varieties of Ocimum) with milk
>
> **Recorded effect:** leprosy (*kushtha*)
>
> *CS, Sutra Sthana, Ch. 26, v. 9*
>
> This is the primary rule: Charaka groups radish and garlic together with several other pungent, hot substances (the Ocimum varieties, Moringa) and names their combination with milk as a viruddha producing leprosy. The textual logic is likely the clash of opposing qualities — milk is sweet, cold, and heavy (snigdha, sheeta, guru), while these substances are predominantly pungent, hot, and light — but the graph records the rule and its effect, not Charaka's explanatory rationale.
>
> #### Rule 2 — A secondary rule involving radish (not garlic)
>
> **Combination:** Meat of domesticated, marshy, or aquatic animals with honey, sesame oil, molasses, milk, Masha (*Phaseolus radiatus*), Mulaka (radish), lotus stalks, or sprouted paddy
>
> **Recorded effect:** deafness, blindness, paralysis agitans, rigidity, lassitude, dumbness, nasal voice, or death
>
> *CS, Sutra Sthana, Ch. 26, v. 7*
>
> Here radish (mulaka) appears again, but embedded in a larger incompatible combination — the trigger is specifically meat of these animal categories eaten together with any of the listed accompaniments including milk and radish. Garlic is not part of this rule. The recorded effects are severe and neurological in character.
>
> #### Summary
>
> | Substances | Rule type | Recorded effect | Source |
> |------------|-----------|-----------------|--------|
> | Radish + garlic (+ other pungents) with milk | Direct pairing | Leprosy | *CS, Su. St., Ch. 26, v. 9* |
> | Meat (domesticated/marshy/aquatic) + milk + radish (among others) | Complex combination | Deafness, blindness, paralysis, death | *CS, Su. St., Ch. 26, v. 7* |
>
> **Scope note:** These rules are textual enumerations of specific combinations Charaka identifies as viruddha. The dataset covers the 26 combinations recorded in Sutra Sthana Ch. 26. This is not an exhaustive survey of all incompatibilities in the tradition, and the graph records what the text states — not clinical validation.

**What to notice.** This is the clearest illustration of the gap. The bare model knows the question is answerable — it even names the right chapter — but correctly declines to state the rule, and ends by recommending exactly the consultation the agent performs. The agent returns the rule verbatim, its recorded effect, its verse, and a second relevant rule the question didn't ask about.

---

## Question 4 — Does Charaka warn against combining turmeric (haridra) with milk?

### Claude alone

> No — and this is actually the **reverse** of what Charaka recommends.
>
> **Turmeric with milk is therapeutic in Charaka's framework**, not contraindicated. *Haridra* (turmeric) is classified as having katu-tikta rasa, ushna virya, and katu vipaka — and milk (madhura, shita, snigdha) is frequently used as its *anupana* (vehicle) precisely because the milk moderates turmeric's heating and drying qualities while enhancing its tissue-level delivery.
>
> Charaka discusses haridra in several contexts:
>
> - As a *kushtha* (skin disease) remedy, often administered with milk or ghee as the vehicle
> - In *prameha* (metabolic/urinary disorders) formulations
> - As a *varnya* (complexion-enhancing) dravya
>
> The combination of turmeric and milk as a therapeutic compound (*haridra dugdha* or *kshira paka* preparations) is well within the Charaka tradition and does not appear in the viruddha ahara lists.
>
> **The broader point** worth flagging for your study: turmeric-milk is one of the cases where popular modern Ayurvedic advice ("golden milk") is actually consistent with classical usage, not a folk corruption of it. The incompatible combinations Charaka flags tend to involve conflicting virya (e.g., hot + cold simultaneously), conflicting vipaka, or specific substance pairs — turmeric and milk don't fall into those categories.
>
> If you are working through the viruddha ahara list systematically, the combinations Charaka most emphasizes are fish+milk, equal-quantity honey+ghee, and hot+cold foods taken together.

### Claude + vidya

> **No.** The graph records no viruddha rule pairing haridra (turmeric) with milk. I scanned all 26 incompatibility rules from Sutra Sthana Ch. 26 in the previous query — haridra appears in none of them.
>
> Haridra is present in the graph as a Dravya with a full property profile, but its entry contains no viruddha association of any kind:
>
> | Property | Value |
> |----------|-------|
> | Rasa | katu (pungent), tikta (bitter) |
> | Veerya | ushna (hot) |
> | Vipaka | katu |
> | Guna | ruksha (dry), laghu (light) |
> | Pacifies | kapha, pitta |
> | Aggravates | vata |
> | Karma | shothahara (anti-inflammatory), krimighna (antiparasitic) |
> | Botanical | *Curcuma longa* |
>
> *(Provenance is Charaka tradition; verse-level citations were not returned for most of these properties.)*
>
> **What this means textually:** The combination of turmeric and milk is widely promoted in modern wellness contexts as "golden milk," but this use is not recorded in the dataset as either a recommended combination or a warned-against one. The guna contrast is apparent — haridra is hot, dry, and light, while milk is cold, unctuous, and heavy — but Charaka does not enumerate this pair among the 26 viruddha combinations in the current dataset.
>
> **Scope reminder:** The dataset covers the specific combinations Charaka names in Sutra Sthana Ch. 26. Absence of a rule here is not a safety claim — it means only that this combination is not among those Charaka explicitly enumerated as viruddha. The tradition is larger than what this graph currently holds.

**What to notice.** Both answers say no, but the epistemics differ. The bare model asserts confidently that turmeric-milk "does not appear in the viruddha ahara lists" — plausible, but resting on unverifiable recall, padded with therapeutic claims the question didn't ask about. The agent's "no" is a check against an enumerated, inspectable set of 26 rules, and it is explicit that absence of a rule is a fact about the dataset, not a safety claim.

---

## Question 5 — What food combinations involving honey does Charaka describe as viruddha (incompatible)?

### Claude alone

> The most clearly attested honey-related viruddha rules in Charaka Samhita are:
>
> **1. Honey + ghee in equal quantities**
> This is the most famous honey viruddha in Charaka. The individual substances are compatible and often therapeutic — but *sama matra* (equal proportions by weight) renders the combination harmful, described as producing toxic effects. The proportions matter: unequal quantities (e.g., more ghee than honey) are not prohibited.
>
> **2. Honey + heat**
> Charaka is explicit that honey should never be heated or mixed with hot substances (*ushna dravyas*, hot water, hot food). Heating honey is said to make it *abhishyandi* (channel-blocking) and to alter its properties in a way that produces toxic byproducts. This is one of the more practically significant warnings because it runs counter to common kitchen habits.
>
> **Where I want to be careful:**
>
> Beyond these two, Charaka's viruddha list includes other honey combinations (some texts cite honey with rain water, honey with certain fats in equal measure, honey with fermented substances), but I am not confident enough in the precise textual attribution to Charaka specifically — versus later commentators or parallel Samhitas — to state them as definite Charaka positions.
>
> **The underlying principle** for honey viruddha appears to be *guna-viruddha* (quality incompatibility): honey is dry, light, and slightly heating in virya, and combinations that double its heating tendency (hot substances) or create guna-conflict (equal ghee, which is heavy and unctuous) are what Charaka flags.
>
> The equal-proportion honey+ghee rule is worth memorizing as a high-confidence Charaka citation.

### Claude + vidya

> The graph returns 10 viruddha rules involving honey (plus one borderline case). Here is the complete picture, organized by type.
>
> #### Group A — Honey as sole or primary subject
>
> | Combination | Recorded effect | Verse |
> |-------------|-----------------|-------|
> | Honey and ghee **in equal portions** | incompatible (poisonous) | v. 20 |
> | Honey and rain water **in equal measures** | incompatible | v. 20 |
> | Honey and Pushkara seeds (lotus seeds) | incompatible | v. 20 |
> | Honey **taken in a heated state**, or consumed by a person afflicted with heat | death | v. 19 |
> | **Warm water drunk after** taking honey | incompatible | v. 20 |
>
> Several of these cluster at v. 20 and share the ratio principle: equal-measure or equal-weight combinations of honey with certain substances are specifically what Charaka flags, not the combination per se at other proportions. The heated-honey rule (v. 19) is the most severe, recording death as the effect and standing on its own verse. The warm-water-after-honey rule is a temporal incompatibility — a sequence, not a simultaneous combination.
>
> #### Group B — Honey as one component in a multi-substance combination
>
> | Combination | Recorded effect | Verse |
> |-------------|-----------------|-------|
> | Bamboo leaves and ripe Lakuchas (*Artocarpus lakoocha*) **with honey and milk** | death; or loss of strength, complexion, energy, vital seed; violent diseases; impotence | v. 10 |
> | Paushkara (lotus leaves) and Rohinika (*Picrorhiza kurroa*) potherbs, or pigeons fried in oil, **with honey and milk** | corruption of blood; exhaustion of nerves; delusions; pain and heat with puffiness in temples/forehead; bronchocele; red eruptions; or death | v. 8 |
> | Pippali (*Piper longum*) or Kakamachi (*Solanum nigrum*) boiled in fish-frying oil, taken **with honey** | death | v. 18 |
> | Padmottarika (*Carthamus tinctorius*) **with sugar-Maireya wine and honey** | excites/provokes wind exceedingly | v. 13 |
> | Meat of domesticated, marshy, or aquatic animals **with honey** (also: sesame oil, molasses, milk, Masha, Mulaka, lotus stalks, or sprouted paddy) | deafness, blindness, paralysis agitans, rigidity, lassitude, dumbness, nasal voice, or death | v. 7 |
>
> #### Borderline case — Kshaudra wine
>
> | Combination | Recorded effect | Verse |
> |-------------|-----------------|-------|
> | Green pigeon (Haritaka) with Kadamba-wood ashes, mixed with Kshaudra wine | death | v. 17 |
>
> *Kshaudra* in Sanskrit is a term for wild or minor honey; "Kshaudra wine" likely refers to a fermented honey preparation (mead-type). The graph labels it as a wine rather than raw honey, so it is not counted in the main list above, but is noted here for completeness.
>
> #### Summary observations
>
> The graph records **10 distinct viruddha rules** naming honey, covering:
> - **Proportional incompatibility:** equal-measure honey + ghee, or honey + rainwater
> - **Thermal incompatibility:** heated honey, or warm water following honey — the most lethal category
> - **Combinatorial incompatibility:** honey with fish-related preparations, with certain potherbs and milk, with certain meats, with lotus seeds, with specific wines
> - **Severity range:** from "incompatible" (unspecified) to death, with several rules naming systemic effects (blood corruption, neurological signs, reproductive damage)
>
> The most widely cited rule in later literature — honey and ghee in equal portions — appears here at v. 20 with the gloss "poisonous," not merely "incompatible." All ten rules are attributed to the Atreya tradition, Charaka Samhita, Sutra Sthana, Ch. 26.

**What to notice.** The bare model retrieves the two famous rules and — again correctly — declines to enumerate further. The agent returns all ten, with recorded effects and verse numbers, and separates a borderline case with reasoning. Completeness on enumeration questions is where parametric recall stops and retrieval starts.

---

## Question 6 — What dosage of ashwagandha does Charaka recommend for daily use?

### Claude alone

> This is another question where I should be honest rather than produce a number I'm not certain of.
>
> **What I can say with confidence:**
>
> - Charaka discusses *ashwagandha* (*Withania somnifera*) primarily in the **Rasayana** context (Chikitsasthana Chapter 1) and as a *balya* (strength-promoting) and *vajikara* (aphrodisiac/reproductive tonic) herb.
> - The classic Charaka rasayana preparation involves ashwagandha taken with milk (*kshira*) and often ghee, over a sustained period — consistent with rasayana protocols generally.
> - Charaka uses weight-based dose units (*karsha*, *tola*, *pala*) for prescriptions, and dosage in his framework varies by preparation form (churna, kwatha, avaleha), patient constitution, season, and therapeutic goal.
>
> **Where I'm uncertain:**
>
> I cannot confidently state a specific Charaka-attributed dosage figure for ashwagandha without risking fabricating a number or misattributing a figure from later commentators, Ashtanga Hridayam, or modern Ayurvedic standardization. The commonly cited 3–6g/day for ashwagandha churna reflects contemporary Ayurvedic clinical practice but I cannot reliably trace that specific figure to a Charaka shloka.
>
> **Recommendation:**
>
> Check Chikitsasthana Chapter 1 (Rasayana Adhyaya) directly, and the *ashwagandha* entry in P.V. Sharma's *Dravyaguna Vijnana* which cross-references classical sources carefully. Dosage questions in classical texts are particularly easy to misattribute.

### Claude + vidya

> **Dosage is not recorded in the current dataset.**
>
> The graph holds ashwagandha's pharmacological profile but no matra (dosage) data. Here is what it does record:
>
> | Property | Value |
> |----------|-------|
> | Rasa | kashaya, katu, tikta |
> | Veerya | ushna |
> | Vipaka | madhura |
> | Guna | snigdha, laghu |
> | Pacifies | vata, kapha |
> | Aggravates | pitta |
> | Karma | rasayana, vajikarana, medhya, balya |
> | Botanical | *Withania somnifera* |
>
> *(Provenance is Charaka Samhita and Bhavaprakasha, Atreya and Dhanvantari traditions; verse-level citations were not returned for most properties.)*
>
> **Why dosage is absent:** Matra (quantity), anupana (vehicle), and yoga (formulation) are outside the scope of the current graph. The dataset captures substance-level properties — taste, quality, potency, post-digestive effect, karma, and dosha relationships — as extracted from the Charaka Samhita and Bhavaprakasha Nighantu. Dosage in classical texts is typically context-dependent: it varies by the patient's constitution (prakriti), strength (bala), season, and the specific therapeutic aim, and is given in the chikitsa (treatment) sthanas rather than in the dravya-guna sections that this graph primarily draws from.
>
> Absence of dosage data here is not evidence that Charaka is silent on it — it reflects the current scope of the dataset, not the text.

**What to notice.** Both refuse, and both refuse well. The difference is the *shape* of the refusal: the bare model's boundary is "I can't trust my memory"; the agent's boundary is a stated scope of a defined dataset — properties in, dosage out — with the substance's full recorded profile still delivered. Neither system gives clinical advice; the agent's design makes that boundary structural rather than judgment-dependent.

---

## Question 7 — What does Charaka say about the properties of pippali (long pepper)? Cite each claim to its source verse.

> ⚠️ **Draft note:** the agent's answer below surfaces extraction artifacts (contradictory kapha entries, dual veerya) currently under source-verse review. This section will be regenerated from the corrected dataset before the doc is finalized.

### Claude alone

> I want to flag something important about how this question is framed before answering.
>
> **On pippali's properties in Charaka's framework — general level:**
>
> Pippali is discussed across multiple sections of Charaka Samhita. The properties consistently attributed to it in the Charaka tradition include:
>
> - **Rasa:** Katu (pungent)
> - **Vipaka:** Madhura (sweet) — this is the notable teaching point; pippali is one of the clearest examples where vipaka diverges from rasa
> - **Virya:** Generally considered anushna (mildly heating, less intensely hot than black pepper or dry ginger) — differentiating it within Trikatu
> - **Dosha effect:** Pacifies Vata and Kapha; considered deepana, pachana, and used extensively in respiratory and digestive conditions
> - **Rasayana use:** Pippali rasayana (graduated dose protocol) is a major Charaka therapeutic
>
> **On verse-level citations — I cannot provide them:**
>
> I know enough to discuss pippali's properties in Charaka's framework with reasonable accuracy. I do not have reliable enough recall of specific adhyaya and shloka numbers to cite them without meaningful risk of fabricating references. Producing verse citations I am not certain of would be more harmful than useful — especially for someone doing serious textual study.
>
> If you need verse-level citations, the right approach is:
> - P.V. Sharma's *Dravyaguna Vijnana* (cites source verses per property)
> - Prabhakara Rao's annotated Charaka translation
> - The CCRAS digital Charaka corpus if accessible to you

### Claude + vidya

> Here is pippali's complete profile from the graph, with each claim cited precisely to the level of detail the provenance data supports.
>
> #### Claims cited to a specific verse
>
> All of the following are attributed to the Atreya tradition, *Charaka Samhita, Sutra Sthana, Ch. 27, v. 298* (confidence 0.90):
>
> | Property | Value |
> |----------|-------|
> | Rasa | katu (pungent), madhura (sweet) |
> | Guna | ushna (hot), guru (heavy), snigdha (unctuous) |
> | Veerya | ushna (hot) |
> | Karma | vajikarana (aphrodisiac) |
> | Pacifies | vata, kapha |
> | Aggravates | kapha |
>
> Note on the kapha entry: the graph records pippali as both pacifying and aggravating kapha, both citing v. 298. This likely reflects a contextual distinction in the original text — perhaps fresh vs. dried pippali, or moderate vs. excess use — but the graph does not record the qualifier. The raw text would need to be consulted to resolve this.
>
> Veerya = ushna is additionally cross-confirmed by the Dhanvantari tradition (*Sushruta Samhita* and *Bhavaprakasha Nighantu*), both without verse-level detail.
>
> #### Claims attributed to Charaka tradition but without verse-level citation
>
> | Property | Value | Confidence |
> |----------|-------|------------|
> | Vipaka | madhura | 0.95 |
> | Rasa | katu (second assertion) | 0.95 |
> | **Veerya** | **sheeta (cold)** | **0.85** |
>
> The sheeta veerya entry is the most textually significant: pippali is recorded as having *both* ushna and sheeta veerya, but the sheeta assertion carries lower confidence (0.85) and no verse citation. This is a known point of classical debate — some texts describe pippali's veerya as sheeta (especially in the context of its sweet vipaka and reproductive-tonic karma), while others give ushna. The graph reflects this ambiguity but cannot resolve it without the underlying verse.
>
> #### Properties in the graph without any provenance annotation
>
> | Property | Value |
> |----------|-------|
> | Guna | tikshna (sharp), laghu (light) |
> | Karma | rasayana (rejuvenating), deepana (digestive stimulant), pachana (digestant) |
> | Aggravates | pitta |
> | Varga | varga_harita (green vegetables group) |
>
> These are present in the dataset but lack annotated provenance — no tradition, source, or verse metadata was returned for them. They should be treated as lower-confidence entries until the source verses are verified.

**What to notice.** The bare model knows pippali well but draws a hard line at verse citations — and points to external references. The agent's answer is stratified *by evidence quality*: verse-cited claims, tradition-level claims, and unprovenanced claims are reported separately, and a contradiction in the extracted data is flagged rather than hidden. This is what an expert review queue looks like from the inside: the graph knows exactly which of its own claims need adjudication.

---

## Question 8 — For Charaka's statement that madhura (sweet) rasa is guru, snigdha, and sheeta — what is the exact source citation?

### Claude alone

> I don't know the exact source citation, and I won't fabricate one.
>
> This is a good test case because the **claim itself is plausible and consistent** with Charaka's framework — madhura rasa being guru (heavy), snigdha (unctuous), and sheeta (cold) is exactly what the tradition holds, and these qualities explain why madhura pacifies Vata and Pitta while increasing Kapha. So it would be easy to produce a confident-sounding citation like "Sutrasthana 26.43" and have it go undetected. I'm not going to do that.
>
> **What I can say honestly:**
>
> The properties of the six rasas, including madhura's guru-snigdha-sheeta character, are treated systematically in Charaka's Sutrasthana — the chapters on dravya, guna, and rasa are the likely locus. Chapter 26 (Atreyabhadrakapyiya) and Chapter 27 (Annapanavidhi) are both candidates, as is the broader rasa discussion in Sutrasthana.
>
> **For an exact citation you can trust:**
>
> - Check the index of P.V. Sharma's translation under *madhura rasa* or *shad rasa*
> - Cross-reference with Chakrapanidatta's commentary (*Ayurveda Dipika*), which often preserves the precise shloka location
> - The Krishnamurthy translation includes verse numbers that can be verified against the Sanskrit
>
> If you have access to the Sanskrit text or a verse-numbered translation, I'm happy to help you interpret or contextualize what you find. But producing a verse number from memory for a citation-level question is where I draw the line.

### Claude + vidya

> All three triples resolve to the same single source, with no conflicting assertions:
>
> **Charaka Samhita, Sutra Sthana, Chapter 26, Verse 57**
> Tradition: Atreya | Pramana: shabda (textual testimony) | Confidence: 0.95
>
> | Triple | Citation |
> |--------|----------|
> | madhura **hasGuna** guru | *CS, Su. St., Ch. 26, v. 57* |
> | madhura **hasGuna** snigdha | *CS, Su. St., Ch. 26, v. 57* |
> | madhura **hasGuna** sheeta | *CS, Su. St., Ch. 26, v. 57* |
>
> All three claims originate from the same verse — Charaka states the three gunas of madhura rasa together in a single locus. There is one source assertion per triple, no cross-tradition disagreement, and no lower-confidence alternative reading recorded in the dataset.
>
> For completeness: the same verse (v. 57) is also the source for madhura's dosha effects (pacifies vata and pitta, aggravates kapha) as returned in Question 1. The full madhura profile in the graph is essentially a single-verse entry.

**What to notice.** The sharpest contrast in the series. The bare model articulates precisely why it must refuse: a fabricated verse number would be plausible and undetectable. The agent answers in seconds — because for it, a citation is not a memory to be trusted but a stored assertion with metadata: source, tradition, pramana classification, confidence. Each triple's provenance can be followed to the verse in the translation and checked.

---

## Question 9 — Compare haritaki and amalaki according to Charaka — where do their properties agree and where do they differ?

> ⚠️ **Draft note:** the agent's answer below reflects the graph as extracted; the identical five-rasa profiles for both substances are under source-verse review (classically the five-rasa profile is haritaki's signature distinction). This section will be regenerated from the corrected dataset before the doc is finalized.

### Claude alone

> This is a question I can engage with substantively at the tradition level, while being clear about confidence gradations.
>
> #### Where haritaki and amalaki agree
>
> **Both possess panca-rasa (five tastes)** — Both herbs contain all six rasas except lavana (salty) — they share madhura, amla, katu, tikta, and kashaya. This five-rasa character is considered significant because it makes both herbs broadly applicable across conditions and constitutions.
>
> **Both have madhura vipaka** — Despite their complex rasa profiles, both yield a sweet post-digestive effect — placing both in the tissue-nourishing, Vata-and-Pitta-pacifying category at the level of final metabolic effect.
>
> **Both are tridosha-shamana** — Both are considered capable of balancing all three doshas, which is relatively rare and partly explains their prominence in rasayana formulations.
>
> **Both are major rasayanas** — Charaka treats both as premier rejuvenating substances. They appear together in Triphala (with bibhitaki) and both are given extended treatment in the Rasayana Adhyaya of Chikitsasthana.
>
> #### Where they differ
>
> **Virya — the primary differentiator:**
>
> | | Haritaki | Amalaki |
> |--|--|--|
> | Virya | Ushna (heating) | Sheeta (cooling) |
> | Gunas | Laghu (light), ruksha (dry) | Guru (heavy), snigdha (unctuous) |
> | Primary dosha affinity | Strongest for Vata | Strongest for Pitta |
>
> **Practical consequences of the virya difference:**
> - Haritaki's ushna virya makes it the preferred choice in Vata-predominant conditions — cold, dry, constipated presentations — and as a deepana/anulomana (digestive stimulant and mild laxative)
> - Amalaki's sheeta virya makes it preferred in Pitta-predominant conditions — inflammatory, bleeding, hyperacid presentations — and is specifically emphasized as chakshushya (beneficial for the eyes) and for rakta (blood tissue)
>
> **Predominant rasa:**
> While both have five rasas, haritaki's most prominent taste is often given as kashaya (astringent), while amalaki's is clearly amla (sour) — to the point where amalaki's Sanskrit name references the sour taste directly.
>
> #### Confidence note
>
> The virya, guna, vipaka, and tridoshic properties described above are well-grounded in the Charaka tradition. The specific framing of the five-rasa character for both herbs and the cooling/heating virya contrast are standard Charaka teachings. I would not produce verse numbers for these claims, but I hold them with higher confidence than the dosage or specific combination questions earlier in this series.

### Claude + vidya

> Here is the full comparison drawn from the graph.
>
> | Property | Haritaki (*Terminalia chebula*) | Amalaki (*Emblica officinalis*) |
> |----------|--------------------------------|--------------------------------|
> | **Rasa** | amla, kashaya, katu, tikta, madhura | amla, kashaya, katu, tikta, madhura |
> | **Veerya** | **ushna** (hot) | **sheeta** (cold) |
> | **Vipaka** | madhura | madhura |
> | **Guna** | ruksha, laghu | ruksha, laghu, **sheeta** |
> | **Pacifies** | vata, kapha, pitta | vata, kapha, pitta |
> | **Karma** | rasayana, deepana, anulomana | rasayana |
>
> #### Where they agree
>
> **1. Rasa — identical five-rasa profile.** Both substances share the same five rasas: amla, kashaya, katu, tikta, and madhura — lacking only lavana. This is an unusual and noteworthy identity. In the classical literature this five-rasa profile is specifically associated with haritaki's distinction; amalaki's dominant rasa is amla, with the others secondary.
>
> **2. Vipaka — both madhura.** Both undergo sweet post-digestive transformation. Haritaki's madhura vipaka is cross-confirmed by the Bhavaprakasha Nighantu (*source-bhavaprakasha*, confidence 0.90) in addition to Charaka.
>
> **3. Gunas — shared core.** Both are ruksha (dry) and laghu (light).
>
> **4. Dosha action — both tridoshahara.** Both pacify all three doshas — comparatively rare in the dataset.
>
> **5. Karma — both rasayana.**
>
> #### Where they differ
>
> **1. Veerya — the key pharmacological divergence:**
> - Haritaki: **ushna** (hot) — *Charaka Samhita*, Atreya tradition, confidence 0.95
> - Amalaki: **sheeta** (cold) — *Charaka Samhita* and *Bhavaprakasha*, both traditions, confidence 0.95/0.90
>
> Two substances with nearly identical rasa and vipaka profiles, both tridoshahara, diverge sharply on veerya. This has clinical implications in the source texts — haritaki's warmth suits cold, damp, or stagnant conditions; amalaki's coolness suits heat and pitta inflammation — but the graph records the property, not the therapeutic rationale.
>
> **2. Guna — amalaki adds sheeta** as an explicit guna in addition to ruksha and laghu, consistent with its sheeta veerya.
>
> **3. Karma — haritaki has two additional recorded actions:** deepana (kindles digestive fire) and anulomana (promotes downward movement). Amalaki's karma list in the dataset records only rasayana. This may reflect a dataset coverage gap rather than an absence in the source texts — amalaki's other karmas are well-attested in the broader tradition but are not in the current graph.
>
> #### Citation status
>
> Neither profile returns verse-level citations for most properties — provenance is recorded at the tradition/source level (Atreya/Charaka or Dhanvantari/Bhavaprakasha) without sthana, chapter, and verse numbers for the majority of triples. Verse-level detail for these two substances is not fully resolved in the current dataset.

**What to notice.** An instructive case in both directions. The agent's virya contrast carries per-source confidence and cross-tradition confirmation, and it names its own gaps (amalaki's thin karma list, missing verse detail). But the bare model captures a nuance the current extraction flattened — the *predominance* ordering within the five rasas. Structured extraction records what the extractor asked for; this is exactly the kind of refinement a domain expert reviewing the graph would catch and correct.

---

## Appendix — Setup

- **Model:** Claude Sonnet 4.6, identical in both sessions. Fresh sessions, no prior conversation, ~10% context used at start.
- **Bare session:** no tools, no system prompt beyond defaults. Questions asked verbatim.
- **Vidya session:** the vidya MCP server (an RDF triplestore with named graphs and RDF-star provenance annotations) plus a system prompt requiring that every factual claim come from graph queries, that every claim be cited, and that gaps be stated explicitly.
- **Graph contents at time of recording:** ~150 dravyas with properties (rasa, guna, veerya, vipaka, karma, dosha effects), 26 viruddha rules, 3 vipaka rules — extracted from the Kaviratna translation of Charaka Samhita Sutra Sthana chapters 26–27 with supplementary Bhavaprakasha Nighantu data. Extraction performed by Claude; unvalidated by domain experts. Every triple carries provenance (tradition, source, sthana, chapter, verse, pramana classification, confidence) precisely so that expert validation can proceed claim by claim.
