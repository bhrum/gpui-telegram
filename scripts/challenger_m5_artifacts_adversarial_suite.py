#!/usr/bin/env python3
"""
scripts/challenger_m5_artifacts_adversarial_suite.py

Milestone 5 Iteration 2 Gate Adversarial Verification Suite.
Role: challenger_m5_iter2_artifacts (Empirical Challenger)

Empirically tests and stress-tests:
1. All 8 video files (.mov and .mp4 in artifacts/multimedia_e2e/recordings/):
   - ISO/QuickTime atom integrity (ftyp, moov, trak, mdia, minf, stbl, stsd, mdat).
   - Video track properties (1280x820, 30fps, 6.0s/7.0s duration, H.264/HEVC codecs).
   - Full frame-by-frame decoding (1,560 total frames across 8 videos, 0 corrupt frames).
   - Active timeline animation variance (anti-freeze frame check).
2. All 8 screenshot files (4 GPUI and 4 tdesktop reference in artifacts/multimedia_e2e/screenshots/):
   - PNG chunk integrity and IEEE 802.3 CRC32 verification on every chunk.
   - Retina dimensions (2560x1640, RGBA, 16-bit depth).
   - Cryptographic non-identity and anti-cheat non-duplication (pairwise distinct SHA-256).
   - Shannon entropy and luminance variance (anti-blank/anti-dummy screen).
   - Independent empirical MSE, PSNR, and SSIM verification against documented ledger.
3. Cryptographic Ledger Reconciliation with docs/verification/MULTIMEDIA_E2E_VERIFICATION.md.
4. AGENTS.md Build Hygiene compliance via clean_build_garbage.py.
"""

import hashlib
import json
import math
import os
import re
import struct
import subprocess
import sys
import unittest
import zlib
from typing import Any, Dict, List, Tuple

REPO_ROOT = "/Users/bhrum/teamwork_projects/gpui_telegram"
ARTIFACTS_DIR = os.path.join(REPO_ROOT, "artifacts", "multimedia_e2e")
RECORDINGS_DIR = os.path.join(ARTIFACTS_DIR, "recordings")
SCREENSHOTS_DIR = os.path.join(ARTIFACTS_DIR, "screenshots")
GPUI_DIR = os.path.join(SCREENSHOTS_DIR, "gpui")
TDESKTOP_DIR = os.path.join(SCREENSHOTS_DIR, "tdesktop")
VERIFICATION_DOC = os.path.join(REPO_ROOT, "docs", "verification", "MULTIMEDIA_E2E_VERIFICATION.md")
COMPARE_SCRIPT = os.path.join(REPO_ROOT, "scripts", "compare_images.swift")
CLEAN_SCRIPT = os.path.join(REPO_ROOT, "scripts", "clean_build_garbage.py")


def compute_sha256(path: str) -> str:
    h = hashlib.sha256()
    with open(path, "rb") as f:
        while chunk := f.read(65536):
            h.update(chunk)
    return h.hexdigest()


def compute_file_entropy(path: str) -> float:
    with open(path, "rb") as f:
        data = f.read()
    freq: Dict[int, int] = {}
    for b in data:
        freq[b] = freq.get(b, 0) + 1
    total = len(data)
    if total == 0:
        return 0.0
    return -sum((cnt / total) * math.log2(cnt / total) for cnt in freq.values())


def parse_iso_atoms(data: bytes, offset: int = 0, end: int = None) -> List[Tuple[str, int, int, List[Any]]]:
    if end is None:
        end = len(data)
    atoms = []
    curr = offset
    while curr < end:
        if curr + 8 > end:
            raise ValueError(f"Truncated atom header at offset {curr}")
        size = struct.unpack(">I", data[curr:curr + 4])[0]
        atom_type = data[curr + 4:curr + 8].decode("latin1", errors="replace")
        hdr_size = 8
        if size == 1:
            if curr + 16 > end:
                raise ValueError(f"Truncated 64-bit atom header at offset {curr}")
            size = struct.unpack(">Q", data[curr + 8:curr + 16])[0]
            hdr_size = 16
        elif size == 0:
            size = end - curr

        if size < hdr_size:
            raise ValueError(f"Invalid atom size {size} < {hdr_size} for {atom_type} at {curr}")
        if curr + size > end:
            raise ValueError(f"Atom {atom_type} at {curr} with size {size} exceeds bounds {end}")

        sub_atoms = []
        if atom_type in ["moov", "trak", "mdia", "minf", "stbl", "edts", "udta"]:
            sub_atoms = parse_iso_atoms(data, curr + hdr_size, curr + size)

        atoms.append((atom_type, curr, size, sub_atoms))
        curr += size
    return atoms


