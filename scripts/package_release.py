#!/usr/bin/env python3
"""
scripts/package_release.py
Automated release packager for Fabushi (GPUI Telegram Desktop Client).

Generates in dist/:
1. bin/gpui_telegram_desktop (standalone executable)
2. Fabushi.app (primary macOS application bundle with Info.plist, PkgInfo, AppIcon.icns)
3. GPUI Telegram.app (compatibility bundle mirror for existing CI/evidence scripts)
4. Fabushi-{version}-macos-{arch}.zip (auto-update self-replacement archive)
5. Fabushi-{version}-macos-{arch}.dmg (drag-to-Applications disk image, on macOS)
6. Fabushi-{version}-macos-{arch}.pkg (click-to-install macOS installer, on macOS)
7. latest-mac.json (in-app auto-updater manifest for bhrum/gpui-telegram GitHub Releases)
8. gpui_telegram_desktop-v{version}-macos-{arch}.tar.gz + universal_or_arm64 alias
9. SHA256SUMS & RELEASE_NOTES.md

Enforces:
- AGENTS.md build hygiene via scripts/clean_build_garbage.py
"""

from __future__ import annotations

import os
import sys
import json
import shutil
import hashlib
import plistlib
import tarfile
import zipfile
import subprocess
import argparse
import platform
import tempfile
from pathlib import Path
from datetime import datetime, timezone

GITHUB_REPO = "bhrum/gpui-telegram"


def log(msg: str):
    print(f"[package_release] {msg}")


