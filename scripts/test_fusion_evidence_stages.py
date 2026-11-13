#!/usr/bin/env python3
"""
Focused validator and unit test suite for FUS-V001a:
  - docs/specs/fusion-parity-evidence-stages-20261108.md
  - docs/verification/fusion-20261108/capability-coverage.json
  - docs/specs/MODULE_BY_MODULE_ROADMAP.md

Verifies multi-stage capability evidence schema integrity, physical shard truth,
negative stage-promotion guards, and historical claim preservation without running Cargo.
"""

import copy
import json
import os
import re
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent
SPEC_PATH = REPO_ROOT / "docs/specs/fusion-parity-evidence-stages-20261108.md"
COVERAGE_PATH = REPO_ROOT / "docs/verification/fusion-20261108/capability-coverage.json"
ROADMAP_PATH = REPO_ROOT / "docs/specs/MODULE_BY_MODULE_ROADMAP.md"
INDEX_PATH = REPO_ROOT / "docs/specs/ledger/index.json"
SHARDS_DIR = REPO_ROOT / "docs/specs/ledger/shards"

REQUIRED_STAGES = (
    "file_mapped",
    "entrypoint_reachable",
    "behavior_tested",
    "ui_evidence_captured",
    "packaged_accepted",
)

LEGACY_CAPABILITY_KEYS = (
    "capability_id",
    "capability",
    "upstream_anchors",
    "candidate_owners",
    "ledger_shards",
    "state",
    "scope_source",
    "next_task",
)


def parse_shard_names(ledger_shards_str: str) -> list[str]:
    return [tok.strip().strip("`") for tok in ledger_shards_str.split(",") if tok.strip()]


def parse_roadmap_statuses_like_verify_ledger_closure(roadmap_content: str) -> dict[str, str]:
    """Mirrors scripts/verify_ledger_closure.py lines 392-410 exactly."""
    module_status_map: dict[str, str] = {}
    for line in roadmap_content.splitlines():
        line_str = line.strip()
        if line_str.startswith("|") and "Module " in line_str:
            parts = [p.strip() for p in line_str.split("|")]
            parts = [p for p in parts if p]
            if len(parts) >= 6:
                m = re.search(r"Module\s*0?(\d+)", parts[0])
                if m:
                    mid = f"{int(m.group(1)):02d}"
                    for cell in parts[3:]:
                        c = cell.upper()
                        if c in ("PLANNED", "IN_PROGRESS", "REVIEW", "ACCEPTED", "DONE"):
                            module_status_map[mid] = c
                            break
    for sm in re.finditer(
        r"###\s+Module\s*0?(\d+)[:\s].*?\n[\s\S]*?\*\*Status\*\*:\s*`?([A-Za-z_]+)`?",
        roadmap_content,
    ):
        mid = f"{int(sm.group(1)):02d}"
        if mid not in module_status_map:
            module_status_map[mid] = sm.group(2).strip().upper()
    return module_status_map


