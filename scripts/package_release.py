#!/usr/bin/env python3
"""
scripts/package_release.py
Automated release packager for GPUI Telegram Desktop Client.

Generates:
1. dist/bin/gpui_telegram_desktop (standalone executable)
2. dist/GPUI Telegram.app (macOS application bundle with Info.plist, PkgInfo, AppIcon.icns)
3. dist/gpui_telegram_desktop-v{version}-macos-{arch}.tar.gz (compressed distribution archive)
4. dist/gpui_telegram_desktop-v{version}-macos-universal_or_arm64.tar.gz (alias release archive)
5. dist/SHA256SUMS (cryptographic verification digests)
6. dist/RELEASE_NOTES.md (release manifest)

Enforces:
- AGENTS.md build hygiene via scripts/clean_build_garbage.py
"""

import os
import sys
import shutil
import hashlib
import tarfile
import subprocess
import argparse
import platform
import tempfile
from pathlib import Path
from datetime import datetime, timezone


def log(msg: str):
    print(f"[package_release] {msg}")


def parse_args():
    parser = argparse.ArgumentParser(description="Package GPUI Telegram Desktop release.")
    parser.add_argument(
        "--repo-root",
        type=str,
        default=None,
        help="Path to repo root (defaults to parent of scripts/)",
    )
    parser.add_argument(
        "--target-dir",
        type=str,
        default=None,
        help="Path to Cargo target directory (defaults to <repo_root>/target)",
    )
    parser.add_argument(
        "--dist-dir",
        type=str,
        default=None,
        help="Output distribution directory (defaults to <repo_root>/dist)",
    )
    parser.add_argument(
        "--version",
        type=str,
        default=None,
        help="Release version string (defaults to workspace version in Cargo.toml)",
    )
    parser.add_argument(
        "--app-name",
        type=str,
        default="GPUI Telegram",
        help="Name of macOS Application Bundle (defaults to 'GPUI Telegram')",
    )
    parser.add_argument(
        "--binary-name",
        type=str,
        default="gpui_telegram_desktop",
        help="Binary name (defaults to 'gpui_telegram_desktop')",
    )
    parser.add_argument(
        "--skip-clean",
        action="store_true",
        help="Skip executing clean_build_garbage.py",
    )
    parser.add_argument(
        "--build-if-missing",
        action="store_true",
        help="Attempt cargo build --release --bin gpui_telegram_desktop if binary not found",
    )
    return parser.parse_args()


def resolve_repo_root(hint_path: str = None) -> Path:
    if hint_path:
        root = Path(hint_path).resolve()
    else:
        root = Path(__file__).resolve().parent.parent
    if not (root / "Cargo.toml").exists():
        raise FileNotFoundError(f"Cargo.toml not found at repo root: {root}")
    return root


def extract_version_from_cargo(repo_root: Path) -> str:
    cargo_path = repo_root / "Cargo.toml"
    with open(cargo_path, "r", encoding="utf-8") as f:
        in_workspace_package = False
        for line in f:
            line = line.strip()
            if line.startswith("[workspace.package]"):
                in_workspace_package = True
                continue
            if in_workspace_package:
                if line.startswith("["):
                    in_workspace_package = False
                    continue
                if line.startswith("version"):
                    parts = line.split("=", 1)
                    if len(parts) == 2:
                        return parts[1].strip().strip('"').strip("'")
    return "0.1.0"


def get_git_commit(repo_root: Path) -> str:
    try:
        res = subprocess.run(
            ["git", "rev-parse", "--short", "HEAD"],
            cwd=str(repo_root),
            capture_output=True,
            text=True,
            check=True,
        )
        return res.stdout.strip()
    except Exception:
        return "unknown"


def run_clean_hygiene(repo_root: Path):
    cleaner = repo_root / "scripts" / "clean_build_garbage.py"
    if cleaner.exists():
        log("Executing AGENTS.md build hygiene cleaner...")
        res = subprocess.run([sys.executable, str(cleaner)], cwd=str(repo_root))
        if res.returncode != 0:
            log(f"WARNING: clean_build_garbage.py exited with code {res.returncode}")
    else:
        log("scripts/clean_build_garbage.py not found, skipping hygiene clean.")