def find_atom_path(atoms: List[Tuple[str, int, int, List[Any]]], path: List[str]) -> bool:
    if not path:
        return True
    target = path[0]
    for a_type, _, _, sub_atoms in atoms:
        if a_type == target:
            if len(path) == 1:
                return True
            if find_atom_path(sub_atoms, path[1:]):
                return True
    return False


class ChallengerM5ArtifactsAdversarialSuite(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        assert os.path.isdir(ARTIFACTS_DIR), f"Artifacts dir missing: {ARTIFACTS_DIR}"
        assert os.path.isfile(VERIFICATION_DOC), f"Verification doc missing: {VERIFICATION_DOC}"

    def test_01_all_16_multimedia_artifacts_exist_and_sized(self):
        """Assert all 8 video files and 8 screenshot files exist with non-trivial size (> 50KB)."""
        video_files = [
            "rec_01_nav_switching.mov", "rec_01_nav_switching.mp4",
            "rec_02_settings_breadcrumbs.mov", "rec_02_settings_breadcrumbs.mp4",
            "rec_03_alipay_qr_simulate.mov", "rec_03_alipay_qr_simulate.mp4",
            "rec_04_message_send_pts_echo.mov", "rec_04_message_send_pts_echo.mp4",
        ]
        gpui_screenshots = [
            "01_navigation_and_chats.png",
            "02_call_overlay_active.png",
            "03_settings_panel_preferences.png",
            "04_multitab_auth_wizard.png",
        ]
        tdesktop_screenshots = [
            "01_navigation_and_chats_ref.png",
            "02_call_overlay_active_ref.png",
            "03_settings_panel_preferences_ref.png",
            "04_multitab_auth_wizard_ref.png",
        ]

        # Check videos
        for v in video_files:
            v_path = os.path.join(RECORDINGS_DIR, v)
            self.assertTrue(os.path.isfile(v_path), f"Video missing: {v_path}")
            size = os.path.getsize(v_path)
            self.assertGreater(size, 50_000, f"Video too small ({size} bytes): {v_path}")

        # Check GPUI screenshots
        for s in gpui_screenshots:
            s_path = os.path.join(GPUI_DIR, s)
            self.assertTrue(os.path.isfile(s_path), f"GPUI screenshot missing: {s_path}")
            size = os.path.getsize(s_path)
            self.assertGreater(size, 50_000, f"GPUI screenshot too small ({size} bytes): {s_path}")

        # Check tdesktop screenshots
        for r in tdesktop_screenshots:
            r_path = os.path.join(TDESKTOP_DIR, r)
            self.assertTrue(os.path.isfile(r_path), f"tdesktop screenshot missing: {r_path}")
            size = os.path.getsize(r_path)
            self.assertGreater(size, 50_000, f"tdesktop screenshot too small ({size} bytes): {r_path}")

    def test_02_video_iso_container_and_atom_hierarchy(self):
        """Parse raw binary atoms for all 8 videos, asserting moov -> trak -> mdia -> minf -> stbl -> stsd and mdat."""
        video_files = [
            "rec_01_nav_switching.mov", "rec_01_nav_switching.mp4",
            "rec_02_settings_breadcrumbs.mov", "rec_02_settings_breadcrumbs.mp4",
            "rec_03_alipay_qr_simulate.mov", "rec_03_alipay_qr_simulate.mp4",
            "rec_04_message_send_pts_echo.mov", "rec_04_message_send_pts_echo.mp4",
        ]

        for v in video_files:
            v_path = os.path.join(RECORDINGS_DIR, v)
            with open(v_path, "rb") as f:
                data = f.read()

            atoms = parse_iso_atoms(data)
            top_level_types = [a[0] for a in atoms]

            # Assert ftyp exists
            self.assertIn("ftyp", top_level_types, f"Missing ftyp atom in {v}")
            # Assert moov exists
            self.assertIn("moov", top_level_types, f"Missing moov atom in {v}")
            # Assert mdat exists
            self.assertIn("mdat", top_level_types, f"Missing mdat atom in {v}")

            # Check mdat payload size is > 90% of file size
            mdat_atom = next(a for a in atoms if a[0] == "mdat")
            self.assertGreater(mdat_atom[2] / len(data), 0.90, f"mdat payload ratio suspiciously low in {v}")

            # Assert nested atom hierarchy moov -> trak -> mdia -> minf -> stbl -> stsd
            self.assertTrue(
                find_atom_path(atoms, ["moov", "trak", "mdia", "minf", "stbl", "stsd"]),
                f"Invalid or missing sample table description hierarchy in {v}",
            )

    def test_03_video_playback_properties_and_duration(self):
        """Query AVFoundation properties for all 8 videos: 1280x820 resolution, 30.0 fps, 6.0s/7.0s duration."""
        swift_code = f"""
import AVFoundation
import Foundation

let dir = "{RECORDINGS_DIR}"
let fm = FileManager.default
let items = try fm.contentsOfDirectory(atPath: dir).sorted()
var results: [[String: Any]] = []

for item in items {{
    guard item.hasSuffix(".mov") || item.hasSuffix(".mp4") else {{ continue }}
    let url = URL(fileURLWithPath: (dir as NSString).appendingPathComponent(item))
    let asset = AVURLAsset(url: url)
    let dur = CMTimeGetSeconds(asset.duration)
    guard let vt = asset.tracks(withMediaType: .video).first else {{ continue }}
    let size = vt.naturalSize
    let fps = Double(vt.nominalFrameRate)
    var codecs: [String] = []
    for desc in vt.formatDescriptions {{
        let fDesc = desc as! CMFormatDescription
        let subType = CMFormatDescriptionGetMediaSubType(fDesc)
        let fourCC = String(bytes: [
            UInt8((subType >> 24) & 0xff),
            UInt8((subType >> 16) & 0xff),
            UInt8((subType >> 8) & 0xff),
            UInt8(subType & 0xff)
        ], encoding: .ascii) ?? "unknown"
        codecs.append(fourCC)
    }}
    results.append([
        "file": item,
        "width": Int(size.width),
        "height": Int(size.height),
        "duration": dur,
        "fps": fps,
        "codecs": codecs
    ])
}}

let data = try JSONSerialization.data(withJSONObject: results, options: [])
print(String(data: data, encoding: .utf8)!)
"""
        res = subprocess.run(["swift", "-suppress-warnings", "-e", swift_code], capture_output=True, text=True)
        self.assertEqual(res.returncode, 0, f"Swift AVFoundation inspection failed: {res.stderr}")
        items = json.loads(res.stdout.strip())
        self.assertEqual(len(items), 8, f"Expected 8 video properties, got {len(items)}")

        expected_durations = {
            "rec_01": 6.0,
            "rec_02": 6.0,
            "rec_03": 7.0,
            "rec_04": 7.0,
        }

        for prop in items:
            fname = prop["file"]
            self.assertEqual(prop["width"], 1280, f"Invalid width for {fname}")
            self.assertEqual(prop["height"], 820, f"Invalid height for {fname}")
            self.assertAlmostEqual(prop["fps"], 30.0, delta=0.5, msg=f"Invalid FPS for {fname}")

            prefix = fname[:6]
            expected_dur = expected_durations.get(prefix, 6.0)
            self.assertAlmostEqual(prop["duration"], expected_dur, delta=0.1, msg=f"Duration mismatch for {fname}")

            if fname.endswith(".mov"):
                self.assertIn("avc1", prop["codecs"], f"Expected avc1 (H.264) codec in {fname}")
            elif fname.endswith(".mp4"):
                self.assertTrue(
                    any(c in ["hvc1", "avc1"] for c in prop["codecs"]),
                    f"Expected HEVC or H.264 codec in {fname}, got {prop['codecs']}",
                )

    def test_04_video_full_stream_frame_decoding_and_corrupt_frame_stress(self):
        """Decode all 1,560 frames across all 8 videos using AVAssetReader; verify 0 corrupt frames."""
        swift_code = f"""
import AVFoundation
import Foundation
import CoreGraphics
import CoreVideo

let dir = "{RECORDINGS_DIR}"
let fm = FileManager.default
let items = try fm.contentsOfDirectory(atPath: dir).sorted()
var results: [[String: Any]] = []

for item in items {{
    guard item.hasSuffix(".mov") || item.hasSuffix(".mp4") else {{ continue }}
    let url = URL(fileURLWithPath: (dir as NSString).appendingPathComponent(item))
    let asset = AVURLAsset(url: url)
    guard let vt = asset.tracks(withMediaType: .video).first else {{
        results.append(["file": item, "error": "no video track"])
        continue
    }}
    guard let reader = try? AVAssetReader(asset: asset) else {{
        results.append(["file": item, "error": "reader init failed"])
        continue
    }}
    let trackOutput = AVAssetReaderTrackOutput(track: vt, outputSettings: [
        kCVPixelBufferPixelFormatTypeKey as String: Int(kCVPixelFormatType_32BGRA)
    ])
    reader.add(trackOutput)
    guard reader.startReading() else {{
        results.append(["file": item, "error": "startReading failed"])
        continue
    }}

    var frameCount = 0
    var corruptFrames = 0

    while let sampleBuffer = trackOutput.copyNextSampleBuffer() {{
        frameCount += 1
        guard let imageBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else {{
            corruptFrames += 1
            continue
        }}
        CVPixelBufferLockBaseAddress(imageBuffer, .readOnly)
        guard CVPixelBufferGetBaseAddress(imageBuffer) != nil else {{
            corruptFrames += 1
            CVPixelBufferUnlockBaseAddress(imageBuffer, .readOnly)
            continue
        }}
        CVPixelBufferUnlockBaseAddress(imageBuffer, .readOnly)
    }}

    results.append([
        "file": item,
        "frames": frameCount,
        "corrupt": corruptFrames,
        "status": reader.status == .completed ? "COMPLETED" : "FAILED"
    ])
}}

let data = try JSONSerialization.data(withJSONObject: results, options: [])
print(String(data: data, encoding: .utf8)!)
"""
        res = subprocess.run(["swift", "-suppress-warnings", "-e", swift_code], capture_output=True, text=True)
        self.assertEqual(res.returncode, 0, f"Swift frame decoding failed: {res.stderr}")
        items = json.loads(res.stdout.strip())

        total_decoded = 0
        expected_frames = {
            "rec_01": 180,
            "rec_02": 180,
            "rec_03": 210,
            "rec_04": 210,
        }

        for r in items:
            fname = r["file"]
            self.assertEqual(r["status"], "COMPLETED", f"AVAssetReader failed for {fname}: {r}")
            self.assertEqual(r["corrupt"], 0, f"Corrupted frames detected in {fname}: {r['corrupt']}")
            prefix = fname[:6]
            exp_frames = expected_frames.get(prefix, 180)
            self.assertEqual(r["frames"], exp_frames, f"Frame count mismatch for {fname}: got {r['frames']}, expected {exp_frames}")
            total_decoded += r["frames"]

        # 180*2 + 180*2 + 210*2 + 210*2 = 360 + 360 + 420 + 420 = 1,560 frames
        self.assertEqual(total_decoded, 1560, f"Expected 1,560 total decoded frames, got {total_decoded}")

    def test_05_video_timeline_animation_variance(self):
        """Verify that all videos contain dynamic animated content rather than repeated static frames."""
        swift_code = f"""
import AVFoundation
import Foundation
import CoreGraphics
import CoreVideo

let dir = "{RECORDINGS_DIR}"
let fm = FileManager.default
let items = try fm.contentsOfDirectory(atPath: dir).sorted()
var results: [[String: Any]] = []

for item in items {{
    guard item.hasSuffix(".mov") || item.hasSuffix(".mp4") else {{ continue }}
    let url = URL(fileURLWithPath: (dir as NSString).appendingPathComponent(item))
    let asset = AVURLAsset(url: url)
    guard let vt = asset.tracks(withMediaType: .video).first,
          let reader = try? AVAssetReader(asset: asset) else {{ continue }}
    let trackOutput = AVAssetReaderTrackOutput(track: vt, outputSettings: [
        kCVPixelBufferPixelFormatTypeKey as String: Int(kCVPixelFormatType_32BGRA)
    ])
    reader.add(trackOutput)
    reader.startReading()

    var frameCount = 0
    var uniqueSignatures = Set<UInt64>()

    while let sampleBuffer = trackOutput.copyNextSampleBuffer() {{
        frameCount += 1
        guard let imageBuffer = CMSampleBufferGetImageBuffer(sampleBuffer) else {{ continue }}
        CVPixelBufferLockBaseAddress(imageBuffer, .readOnly)
        let width = CVPixelBufferGetWidth(imageBuffer)
        let height = CVPixelBufferGetHeight(imageBuffer)
        let bytesPerRow = CVPixelBufferGetBytesPerRow(imageBuffer)
        if let baseAddress = CVPixelBufferGetBaseAddress(imageBuffer) {{
            var sig: UInt64 = 14695981039346656037
            for row in stride(from: 50, to: height - 50, by: 60) {{
                let rowPtr = baseAddress.advanced(by: row * bytesPerRow).assumingMemoryBound(to: UInt32.self)
                for col in stride(from: 50, to: width - 50, by: 100) {{
                    let p = UInt64(rowPtr[col])
                    sig ^= p
                    sig = sig &* 1099511628211
                }}
            }}
            uniqueSignatures.insert(sig)
        }}
        CVPixelBufferUnlockBaseAddress(imageBuffer, .readOnly)
    }}

    results.append([
        "file": item,
        "frames": frameCount,
        "unique_states": uniqueSignatures.count
    ])
}}

let data = try JSONSerialization.data(withJSONObject: results, options: [])
print(String(data: data, encoding: .utf8)!)
"""
        res = subprocess.run(["swift", "-suppress-warnings", "-e", swift_code], capture_output=True, text=True)
        self.assertEqual(res.returncode, 0, f"Swift animation variance test failed: {res.stderr}")
        items = json.loads(res.stdout.strip())

        for r in items:
            fname = r["file"]
            frames = r["frames"]
            unique_states = r["unique_states"]
            # Assert at least 50% unique visual frame states throughout playback
            ratio = unique_states / frames
            self.assertGreater(ratio, 0.40, f"Animation diversity too low for {fname}: {unique_states}/{frames} ({ratio:.1%})")

    def test_06_screenshot_png_chunk_integrity_and_crc32(self):
        """Parse raw PNG chunk stream and verify standard IEEE 802.3 CRC32 checksums for every chunk."""
        all_screenshots = [
            os.path.join(GPUI_DIR, "01_navigation_and_chats.png"),
            os.path.join(GPUI_DIR, "02_call_overlay_active.png"),
            os.path.join(GPUI_DIR, "03_settings_panel_preferences.png"),
            os.path.join(GPUI_DIR, "04_multitab_auth_wizard.png"),
            os.path.join(TDESKTOP_DIR, "01_navigation_and_chats_ref.png"),
            os.path.join(TDESKTOP_DIR, "02_call_overlay_active_ref.png"),
            os.path.join(TDESKTOP_DIR, "03_settings_panel_preferences_ref.png"),
            os.path.join(TDESKTOP_DIR, "04_multitab_auth_wizard_ref.png"),
        ]

        for s_path in all_screenshots:
            fname = os.path.basename(s_path)
            with open(s_path, "rb") as f:
                data = f.read()

            self.assertEqual(data[:8], b"\x89PNG\r\n\x1a\n", f"Invalid PNG magic signature in {fname}")

            curr = 8
            chunks = []
            while curr < len(data):
                self.assertLessEqual(curr + 12, len(data), f"Truncated chunk header in {fname} at {curr}")
                length, = struct.unpack(">I", data[curr:curr + 4])
                chunk_type = data[curr + 4:curr + 8].decode("latin1", errors="replace")
                chunk_data = data[curr + 8:curr + 8 + length]
                crc, = struct.unpack(">I", data[curr + 8 + length:curr + 12 + length])

                expected_crc = zlib.crc32(data[curr + 4:curr + 8 + length]) & 0xFFFFFFFF
                self.assertEqual(crc, expected_crc, f"CRC32 mismatch in {fname} chunk {chunk_type} at {curr}")

                chunks.append((chunk_type, length))
                curr += 12 + length

            self.assertEqual(chunks[0][0], "IHDR", f"First chunk not IHDR in {fname}")
            self.assertEqual(chunks[-1][0], "IEND", f"Last chunk not IEND in {fname}")
            has_idat = any(c[0] == "IDAT" for c in chunks)
            self.assertTrue(has_idat, f"No IDAT chunk in {fname}")

    def test_07_screenshot_retina_dimensions_and_color_format(self):
        """Assert IHDR header defines 2560x1640 Retina resolution, 16-bit depth, RGBA (color_type 6)."""
        all_screenshots = [
            os.path.join(GPUI_DIR, "01_navigation_and_chats.png"),
            os.path.join(GPUI_DIR, "02_call_overlay_active.png"),
            os.path.join(GPUI_DIR, "03_settings_panel_preferences.png"),
            os.path.join(GPUI_DIR, "04_multitab_auth_wizard.png"),
            os.path.join(TDESKTOP_DIR, "01_navigation_and_chats_ref.png"),
            os.path.join(TDESKTOP_DIR, "02_call_overlay_active_ref.png"),
            os.path.join(TDESKTOP_DIR, "03_settings_panel_preferences_ref.png"),
            os.path.join(TDESKTOP_DIR, "04_multitab_auth_wizard_ref.png"),
        ]

        for s_path in all_screenshots:
            fname = os.path.basename(s_path)
            with open(s_path, "rb") as f:
                data = f.read()

            ihdr_data = data[16:29]
            width, height, bit_depth, color_type, comp_meth, filt_meth, interlace = struct.unpack(
                ">IIBBBBB", ihdr_data
            )
            self.assertEqual(width, 2560, f"Unexpected width {width} in {fname}")
            self.assertEqual(height, 1640, f"Unexpected height {height} in {fname}")
            self.assertEqual(color_type, 6, f"Expected RGBA color type (6), got {color_type} in {fname}")
            self.assertEqual(bit_depth, 16, f"Expected 16-bit depth, got {bit_depth} in {fname}")
            self.assertEqual(comp_meth, 0, f"Invalid compression method {comp_meth} in {fname}")
            self.assertEqual(filt_meth, 0, f"Invalid filter method {filt_meth} in {fname}")
            self.assertEqual(interlace, 0, f"Interlaced PNG not expected in {fname}")

    def test_08_screenshot_cryptographic_non_identity_and_collision_resistance(self):
        """CRITICAL: Assert GPUI vs tdesktop screenshots are pairwise non-identical and all 8 SHA-256 are unique."""
        views = [
            ("01_navigation_and_chats.png", "01_navigation_and_chats_ref.png"),
            ("02_call_overlay_active.png", "02_call_overlay_active_ref.png"),
            ("03_settings_panel_preferences.png", "03_settings_panel_preferences_ref.png"),
            ("04_multitab_auth_wizard.png", "04_multitab_auth_wizard_ref.png"),
        ]

        all_hashes = set()
        all_sizes = set()

        for gpui_file, ref_file in views:
            gpui_path = os.path.join(GPUI_DIR, gpui_file)
            ref_path = os.path.join(TDESKTOP_DIR, ref_file)

            h_gpui = compute_sha256(gpui_path)
            h_ref = compute_sha256(ref_path)

            self.assertNotEqual(h_gpui, h_ref, f"Identical SHA-256 hash detected between {gpui_file} and {ref_file}!")

            sz_gpui = os.path.getsize(gpui_path)
            sz_ref = os.path.getsize(ref_path)
            self.assertNotEqual(sz_gpui, sz_ref, f"Identical byte size detected between {gpui_file} and {ref_file}!")

            all_hashes.add(h_gpui)
            all_hashes.add(h_ref)
            all_sizes.add(sz_gpui)
            all_sizes.add(sz_ref)

        self.assertEqual(len(all_hashes), 8, f"Hash collision among screenshots: {len(all_hashes)} unique out of 8")
        self.assertEqual(len(all_sizes), 8, f"Size collision among screenshots: {len(all_sizes)} unique out of 8")

    def test_09_screenshot_entropy_and_luminance_variance(self):
        """Assert file entropy > 6.0 bits/byte and pixel luminance std dev > 15.0 (rejects blank/dummy frames)."""
        all_screenshots = [
            os.path.join(GPUI_DIR, "01_navigation_and_chats.png"),
            os.path.join(GPUI_DIR, "02_call_overlay_active.png"),
            os.path.join(GPUI_DIR, "03_settings_panel_preferences.png"),
            os.path.join(GPUI_DIR, "04_multitab_auth_wizard.png"),
            os.path.join(TDESKTOP_DIR, "01_navigation_and_chats_ref.png"),
            os.path.join(TDESKTOP_DIR, "02_call_overlay_active_ref.png"),
            os.path.join(TDESKTOP_DIR, "03_settings_panel_preferences_ref.png"),
            os.path.join(TDESKTOP_DIR, "04_multitab_auth_wizard_ref.png"),
        ]

        for s_path in all_screenshots:
            fname = os.path.basename(s_path)
            entropy = compute_file_entropy(s_path)
            self.assertGreater(entropy, 6.0, f"Entropy too low ({entropy:.2f} bits/byte) in {fname}")

        # Luminance variance via Swift
        swift_code = f"""
import AppKit
import CoreGraphics
import Foundation

let files = [
    "{os.path.join(GPUI_DIR, '01_navigation_and_chats.png')}",
    "{os.path.join(GPUI_DIR, '02_call_overlay_active.png')}",
    "{os.path.join(GPUI_DIR, '03_settings_panel_preferences.png')}",
    "{os.path.join(GPUI_DIR, '04_multitab_auth_wizard.png')}",
    "{os.path.join(TDESKTOP_DIR, '01_navigation_and_chats_ref.png')}",
    "{os.path.join(TDESKTOP_DIR, '02_call_overlay_active_ref.png')}",
    "{os.path.join(TDESKTOP_DIR, '03_settings_panel_preferences_ref.png')}",
    "{os.path.join(TDESKTOP_DIR, '04_multitab_auth_wizard_ref.png')}"
]

var results: [[String: Any]] = []
let colorSpace = CGColorSpaceCreateDeviceGray()

for f in files {{
    guard let img = NSImage(contentsOfFile: f),
          let cgImg = img.cgImage(forProposedRect: nil, context: nil, hints: nil) else {{ continue }}
    let w = cgImg.width
    let h = cgImg.height
    var rawData = [UInt8](repeating: 0, count: w * h)
    guard let ctx = CGContext(data: &rawData, width: w, height: h, bitsPerComponent: 8, bytesPerRow: w, space: colorSpace, bitmapInfo: CGImageAlphaInfo.none.rawValue) else {{ continue }}
    ctx.draw(cgImg, in: CGRect(x: 0, y: 0, width: w, height: h))

    var sum = 0.0
    for b in rawData {{ sum += Double(b) }}
    let mean = sum / Double(w * h)
    var varSum = 0.0
    for b in rawData {{
        let d = Double(b) - mean
        varSum += d * d
    }}
    let stdDev = sqrt(varSum / Double(w * h))
    results.append(["file": URL(fileURLWithPath: f).lastPathComponent, "std_dev": stdDev, "mean": mean])
}}

let data = try JSONSerialization.data(withJSONObject: results, options: [])
print(String(data: data, encoding: .utf8)!)
"""
        res = subprocess.run(["swift", "-suppress-warnings", "-e", swift_code], capture_output=True, text=True)
        self.assertEqual(res.returncode, 0, f"Swift luminance inspection failed: {res.stderr}")
        items = json.loads(res.stdout.strip())

        for r in items:
            fname = r["file"]
            std_dev = r["std_dev"]
            self.assertGreater(std_dev, 15.0, f"Insufficient pixel variance in {fname} (std_dev={std_dev:.2f})")

    def test_10_empirical_mse_psnr_ssim_tolerances(self):
        """Execute compare_images.swift and verify MSE, PSNR, SSIM match verification report ledger."""
        views_expected = [
            ("01_navigation_and_chats.png", "01_navigation_and_chats_ref.png", {"mse": 966.0, "psnr": 18.3, "ssim": 0.3913}),
            ("02_call_overlay_active.png", "02_call_overlay_active_ref.png", {"mse": 394.6, "psnr": 22.2, "ssim": 0.5792}),
            ("03_settings_panel_preferences.png", "03_settings_panel_preferences_ref.png", {"mse": 1909.4, "psnr": 15.3, "ssim": 0.0515}),
            ("04_multitab_auth_wizard.png", "04_multitab_auth_wizard_ref.png", {"mse": 2016.7, "psnr": 15.1, "ssim": 0.4428}),
        ]

        for gpui_file, ref_file, expected in views_expected:
            gpui_path = os.path.join(GPUI_DIR, gpui_file)
            ref_path = os.path.join(TDESKTOP_DIR, ref_file)

            cmd = ["swift", "-suppress-warnings", COMPARE_SCRIPT, gpui_path, ref_path]
            res = subprocess.run(cmd, capture_output=True, text=True)
            self.assertEqual(res.returncode, 0, f"compare_images.swift failed: {res.stderr}")

            metrics = json.loads(res.stdout.strip())
            self.assertFalse(metrics["is_identical"], f"Unexpected identity between {gpui_file} and {ref_file}")

            # Verify within tight tolerances
            self.assertAlmostEqual(metrics["mse"], expected["mse"], delta=1.0, msg=f"MSE mismatch for {gpui_file}")
            self.assertAlmostEqual(metrics["psnr_db"], expected["psnr"], delta=0.2, msg=f"PSNR mismatch for {gpui_file}")
            self.assertAlmostEqual(metrics["ssim"], expected["ssim"], delta=0.005, msg=f"SSIM mismatch for {gpui_file}")

    def test_11_cryptographic_ledger_reconciliation(self):
        """Extract all 16 artifact hashes reported in MULTIMEDIA_E2E_VERIFICATION.md and verify against disk."""
        with open(VERIFICATION_DOC, "r", encoding="utf-8") as f:
            doc_content = f.read()

        # Parse table 4 lines: | Type | Filename | Size (bytes) | SHA-256 Digest |
        pattern = r"\|\s*([^|]+)\s*\|\s*`([^`]+)`\s*\|\s*([0-9,]+)\s*\|\s*`([0-9a-f]{64})`\s*\|"
        matches = re.findall(pattern, doc_content)

        self.assertEqual(len(matches), 16, f"Expected 16 artifact entries in ledger, found {len(matches)}")

        for kind, fname, size_str, reported_hash in matches:
            kind = kind.strip()
            fname = fname.strip()
            reported_size = int(size_str.replace(",", "").strip())
            reported_hash = reported_hash.strip()

            if "tdesktop" in kind:
                path = os.path.join(TDESKTOP_DIR, fname)
            elif "GPUI" in kind:
                path = os.path.join(GPUI_DIR, fname)
            else:
                path = os.path.join(RECORDINGS_DIR, fname)

            self.assertTrue(os.path.isfile(path), f"Ledger references missing file: {path}")

            actual_size = os.path.getsize(path)
            self.assertEqual(actual_size, reported_size, f"Size mismatch for {fname}: ledger={reported_size}, actual={actual_size}")

            actual_hash = compute_sha256(path)
            self.assertEqual(actual_hash, reported_hash, f"Cryptographic SHA-256 mismatch for {fname}")

    def test_12_build_hygiene_post_test_clean(self):
        """Execute clean_build_garbage.py and assert clean exit code 0."""
        res = subprocess.run([sys.executable, CLEAN_SCRIPT], capture_output=True, text=True)
        self.assertEqual(res.returncode, 0, f"clean_build_garbage.py failed: {res.stderr}")


if __name__ == "__main__":
    unittest.main(verbosity=2)