def validate_capability_entry(entry: dict, index_shard_map: dict[str, dict], check_disk: bool = True) -> list[str]:
    """Validates a single capability entry against FUS-V001a multi-stage evidence invariants."""
    errors: list[str] = []
    cid = entry.get("capability_id", "<unknown>")

    for k in LEGACY_CAPABILITY_KEYS:
        if k not in entry or entry[k] in (None, ""):
            errors.append(f"{cid}: missing legacy field '{k}'")

    oracle = entry.get("source_oracle")
    if not isinstance(oracle, dict):
        errors.append(f"{cid}: missing 'source_oracle' object")
    else:
        if cid.startswith("TCG-"):
            if oracle.get("kind") != "upstream_tdesktop":
                errors.append(f"{cid}: expected source_oracle.kind == 'upstream_tdesktop'")
            if oracle.get("revision") != "f985723b552b30fe1c17d377d57f44e61a15a8c4":
                errors.append(f"{cid}: unexpected upstream revision '{oracle.get('revision')}'")
        elif cid.startswith("CAP-"):
            if oracle.get("kind") != "inherited_fabushi":
                errors.append(f"{cid}: expected source_oracle.kind == 'inherited_fabushi'")
            if oracle.get("revision") != "fabushi-unified-design-20261105":
                errors.append(f"{cid}: unexpected inherited baseline '{oracle.get('revision')}'")

    stages = entry.get("evidence_stages")
    if not isinstance(stages, dict):
        errors.append(f"{cid}: missing 'evidence_stages' object")
        return errors

    for st in REQUIRED_STAGES:
        if st not in stages or not isinstance(stages[st], dict):
            errors.append(f"{cid}: missing evidence stage '{st}'")
            return errors

    fm = stages["file_mapped"]
    er = stages["entrypoint_reachable"]
    bt = stages["behavior_tested"]
    ui = stages["ui_evidence_captured"]
    pa = stages["packaged_accepted"]

    existing_shards = fm.get("existing_shards", [])
    missing_shards = fm.get("missing_shards", [])
    declared_shards = parse_shard_names(entry.get("ledger_shards", ""))

    if existing_shards + missing_shards != declared_shards and set(existing_shards) | set(missing_shards) != set(declared_shards):
        errors.append(f"{cid}: existing_shards + missing_shards do not match ledger_shards ({declared_shards})")

    if check_disk:
        for s in existing_shards:
            if not (SHARDS_DIR / s).is_file() or s not in index_shard_map:
                errors.append(f"{cid}: shard '{s}' listed in existing_shards but missing on disk/index.json")
        for s in missing_shards:
            if (SHARDS_DIR / s).is_file() or s in index_shard_map:
                errors.append(f"{cid}: shard '{s}' listed in missing_shards but exists on disk/index.json")

    if not existing_shards:
        if fm.get("status") != "missing_shard":
            errors.append(f"{cid}: file_mapped.status must be 'missing_shard' when existing_shards is empty")
    elif missing_shards:
        if fm.get("status") != "partial":
            errors.append(f"{cid}: file_mapped.status must be 'partial' when missing_shards is non-empty")
    else:
        if fm.get("status") not in ("verified", "partial", "unverified"):
            errors.append(f"{cid}: invalid file_mapped.status '{fm.get('status')}'")

    if er.get("status") == "verified" and not er.get("verified_entrypoints"):
        errors.append(f"{cid}: entrypoint_reachable.status == 'verified' requires non-empty verified_entrypoints")

    if bt.get("status") in ("partial", "verified") and not bt.get("verified_runs"):
        errors.append(f"{cid}: behavior_tested.status == '{bt.get('status')}' requires non-empty verified_runs")

    if ui.get("status") == "verified" and not ui.get("artifacts"):
        errors.append(f"{cid}: ui_evidence_captured.status == 'verified' requires non-empty artifacts")

    if pa.get("status") == "verified":
        if not pa.get("artifacts"):
            errors.append(f"{cid}: packaged_accepted.status == 'verified' requires non-empty artifacts")
        for upstream_stage in ("file_mapped", "entrypoint_reachable", "behavior_tested", "ui_evidence_captured"):
            if stages[upstream_stage].get("status") != "verified":
                errors.append(
                    f"{cid}: packaged_accepted cannot be 'verified' while {upstream_stage} is '{stages[upstream_stage].get('status')}'"
                )

    if entry.get("state") in ("accepted", "complete", "verified"):
        for st in REQUIRED_STAGES:
            if stages[st].get("status") != "verified":
                errors.append(
                    f"{cid}: capability state '{entry.get('state')}' prohibited while stage '{st}' is '{stages[st].get('status')}'"
                )

    hc = entry.get("historical_claims")
    if not isinstance(hc, dict):
        errors.append(f"{cid}: missing 'historical_claims' object")
    else:
        if hc.get("disposition") != "historical_claim_retained_not_accepted_for_fusion_20261108":
            errors.append(f"{cid}: invalid historical_claims.disposition '{hc.get('disposition')}'")

    return errors