def generate_icns(repo_root: Path, output_icns: Path) -> bool:
    """Generates macOS AppIcon.icns from source icon PNG using sips and iconutil."""
    src_candidates = [
        repo_root / "inherited" / "fabushi-backend" / "fabushi" / "web" / "icons" / "Icon-512.png",
        repo_root / "inherited" / "fabushi-backend" / "fabushi" / "web" / "favicon.png",
    ]
    src_icon = None
    for cand in src_candidates:
        if cand.exists():
            src_icon = cand
            break

    if not src_icon:
        log("No source icon found, skipping AppIcon.icns generation.")
        return False

    sips_bin = shutil.which("sips")
    iconutil_bin = shutil.which("iconutil")
    if not (sips_bin and iconutil_bin):
        log("sips or iconutil not available on this platform, skipping ICNS generation.")
        return False

    temp_iconset = Path(tempfile.mkdtemp(suffix=".iconset"))
    try:
        icon_specs = [
            ("icon_16x16.png", 16),
            ("icon_16x16@2x.png", 32),
            ("icon_32x32.png", 32),
            ("icon_32x32@2x.png", 64),
            ("icon_128x128.png", 128),
            ("icon_128x128@2x.png", 256),
            ("icon_256x256.png", 256),
            ("icon_256x256@2x.png", 512),
            ("icon_512x512.png", 512),
            ("icon_512x512@2x.png", 1024),
        ]
        for name, sz in icon_specs:
            dest_png = temp_iconset / name
            subprocess.run(
                [sips_bin, "-z", str(sz), str(sz), str(src_icon), "--out", str(dest_png)],
                check=True,
                capture_output=True,
            )

        output_icns.parent.mkdir(parents=True, exist_ok=True)
        subprocess.run(
            [iconutil_bin, "--convert", "icns", "--output", str(output_icns), str(temp_iconset)],
            check=True,
            capture_output=True,
        )
        log(f"Generated multi-resolution ICNS at {output_icns} ({output_icns.stat().st_size} bytes)")
        return True
    except Exception as e:
        log(f"WARNING: ICNS generation failed: {e}")
        return False
    finally:
        shutil.rmtree(temp_iconset, ignore_errors=True)


def build_macos_app_bundle(
    repo_root: Path,
    dist_dir: Path,
    source_bin: Path,
    binary_name: str,
    app_name: str,
    version: str,
    git_commit: str,
) -> Path:
    app_dir = dist_dir / f"{app_name}.app"
    contents_dir = app_dir / "Contents"
    macos_dir = contents_dir / "MacOS"
    resources_dir = contents_dir / "Resources"

    # Reset existing bundle
    if app_dir.exists():
        shutil.rmtree(app_dir)

    macos_dir.mkdir(parents=True, exist_ok=True)
    resources_dir.mkdir(parents=True, exist_ok=True)

    # 1. Copy executable binary
    dest_bin = macos_dir / binary_name
    shutil.copy2(source_bin, dest_bin)
    os.chmod(dest_bin, 0o755)

    # 2. Write Info.plist
    info_plist = contents_dir / "Info.plist"
    plist_content = f"""<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleInfoDictionaryVersion</key>
    <string>6.0</string>
    <key>CFBundleName</key>
    <string>{app_name}</string>
    <key>CFBundleDisplayName</key>
    <string>{app_name}</string>
    <key>CFBundleIdentifier</key>
    <string>org.gpui-telegram.desktop</string>
    <key>CFBundleVersion</key>
    <string>{version}</string>
    <key>CFBundleShortVersionString</key>
    <string>{version}</string>
    <key>CFBundleExecutable</key>
    <string>{binary_name}</string>
    <key>CFBundleIconFile</key>
    <string>AppIcon</string>
    <key>LSMinimumSystemVersion</key>
    <string>12.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
    <key>NSRequiresAquaSystemAppearance</key>
    <false/>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.social-networking</string>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2026 Teamwork GPUI Telegram Contributors. Apache-2.0.</string>
</dict>
</plist>
"""
    with open(info_plist, "w", encoding="utf-8") as f:
        f.write(plist_content)
    os.chmod(info_plist, 0o644)

    # 3. Write PkgInfo
    pkg_info = contents_dir / "PkgInfo"
    with open(pkg_info, "wb") as f:
        f.write(b"APPL????\n")
    os.chmod(pkg_info, 0o644)

    # 4. Generate AppIcon.icns
    icns_path = resources_dir / "AppIcon.icns"
    generate_icns(repo_root, icns_path)

    # 5. Ad-hoc codesign if on macOS
    if platform.system() == "Darwin" and shutil.which("codesign"):
        try:
            subprocess.run(
                ["codesign", "--force", "--deep", "--sign", "-", str(app_dir)],
                check=True,
                capture_output=True,
            )
            log(f"Applied ad-hoc codesignature to {app_dir.name}")
        except Exception as e:
            log(f"WARNING: codesign ad-hoc failed: {e}")

    log(f"Successfully constructed macOS App Bundle: {app_dir}")
    return app_dir


