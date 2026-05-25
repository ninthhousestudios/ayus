import json

with open("extraction/output/charaka-ss-27.json") as f:
    data = json.load(f)

n = len(data["dravyas"])

vipaka_enum = {"madhura", "amla", "katu", None}
rasa_enum = {"madhura", "amla", "lavana", "katu", "tikta", "kashaya"}
guna_enum = {"guru", "laghu", "sheeta", "ushna", "snigdha", "ruksha", "tikshna", "manda", "sthira", "sara", "mridu", "kathina", "vishada", "picchila", "shlakshna", "khara", "sukshma", "sthula", "sandra", "drava"}
veerya_enum = {"sheeta", "ushna", None}
dosha_enum = {"vata", "pitta", "kapha"}

errors = []
for d in data["dravyas"]:
    for r in d.get("rasa", []):
        if r not in rasa_enum:
            errors.append(f'{d["id"]}: invalid rasa "{r}"')
    for g in d.get("guna", []):
        if g not in guna_enum:
            errors.append(f'{d["id"]}: invalid guna "{g}"')
    if d.get("veerya") not in veerya_enum:
        errors.append(f'{d["id"]}: invalid veerya "{d.get("veerya")}"')
    if d.get("vipaka") not in vipaka_enum:
        errors.append(f'{d["id"]}: invalid vipaka "{d.get("vipaka")}"')
    for p in d.get("dosha_effects", {}).get("pacifies", []):
        if p not in dosha_enum:
            errors.append(f'{d["id"]}: invalid pacifies "{p}"')
    for a in d.get("dosha_effects", {}).get("aggravates", []):
        if a not in dosha_enum:
            errors.append(f'{d["id"]}: invalid aggravates "{a}"')
    for field in ["id", "sanskrit_name", "verses", "confidence"]:
        if field not in d:
            errors.append(f'{d.get("id", "?")} missing required field: {field}')

if errors:
    for e in errors:
        print(e)
else:
    print(f"All valid. {n} dravyas, 0 schema errors.")

# Category counts
cats = {}
for d in data["dravyas"]:
    c = d.get("category", "?")
    cats[c] = cats.get(c, 0) + 1
for c in sorted(cats):
    print(f"  {c}: {cats[c]}")