class TestFusionEvidenceStages(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        with open(COVERAGE_PATH, "r", encoding="utf-8") as f:
            cls.coverage = json.load(f)
        with open(INDEX_PATH, "r", encoding="utf-8") as f:
            cls.index = json.load(f)
        with open(ROADMAP_PATH, "r", encoding="utf-8") as f:
            cls.roadmap = f.read()
        cls.index_shard_map = {
            f"{s['subsystem']}.json": s for s in cls.index.get("shards", [])
        }

    def test_spec_exists_and_documents_five_stages_and_v001b(self) -> None:
        self.assertTrue(SPEC_PATH.is_file(), f"Missing spec: {SPEC_PATH}")
        spec_text = SPEC_PATH.read_text(encoding="utf-8")
        for stage in REQUIRED_STAGES:
            self.assertIn(stage, spec_text)
        self.assertIn("FUS-V001b", spec_text)

    def test_all_58_capabilities_and_legacy_fields_preserved(self) -> None:
        caps = self.coverage.get("capabilities", [])
        self.assertEqual(len(caps), 58)
        expected_ids = [f"TCG-N{i:02d}" for i in range(1, 53)] + [f"CAP-0{i}" for i in range(1, 7)]
        actual_ids = [c.get("capability_id") for c in caps]
        self.assertEqual(actual_ids, expected_ids)
        for cap in caps:
            errors = validate_capability_entry(cap, self.index_shard_map, check_disk=True)
            self.assertEqual(errors, [], f"Validation errors for {cap.get('capability_id')}: {errors}")

    def test_missing_shards_and_unmapped_shards_accounted_honestly(self) -> None:
        cap_by_id = {c["capability_id"]: c for c in self.coverage["capabilities"]}
        for partial_id, missing_name in (
            ("TCG-N01", "webauthn.json"),
            ("TCG-N16", "poll.json"),
            ("TCG-N22", "overview.json"),
            ("TCG-N30", "profile.json"),
        ):
            fm = cap_by_id[partial_id]["evidence_stages"]["file_mapped"]
            self.assertEqual(fm["status"], "partial", f"{partial_id} must be partial due to {missing_name}")
            self.assertIn(missing_name, fm["missing_shards"])

        for cap_id in [f"CAP-0{i}" for i in range(1, 7)]:
            fm = cap_by_id[cap_id]["evidence_stages"]["file_mapped"]
            self.assertEqual(fm["status"], "missing_shard")
            self.assertEqual(fm["existing_shards"], [])
            self.assertEqual(fm["shard_mapped_files"], 0)

        gaps = self.coverage.get("ledger_coverage_gaps", {})
        self.assertEqual(
            sorted(gaps.get("shards_present_on_disk_but_unreferenced_in_capabilities", [])),
            ["iv.json", "passport.json", "payments.json", "statistics.json", "wallet.json"],
        )

    def test_no_capability_claims_ui_evidence_or_packaged_acceptance(self) -> None:
        self.assertEqual(self.coverage.get("status"), "incomplete")
        summary = self.coverage.get("stage_summary", {})
        self.assertEqual(summary.get("ui_evidence_captured", {}).get("verified"), 0)
        self.assertEqual(summary.get("packaged_accepted", {}).get("verified"), 0)
        computed = {st: {"verified": 0} for st in REQUIRED_STAGES}
        for cap in self.coverage["capabilities"]:
            self.assertEqual(cap["state"], "requires_behavior_audit")
            self.assertEqual(cap["evidence_stages"]["ui_evidence_captured"]["status"], "unverified")
            self.assertEqual(cap["evidence_stages"]["packaged_accepted"]["status"], "unverified")
            for st in REQUIRED_STAGES:
                val = cap["evidence_stages"][st]["status"]
                computed[st][val] = computed[st].get(val, 0) + 1
        self.assertEqual(summary, computed)

    def test_negative_stage_promotion_guards_reject_false_acceptance(self) -> None:
        sample = copy.deepcopy(self.coverage["capabilities"][0])

        # 1. Promoting capability state to 'accepted' while stages are unverified must fail.
        bad_state = copy.deepcopy(sample)
        bad_state["state"] = "accepted"
        errs = validate_capability_entry(bad_state, self.index_shard_map, check_disk=False)
        self.assertTrue(any("prohibited while stage" in e for e in errs), errs)

        # 2. Promoting packaged_accepted to 'verified' without upstream stages verified must fail.
        bad_pkg = copy.deepcopy(sample)
        bad_pkg["evidence_stages"]["packaged_accepted"] = {
            "status": "verified",
            "artifacts": ["fake.app"],
        }
        errs = validate_capability_entry(bad_pkg, self.index_shard_map, check_disk=False)
        self.assertTrue(any("packaged_accepted cannot be 'verified'" in e for e in errs), errs)

        # 3. Marking file_mapped 'verified' when a shard is missing must fail.
        bad_fm = copy.deepcopy(sample)
        bad_fm["evidence_stages"]["file_mapped"]["status"] = "verified"
        errs = validate_capability_entry(bad_fm, self.index_shard_map, check_disk=False)
        self.assertTrue(any("file_mapped.status must be 'partial'" in e for e in errs), errs)

        # 4. Marking behavior_tested 'partial' or 'verified' with empty verified_runs must fail.
        bad_bt = copy.deepcopy(sample)
        bad_bt["evidence_stages"]["behavior_tested"] = {
            "status": "verified",
            "verified_runs": [],
            "unverified_scope": "none",
        }
        errs = validate_capability_entry(bad_bt, self.index_shard_map, check_disk=False)
        self.assertTrue(any("requires non-empty verified_runs" in e for e in errs), errs)

    def test_roadmap_demotes_active_accepted_and_preserves_historical_claims(self) -> None:
        status_map = parse_roadmap_statuses_like_verify_ledger_closure(self.roadmap)
        self.assertEqual(len(status_map), 13)
        for mid in [f"{i:02d}" for i in range(1, 14)]:
            self.assertEqual(
                status_map.get(mid),
                "REVIEW",
                f"Module {mid} active status must be REVIEW, got {status_map.get(mid)}",
            )

        # Verify aggregate 100% active ACCEPTED claim is replaced while historical counts remain.
        self.assertIn("0 / 13 ACCEPTED (13 / 13 REVIEW)", self.roadmap)
        self.assertIn("3,441", self.roadmap)
        self.assertIn("1,897", self.roadmap)
        self.assertIn("1,501", self.roadmap)
        self.assertEqual(len(re.findall(r"Historical Pre-Fusion Claim", self.roadmap)), 13)


if __name__ == "__main__":
    unittest.main()