def package_standalone_bin(dist_dir: Path, source_bin: Path, binary_name: str) -> Path:
    bin_dir = dist_dir / "bin"
    bin_dir.mkdir(parents=True, exist_ok=True)
    dest_bin = bin_dir / binary_name
    shutil.copy2(source_bin, dest_bin)
    os.chmod(dest_bin, 0o755)
    log(f"Installed standalone binary: {dest_bin} ({dest_bin.stat().st_size} bytes)")
    return dest_bin


def generate_release_notes(
    dist_dir: Path,
    version: str,
    git_commit: str,
    arch: str,
) -> Path:
    notes_path = dist_dir / "RELEASE_NOTES.md"
    now_utc = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    content = f"""# GPUI Telegram Client — Release v{version}

- **Release Timestamp**: `{now_utc}`
- **Git Commit**: `{git_commit}`
- **Target Platform**: macOS Darwin ({arch})
- **Build Mode**: Production Release (`opt-level=3`, `panic=abort`, `debug=0`, `strip=debuginfo`, `lto=thin`, `incremental=false`)

## Production Cloud Endpoints
The GPUI Telegram client connects directly to the production cloud infrastructure on Google Cloud Platform:
- **Rust Telegram Server Gateway (HTTP/WS)**: `http://34.46.77.133:8080`
- **Node.js AI Gateway (DeepSeek SSE Stream)**: `http://34.46.77.133:8788`

## Configuration Options
The client supports 4-tier configuration precedence:
1. CLI Flag: `./bin/gpui_telegram_desktop --server-url http://34.46.77.133:8080`
2. Environment Variable: `export GPUI_TELEGRAM_SERVER_URL=http://34.46.77.133:8080`
3. Local Config File: `~/.config/gpui_telegram/config.json` (`{{"server_url": "http://34.46.77.133:8080"}}`)
4. Fallback Default: `http://127.0.0.1:8080`

## Package Structure
- `GPUI Telegram.app/`: Full macOS Application Bundle. Launch via Finder or `open "GPUI Telegram.app"`.
- `bin/gpui_telegram_desktop`: Standalone executable binary for command-line execution.
- `gpui_telegram_desktop-v{version}-macos-{arch}.tar.gz`: Gzip distribution archive.
- `SHA256SUMS`: Verification hashes for all deliverables.

## Verified Features
- Four-column GPU-accelerated workspace (Navigation Rail, Chat Sidebar, Conversation Stage, Context Inspector).
- Remote user authentication (`/api/v1/auth/login`, `/register`) with atomic session token persistence (`0600`).
- Streamed AI Bot dialog with real cloud DeepSeek model responses.
- Resilient network error handling and connection status indicators.

## AGENTS.md Build Hygiene Compliance
- Compiled in unified `target/` workspace without cache proliferation.
- Local debug bundles and incremental caches cleaned via `scripts/clean_build_garbage.py`.
"""
    with open(notes_path, "w", encoding="utf-8") as f:
        f.write(content)
    os.chmod(notes_path, 0o644)
    log(f"Generated release notes: {notes_path}")
    return notes_path


def create_release_tarballs(
    dist_dir: Path,
    app_dir: Path,
    bin_dir: Path,
    notes_file: Path,
    version: str,
    arch: str,
) -> list[Path]:
    tarball_name = f"gpui_telegram_desktop-v{version}-macos-{arch}.tar.gz"
    tarball_path = dist_dir / tarball_name

    log(f"Creating release archive: {tarball_path}...")
    with tarfile.open(tarball_path, "w:gz", compresslevel=9) as tar:
        # Add GPUI Telegram.app
        tar.add(str(app_dir), arcname=app_dir.name)
        # Add bin/
        tar.add(str(bin_dir), arcname=bin_dir.name)
        # Add RELEASE_NOTES.md
        tar.add(str(notes_file), arcname=notes_file.name)

    os.chmod(tarball_path, 0o644)
    log(f"Created archive: {tarball_path} ({tarball_path.stat().st_size} bytes)")

    # Also create the alias requested in dispatch
    alias_name = f"gpui_telegram_desktop-v{version}-macos-universal_or_arm64.tar.gz"
    alias_path = dist_dir / alias_name
    shutil.copyfile(tarball_path, alias_path)
    os.chmod(alias_path, 0o644)
    log(f"Created release alias: {alias_path}")

    return [tarball_path, alias_path]


