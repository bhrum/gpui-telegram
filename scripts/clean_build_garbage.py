#!/usr/bin/env python3
"""
scripts/clean_build_garbage.py
Mandatory local build garbage & stale artifact cleaner.
Removes:
1. target/*/incremental caches and *.dSYM debug bundles
2. Stale test executable binaries in target/debug/deps (and their .fingerprint entries so cargo never hits missing binary/rmeta errors)
3. Reconciles target/debug/.fingerprint against target/debug/deps so any missing .rlib/.rmeta triggers a clean Cargo rebuild instead of E0463.
"""
import os
import re
import shutil
import glob
import time


def clean_target_dir(target_dir: str) -> int:
    if not os.path.isdir(target_dir):
        return 0
    freed_bytes = 0

    # 1. Remove incremental directories
    for inc in glob.glob(os.path.join(target_dir, "*", "incremental")):
        if os.path.isdir(inc):
            for dp, _, fns in os.walk(inc):
                for f in fns:
                    try:
                        freed_bytes += os.path.getsize(os.path.join(dp, f))
                    except OSError:
                        pass
            shutil.rmtree(inc, ignore_errors=True)

    for profile in ("debug", "release"):
        prof_dir = os.path.join(target_dir, profile)
        deps_dir = os.path.join(prof_dir, "deps")
        fp_dir = os.path.join(prof_dir, ".fingerprint")
        if not os.path.isdir(deps_dir):
            continue

        removed_hashes = set()

        # 2. Remove *.dSYM directories and stale test binaries
        for entry in os.listdir(deps_dir):
            full_p = os.path.join(deps_dir, entry)
            if entry.endswith(".dSYM") and os.path.isdir(full_p):
                for dp, _, fns in os.walk(full_p):
                    for f in fns:
                        try:
                            freed_bytes += os.path.getsize(os.path.join(dp, f))
                        except OSError:
                            pass
                shutil.rmtree(full_p, ignore_errors=True)
            elif os.path.isfile(full_p):
                _, ext = os.path.splitext(entry)
                if ext == "" and os.access(full_p, os.X_OK):
                    try:
                        if (time.time() - os.path.getmtime(full_p)) < 15:
                            continue
                    except OSError:
                        pass
                    m = re.match(r"^.+ -?([0-9a-f]{16})$", entry) or re.search(r"-([0-9a-f]{16})$", entry)
                    if m:
                        removed_hashes.add(m.group(1))
                    try:
                        freed_bytes += os.path.getsize(full_p)
                        os.remove(full_p)
                    except OSError:
                        pass

        # 3. Reconcile .fingerprint so Cargo never sees a fingerprint whose .rmeta/.rlib is missing in deps/
        if os.path.isdir(fp_dir):
            existing_dep_hashes = set()
            for entry in os.listdir(deps_dir):
                m = re.search(r"-([0-9a-f]{16})\.(rlib|rmeta|dylib|so|a)$", entry)
                if m:
                    existing_dep_hashes.add(m.group(1))

            for fp_entry in os.listdir(fp_dir):
                m = re.search(r"-([0-9a-f]{16})$", fp_entry)
                if not m:
                    continue
                h = m.group(1)
                fp_path = os.path.join(fp_dir, fp_entry)
                # Check for unhashed library artifacts (e.g. libfabushi_messaging_core.rlib/dylib/a)
                stem = fp_entry.rsplit("-", 1)[0].replace("-", "_")
                has_unhashed_artifact = (
                    os.path.exists(os.path.join(deps_dir, f"lib{stem}.rlib"))
                    or os.path.exists(os.path.join(deps_dir, f"lib{stem}.dylib"))
                    or os.path.exists(os.path.join(deps_dir, f"lib{stem}.a"))
                )
                try:
                    if os.path.isdir(fp_path) and (time.time() - os.path.getmtime(fp_path)) < 15:
                        continue
                except OSError:
                    pass

                # If this fingerprint was for a removed test binary or its .rlib/.rmeta was deleted earlier,
                # check if it's a build-script fingerprint or a library fingerprint missing from deps
                if not has_unhashed_artifact and (
                    h in removed_hashes
                    or (
                        h not in existing_dep_hashes
                        and not os.path.isdir(os.path.join(prof_dir, "build", fp_entry))
                    )
                ):
                    shutil.rmtree(fp_path, ignore_errors=True)

    return freed_bytes


def main():
    script_dir = os.path.dirname(os.path.abspath(__file__))
    repo_root = os.path.abspath(os.path.join(script_dir, ".."))
    target_dir = os.path.join(repo_root, "target")
    freed = clean_target_dir(target_dir)
    print(f"[clean_build_garbage] Freed {freed / (1024**3):.2f} GB from {target_dir}")


if __name__ == "__main__":
    main()