def parse_args():
    parser = argparse.ArgumentParser(description="Package Fabushi Desktop release.")
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
        default="Fabushi",
        help="Name of primary macOS Application Bundle (defaults to 'Fabushi')",
    )
    parser.add_argument(
        "--bundle-id",
        type=str,
        default="com.ombhrum.fabushi",
        help="CFBundleIdentifier for macOS Application Bundle (defaults to 'com.ombhrum.fabushi')",
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
    parser.add_argument(
        "--install-to-applications",
        action="store_true",
        help="Install the built Fabushi.app into /Applications and register with LaunchServices & Spotlight",
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
    """Installs prebuilt AppIcon.icns if present, else generates ICNS from PNG via sips/iconutil."""
    prebuilt_candidates = [
        repo_root / "crates" / "gpui_telegram_desktop" / "assets" / "AppIcon.icns",
    ]
    for cand in prebuilt_candidates:
        if cand.is_file():
            output_icns.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(cand, output_icns)
            os.chmod(output_icns, 0o644)
            log(f"Copied prebuilt ICNS to {output_icns} ({output_icns.stat().st_size} bytes)")
            return True

    src_candidates = [
        repo_root / "crates" / "gpui_telegram_desktop" / "assets" / "AppIcon-512.png",
        repo_root / "crates" / "gpui_telegram_desktop" / "assets" / "icons" / "AppIcon-512.png",
        repo_root / "crates" / "gpui_telegram_desktop" / "assets" / "app_icon.png",
        repo_root / "inherited" / "fabushi-backend" / "fabushi" / "web" / "icons" / "Icon-512.png",
        repo_root / "inherited" / "fabushi-backend" / "fabushi" / "web" / "favicon.png",
    ]
    src_icon = next((c for c in src_candidates if c.exists()), None)
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


def adhoc_codesign_bundle(app_dir: Path):
    if platform.system() != "Darwin":
        return
    if shutil.which("xattr"):
        subprocess.run(["xattr", "-cr", str(app_dir)], check=False, capture_output=True)
    if shutil.which("codesign"):
        try:
            subprocess.run(
                ["codesign", "--force", "--deep", "--sign", "-", str(app_dir)],
                check=True,
                capture_output=True,
            )
            log(f"Applied ad-hoc codesignature to {app_dir.name}")
        except Exception as e:
            log(f"WARNING: codesign ad-hoc failed for {app_dir.name}: {e}")


def build_macos_app_bundle(
    repo_root: Path,
    dist_dir: Path,
    source_bin: Path,
    binary_name: str,
    app_name: str,
    bundle_id: str,
    version: str,
) -> Path:
    app_dir = dist_dir / f"{app_name}.app"
    contents_dir = app_dir / "Contents"
    macos_dir = contents_dir / "MacOS"
    resources_dir = contents_dir / "Resources"

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
    <string>{bundle_id}</string>
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
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.social-networking</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>NSPrincipalClass</key>
    <string>NSApplication</string>
    <key>NSRequiresAquaSystemAppearance</key>
    <false/>
    <key>NSHumanReadableCopyright</key>
    <string>Copyright © 2026 Fabushi Contributors. Apache-2.0.</string>
    <key>CFBundleURLTypes</key>
    <array>
        <dict>
            <key>CFBundleTypeRole</key>
            <string>Editor</string>
            <key>CFBundleURLName</key>
            <string>{bundle_id}</string>
            <key>CFBundleURLSchemes</key>
            <array>
                <string>fabushi</string>
                <string>com.ombhrum.fabushi</string>
                <string>gpui-telegram</string>
            </array>
        </dict>
    </array>
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

    # 4. Install AppIcon.icns
    icns_path = resources_dir / "AppIcon.icns"
    generate_icns(repo_root, icns_path)

    # 4b. Bundle desktop assets/themes/icons/wallpapers into Contents/Resources/assets
    src_assets = repo_root / "crates" / "gpui_telegram_desktop" / "assets"
    if src_assets.is_dir():
        dest_assets = resources_dir / "assets"
        shutil.copytree(
            src_assets,
            dest_assets,
            dirs_exist_ok=True,
            ignore=shutil.ignore_patterns(".DS_Store", "._*", "AppIcon.icns", "AppIcon-512.png"),
        )
        log(f"Bundled desktop assets into {dest_assets}")

    # 5. Ad-hoc codesign on macOS
    adhoc_codesign_bundle(app_dir)
    log(f"Successfully constructed macOS App Bundle: {app_dir}")
    return app_dir


def mirror_compat_bundle(primary_app: Path, dist_dir: Path, compat_name: str = "GPUI Telegram") -> Path:
    """Maintains dist/GPUI Telegram.app mirror for existing CI & evidence verification scripts."""
    compat_app = dist_dir / f"{compat_name}.app"
    if compat_app == primary_app:
        return compat_app
    if compat_app.exists():
        shutil.rmtree(compat_app)
    if platform.system() == "Darwin" and shutil.which("ditto"):
        subprocess.run(["ditto", str(primary_app), str(compat_app)], check=True)
    else:
        shutil.copytree(primary_app, compat_app, symlinks=True)
    adhoc_codesign_bundle(compat_app)
    log(f"Mirrored compatibility bundle: {compat_app}")
    return compat_app


def package_standalone_bin(dist_dir: Path, source_bin: Path, binary_name: str) -> Path:
    bin_dir = dist_dir / "bin"
    bin_dir.mkdir(parents=True, exist_ok=True)
    dest_bin = bin_dir / binary_name
    shutil.copy2(source_bin, dest_bin)
    os.chmod(dest_bin, 0o755)
    log(f"Installed standalone binary: {dest_bin} ({dest_bin.stat().st_size} bytes)")
    return dest_bin


def create_app_zip(dist_dir: Path, app_dir: Path, app_name: str, version: str, arch: str) -> Path:
    slug = app_name.lower().replace(" ", "-")
    zip_name = f"{slug}-{version}-macos-{arch}.zip" if app_name != "Fabushi" else f"Fabushi-{version}-macos-{arch}.zip"
    zip_path = dist_dir / zip_name
    if zip_path.exists():
        zip_path.unlink()

    if platform.system() == "Darwin" and shutil.which("ditto"):
        subprocess.run(
            ["ditto", "-c", "-k", "--sequesterRsrc", "--keepParent", str(app_dir), str(zip_path)],
            check=True,
        )
    else:
        with zipfile.ZipFile(zip_path, "w", compression=zipfile.ZIP_DEFLATED, compresslevel=9) as zf:
            for root, _, files in os.walk(app_dir):
                for file in sorted(files):
                    if file == ".DS_Store" or file.startswith("._"):
                        continue
                    full_path = Path(root) / file
                    rel_path = full_path.relative_to(app_dir.parent)
                    info = zipfile.ZipInfo.from_file(full_path, arcname=str(rel_path))
                    if os.access(full_path, os.X_OK):
                        info.external_attr = (0o100755 & 0xFFFF) << 16
                    else:
                        info.external_attr = (0o100644 & 0xFFFF) << 16
                    with open(full_path, "rb") as fh:
                        zf.writestr(info, fh.read(), compress_type=zipfile.ZIP_DEFLATED)

    os.chmod(zip_path, 0o644)
    log(f"Created auto-update ZIP: {zip_path} ({zip_path.stat().st_size} bytes)")
    return zip_path


def create_macos_dmg(dist_dir: Path, app_dir: Path, app_name: str, version: str, arch: str) -> Path | None:
    if platform.system() != "Darwin" or not shutil.which("hdiutil"):
        log("hdiutil unavailable on this host, skipping .dmg creation.")
        return None
    dmg_path = dist_dir / f"{app_name}-{version}-macos-{arch}.dmg"
    if dmg_path.exists():
        dmg_path.unlink()

    stage_dir = Path(tempfile.mkdtemp(prefix="fabushi-dmg-"))
    try:
        staged_app = stage_dir / app_dir.name
        if shutil.which("ditto"):
            subprocess.run(["ditto", str(app_dir), str(staged_app)], check=True)
        else:
            shutil.copytree(app_dir, staged_app, symlinks=True)
        os.symlink("/Applications", stage_dir / "Applications")

        subprocess.run(
            [
                "hdiutil",
                "create",
                "-volname",
                app_name,
                "-srcfolder",
                str(stage_dir),
                "-ov",
                "-format",
                "UDZO",
                str(dmg_path),
            ],
            check=True,
            capture_output=True,
        )
        os.chmod(dmg_path, 0o644)
        log(f"Created macOS DMG installer: {dmg_path} ({dmg_path.stat().st_size} bytes)")
        return dmg_path
    finally:
        shutil.rmtree(stage_dir, ignore_errors=True)


def create_macos_pkg(
    dist_dir: Path,
    app_dir: Path,
    app_name: str,
    bundle_id: str,
    version: str,
    arch: str,
) -> Path | None:
    if platform.system() != "Darwin" or not shutil.which("pkgbuild"):
        log("pkgbuild unavailable on this host, skipping .pkg creation.")
        return None
    pkg_path = dist_dir / f"{app_name}-{version}-macos-{arch}.pkg"
    if pkg_path.exists():
        pkg_path.unlink()

    work_dir = Path(tempfile.mkdtemp(prefix="fabushi-pkg-"))
    try:
        pkg_root = work_dir / "root"
        apps_dir = pkg_root / "Applications"
        apps_dir.mkdir(parents=True, exist_ok=True)
        staged_app = apps_dir / app_dir.name
        if shutil.which("ditto"):
            subprocess.run(["ditto", "--norsrc", str(app_dir), str(staged_app)], check=True)
        else:
            shutil.copytree(app_dir, staged_app, symlinks=True)
        adhoc_codesign_bundle(staged_app)

        comp_plist = work_dir / "components.plist"
        subprocess.run(
            ["pkgbuild", "--analyze", "--root", str(pkg_root), str(comp_plist)],
            check=True,
            capture_output=True,
        )
        with open(comp_plist, "rb") as f:
            components = plistlib.load(f)
        for comp in components:
            comp["BundleIsRelocatable"] = False
            comp["BundleHasStrictIdentifier"] = True
            comp["BundleIsVersionChecked"] = False
            comp["BundleOverwriteAction"] = "upgrade"
        with open(comp_plist, "wb") as f:
            plistlib.dump(components, f)

        comp_pkg = work_dir / "component.pkg"
        pkg_env = {**os.environ, "COPYFILE_DISABLE": "1"}
        subprocess.run(
            [
                "pkgbuild",
                "--root",
                str(pkg_root),
                "--component-plist",
                str(comp_plist),
                "--identifier",
                bundle_id,
                "--version",
                version,
                "--install-location",
                "/",
                str(comp_pkg),
            ],
            check=True,
            capture_output=True,
            env=pkg_env,
        )

        if shutil.which("productbuild"):
            subprocess.run(
                [
                    "productbuild",
                    "--package",
                    str(comp_pkg),
                    str(pkg_path),
                ],
                check=True,
                capture_output=True,
                env=pkg_env,
            )
        else:
            shutil.copy2(comp_pkg, pkg_path)

        os.chmod(pkg_path, 0o644)
        log(f"Created macOS PKG click-to-install package: {pkg_path} ({pkg_path.stat().st_size} bytes)")
        return pkg_path
    finally:
        shutil.rmtree(work_dir, ignore_errors=True)


def compute_sha256(filepath: Path) -> str:
    h = hashlib.sha256()
    with open(filepath, "rb") as f:
        while chunk := f.read(65536):
            h.update(chunk)
    return h.hexdigest()


def generate_latest_mac_manifest(
    dist_dir: Path,
    version: str,
    arch: str,
    zip_path: Path,
    dmg_path: Path | None,
    pkg_path: Path | None,
) -> Path:
    manifest_path = dist_dir / "latest-mac.json"
    tag_name = f"v{version}"
    base_url = f"https://github.com/{GITHUB_REPO}/releases/download/{tag_name}"
    plat_key = "darwin-x86_64" if arch in ("x86_64", "amd64") else "darwin-arm64"

    entry = {
        "zip_url": f"{base_url}/{zip_path.name}",
        "zip_sha256": compute_sha256(zip_path),
        "zip_size": zip_path.stat().st_size,
    }
    if dmg_path and dmg_path.exists():
        entry["dmg_url"] = f"{base_url}/{dmg_path.name}"
    if pkg_path and pkg_path.exists():
        entry["pkg_url"] = f"{base_url}/{pkg_path.name}"

    manifest = {
        "version": version,
        "tag_name": tag_name,
        "published_at": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "release_notes": f"Fabushi Desktop {tag_name} (macOS {arch})",
        "platforms": {
            plat_key: entry,
            "darwin-universal": entry,
        },
    }
    with open(manifest_path, "w", encoding="utf-8") as f:
        json.dump(manifest, f, indent=2)
        f.write("\n")
    os.chmod(manifest_path, 0o644)
    log(f"Generated auto-updater manifest: {manifest_path}")
    return manifest_path


def generate_release_notes(
    dist_dir: Path,
    app_name: str,
    version: str,
    git_commit: str,
    arch: str,
) -> Path:
    notes_path = dist_dir / "RELEASE_NOTES.md"
    now_utc = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    content = f"""# {app_name} Desktop Client — Release v{version}

- **Release Timestamp**: `{now_utc}`
- **Git Commit**: `{git_commit}`
- **Target Platform**: macOS Darwin ({arch})
- **Build Mode**: Production Release (`opt-level=3`, `panic=abort`, `debug=0`, `strip=debuginfo`, `lto=thin`, `incremental=false`)

## Production Cloud Endpoints
The {app_name} desktop client connects directly to the production cloud infrastructure on Google Cloud Platform:
- **Rust Telegram Server Gateway (HTTP/WS)**: `http://34.46.77.133:8080`
- **Node.js AI Gateway (DeepSeek SSE Stream)**: `http://34.46.77.133:8788`

## Installers & Auto-Update Artifacts
- `{app_name}-{version}-macos-{arch}.pkg`: Double-click native macOS installer placing `{app_name}.app` into `/Applications`.
- `{app_name}-{version}-macos-{arch}.dmg`: Drag-and-drop macOS disk image containing `{app_name}.app` and `/Applications` shortcut.
- `{app_name}-{version}-macos-{arch}.zip`: Self-contained app bundle archive used by the in-app one-click updater.
- `latest-mac.json`: Auto-updater release manifest pointing to `https://github.com/{GITHUB_REPO}/releases`.
- `{app_name}.app/`: Standalone macOS Application Bundle (`com.ombhrum.fabushi`).
- `bin/gpui_telegram_desktop`: Standalone executable binary for CLI diagnostics.
- `SHA256SUMS`: Cryptographic digests for all deliverables.
"""
    with open(notes_path, "w", encoding="utf-8") as f:
        f.write(content)
    os.chmod(notes_path, 0o644)
    log(f"Generated release notes: {notes_path}")
    return notes_path


def _tar_clean_filter(tarinfo: tarfile.TarInfo) -> tarfile.TarInfo | None:
    base = os.path.basename(tarinfo.name)
    if base == ".DS_Store" or base.startswith("._"):
        return None
    return tarinfo


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
        tar.add(str(app_dir), arcname=app_dir.name, filter=_tar_clean_filter)
        tar.add(str(bin_dir), arcname=bin_dir.name, filter=_tar_clean_filter)
        tar.add(str(notes_file), arcname=notes_file.name, filter=_tar_clean_filter)

    os.chmod(tarball_path, 0o644)
    log(f"Created archive: {tarball_path} ({tarball_path.stat().st_size} bytes)")

    alias_name = f"gpui_telegram_desktop-v{version}-macos-universal_or_arm64.tar.gz"
    alias_path = dist_dir / alias_name
    shutil.copyfile(tarball_path, alias_path)
    os.chmod(alias_path, 0o644)
    log(f"Created release alias: {alias_path}")

    return [tarball_path, alias_path]


def generate_sha256sums(dist_dir: Path) -> Path:
    sums_file = dist_dir / "SHA256SUMS"
    lines = []

    entries = []
    bin_file = dist_dir / "bin" / "gpui_telegram_desktop"
    if bin_file.exists():
        entries.append(bin_file)

    for item in sorted(dist_dir.iterdir()):
        if item.is_file() and item.name != "SHA256SUMS" and not item.name.startswith("."):
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


def stop_stale_app_instances(app_name: str):
    """Terminates stale Fabushi/GPUI Telegram desktop app instances without touching FabushiSupervisor."""
    patterns = [
        f"/Applications/{app_name}.app/Contents/MacOS/",
        "dist/GPUI Telegram.app/Contents/MacOS/gpui_telegram_desktop",
        f"dist/{app_name}.app/Contents/MacOS/gpui_telegram_desktop",
    ]
    for pat in patterns:
        subprocess.run(["pkill", "-f", pat], check=False, capture_output=True)


def install_to_applications(app_dir: Path, app_name: str):
    if platform.system() != "Darwin":
        log("--install-to-applications is macOS-only; skipping.")
        return

    stop_stale_app_instances(app_name)
    target_app = Path("/Applications") / f"{app_name}.app"
    if target_app.exists():
        log(f"Removing previous bundle at {target_app}...")
        shutil.rmtree(target_app)

    log(f"Installing {app_dir} -> {target_app}...")
    if shutil.which("ditto"):
        subprocess.run(["ditto", str(app_dir), str(target_app)], check=True)
    else:
        shutil.copytree(app_dir, target_app, symlinks=True)

    adhoc_codesign_bundle(target_app)

    lsregister = "/System/Library/Frameworks/CoreServices.framework/Versions/A/Frameworks/LaunchServices.framework/Versions/A/Support/lsregister"
    if Path(lsregister).exists():
        subprocess.run([lsregister, "-f", str(target_app)], check=False, capture_output=True)
        log("Registered bundle in macOS LaunchServices.")
    if shutil.which("mdimport"):
        subprocess.run(["mdimport", str(target_app)], check=False, capture_output=True)
        log("Triggered Spotlight metadata indexing via mdimport.")


def main():
    args = parse_args()
    repo_root = resolve_repo_root(args.repo_root)
    target_dir = Path(args.target_dir).resolve() if args.target_dir else repo_root / "target"
    dist_dir = Path(args.dist_dir).resolve() if args.dist_dir else repo_root / "dist"

    version = args.version or extract_version_from_cargo(repo_root)
    git_commit = get_git_commit(repo_root)
    arch = platform.machine() or "arm64"

    log(f"Target repository: {repo_root}")
    log(f"App bundle name:   {args.app_name}.app ({args.bundle_id})")
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

    # Step 5: Package primary Fabushi.app + compatibility GPUI Telegram.app mirror
    app_dir = build_macos_app_bundle(
        repo_root=repo_root,
        dist_dir=dist_dir,
        source_bin=release_bin,
        binary_name=args.binary_name,
        app_name=args.app_name,
        bundle_id=args.bundle_id,
        version=version,
    )
    compat_dir = mirror_compat_bundle(app_dir, dist_dir, "GPUI Telegram")

    # Step 6: Generate Release Notes
    notes_file = generate_release_notes(dist_dir, args.app_name, version, git_commit, arch)

    # Step 7: Create ZIP, DMG, PKG & latest-mac.json
    zip_path = create_app_zip(dist_dir, app_dir, args.app_name, version, arch)
    dmg_path = create_macos_dmg(dist_dir, app_dir, args.app_name, version, arch)
    pkg_path = create_macos_pkg(dist_dir, app_dir, args.app_name, args.bundle_id, version, arch)
    generate_latest_mac_manifest(dist_dir, version, arch, zip_path, dmg_path, pkg_path)

    # Step 8: Create legacy Release Tarball Archives (preserving CI compatibility)
    create_release_tarballs(
        dist_dir=dist_dir,
        app_dir=compat_dir,
        bin_dir=dist_dir / "bin",
        notes_file=notes_file,
        version=version,
        arch=arch,
    )

    # Step 9: Compute SHA256 Checksums
    generate_sha256sums(dist_dir)

    # Step 10: Optionally install to /Applications & register with Spotlight
    if args.install_to_applications:
        install_to_applications(app_dir, args.app_name)

    # Step 11: Post-packaging hygiene cleanup
    if not args.skip_clean:
        run_clean_hygiene(repo_root)

    log("=" * 60)
    log("Release packaging completed successfully!")
    log(f"Deliverables located in: {dist_dir}")
    log("=" * 60)


if __name__ == "__main__":
    main()