def compute_sha256(filepath: Path) -> str:
    h = hashlib.sha256()
    with open(filepath, "rb") as f:
        while chunk := f.read(65536):
            h.update(chunk)
    return h.hexdigest()


def generate_sha256sums(dist_dir: Path) -> Path:
    sums_file = dist_dir / "SHA256SUMS"
    lines = []

    # Iterate over files in dist_dir (including bin/)
    entries = []
    bin_file = dist_dir / "bin" / "gpui_telegram_desktop"
    if bin_file.exists():
        entries.append(bin_file)

    for item in sorted(dist_dir.iterdir()):
        if item.is_file() and item.name != "SHA256SUMS":
            entries.append(item)

    for p in entries:
        rel_path = p.relative_to(dist_dir)
        digest = compute_sha256(p)
        lines.append(f"{digest}  {rel_path}\n")

    with open(sums_file, "w", encoding="utf-8") as f:
        f.writelines(lines)
    os.chmod(sums_file, 0o644)

    log(f"Generated SHA256 checksums at {sums_file}:")
    for line in lines:
        log(f"  {line.strip()}")
    return sums_file


def main():
    args = parse_args()
    repo_root = resolve_repo_root(args.repo_root)
    target_dir = Path(args.target_dir).resolve() if args.target_dir else repo_root / "target"
    dist_dir = Path(args.dist_dir).resolve() if args.dist_dir else repo_root / "dist"

    version = args.version or extract_version_from_cargo(repo_root)
    git_commit = get_git_commit(repo_root)
    arch = platform.machine() or "arm64"

    log(f"Target repository: {repo_root}")
    log(f"Release version:   v{version} (Commit: {git_commit})")
    log(f"Target platform:   {platform.system()} {arch}")
    log(f"Output directory:  {dist_dir}")

    # Step 1: Pre-packaging hygiene
    if not args.skip_clean:
        run_clean_hygiene(repo_root)

    # Step 2: Verify release binary exists
    release_bin = target_dir / "release" / args.binary_name
    if not release_bin.exists():
        if args.build_if_missing:
            log(f"Binary {release_bin} not found. Invoking 'cargo build --release --bin {args.binary_name}'...")
            subprocess.run(
                ["cargo", "build", "--release", "--bin", args.binary_name],
                cwd=str(repo_root),
                check=True,
            )
        else:
            log(f"ERROR: Release binary does not exist at {release_bin}.")
            log("Please run: cargo build --release --bin gpui_telegram_desktop")
            sys.exit(1)

    log(f"Verified release binary: {release_bin} ({release_bin.stat().st_size} bytes)")

    # Step 3: Prepare dist directory
    dist_dir.mkdir(parents=True, exist_ok=True)

    # Step 4: Package standalone binary
    package_standalone_bin(dist_dir, release_bin, args.binary_name)

    # Step 5: Package macOS App Bundle
    app_dir = build_macos_app_bundle(
        repo_root=repo_root,
        dist_dir=dist_dir,
        source_bin=release_bin,
        binary_name=args.binary_name,
        app_name=args.app_name,
        version=version,
        git_commit=git_commit,
    )

    # Step 6: Generate Release Notes
    notes_file = generate_release_notes(dist_dir, version, git_commit, arch)

    # Step 7: Create Release Tarball Archives
    create_release_tarballs(
        dist_dir=dist_dir,
        app_dir=app_dir,
        bin_dir=dist_dir / "bin",
        notes_file=notes_file,
        version=version,
        arch=arch,
    )

    # Step 8: Compute SHA256 Checksums
    generate_sha256sums(dist_dir)

    # Step 9: Post-packaging hygiene cleanup
    if not args.skip_clean:
        run_clean_hygiene(repo_root)

    log("=" * 60)
    log("Release packaging completed successfully!")
    log(f"Deliverables located in: {dist_dir}")
    log("=" * 60)


if __name__ == "__main__":
    main()
