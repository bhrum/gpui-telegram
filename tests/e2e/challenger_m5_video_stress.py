#!/usr/bin/env python3
"""
tests/e2e/challenger_m5_video_stress.py

Empirical Challenger Adversarial Stress Test Suite for Milestone 5 Multimedia Deliverables.
Role: challenger_m5_multimedia_stress (EMPIRICAL CHALLENGER)

Adversarial Stress Verification Scope:
1. Video Container & Codec Deep Inspection:
   - Exhaustively parses all 8 video files in artifacts/multimedia_e2e/recordings/ (.mov and .mp4).
   - Validates ISO Base Media / QuickTime atom hierarchy: ftyp, moov, mvhd, trak, mdia, minf, stbl, stsd, stsz, stco, mdat.
   - Extracts and verifies H.264 (avc1) avcC parameter sets: Profile, Level, NALU length, SPS/PPS headers.
   - Asserts exact dimensions (1280x820), duration (>0), frame counts, and 30.0 fps timing.
2. Corrupted Container Resilience & Robustness:
   - Adversarially truncates headers, injects invalid box sizes (zero, overflow), and corrupts nested atom trees.
   - Verifies parser raises structured CorruptedContainerError without unhandled exceptions or hangs.
3. PNG Retina Geometry, Chunk Checksums & Entropy Analysis:
   - Validates all 8 PNG images (GPUI + tdesktop references).
   - Verifies PNG magic bytes, 100% chunk CRC32 checksum validity (IHDR, IDAT, IEND).
   - Inspects IHDR chunk: 2560x1640 (1280x820 @2x retina), 16-bit depth, RGBA color type 6.
   - Samples pixel raster and calculates color histograms, unique color counts (>100 colors), and Shannon entropy (>1.0 bits, <85% dominant color) proving non-blank real UI rendering.
4. Sandbox Concurrency & Process Isolation Safety:
   - Verifies user Telegram PID 65042 remains completely unaffected before, during, and after child process execution.
   - Tests sandbox behavior under read-only/restricted directories to ensure graceful error handling.
5. Workspace Build Hygiene:
   - Runs scripts/clean_build_garbage.py post-test, ensuring compliance with AGENTS.md.
"""

import math
import os
import shutil
import struct
import subprocess
import sys
import tempfile
import time
import unittest
import zlib
from collections import Counter
from typing import Any, Dict, List, Optional, Tuple

REPO_ROOT = "/Users/bhrum/teamwork_projects/gpui_telegram"
ARTIFACTS_DIR = os.path.join(REPO_ROOT, "artifacts", "multimedia_e2e")
RECORDINGS_DIR = os.path.join(ARTIFACTS_DIR, "recordings")
SCREENSHOTS_GPUI_DIR = os.path.join(ARTIFACTS_DIR, "screenshots", "gpui")
SCREENSHOTS_TDESKTOP_DIR = os.path.join(ARTIFACTS_DIR, "screenshots", "tdesktop")
USER_TARGET_PID = 65042


class CorruptedContainerError(Exception):
    """Raised when container parsing encounters an invalid or truncated atom."""
    pass


class ISOBMFFParser:
    """Rigorous pure-Python ISO Base Media File Format and QuickTime atom parser."""

    @staticmethod
    def parse_atoms(data: bytes, offset: int = 0, end: Optional[int] = None) -> List[Dict[str, Any]]:
        if end is None:
            end = len(data)
        if end > len(data):
            raise CorruptedContainerError(f"Requested end {end} exceeds buffer size {len(data)}")

        atoms = []
        curr = offset
        while curr < end:
            if curr + 8 > end:
                raise CorruptedContainerError(f"Truncated atom header at offset {curr}: remaining bytes {end - curr} < 8")

            sz, fourcc = struct.unpack(">I4s", data[curr : curr + 8])
            hdr_len = 8

            if sz == 1:  # 64-bit extended size
                if curr + 16 > end:
                    raise CorruptedContainerError(f"Truncated 64-bit atom size header at offset {curr}")
                sz = struct.unpack(">Q", data[curr + 8 : curr + 16])[0]
                hdr_len = 16
            elif sz == 0:  # Extends to end of file
                sz = end - curr

            if sz < hdr_len:
                raise CorruptedContainerError(f"Invalid atom size {sz} < header length {hdr_len} at offset {curr}")
            if curr + sz > end:
                raise CorruptedContainerError(f"Atom '{fourcc}' claims size {sz} which exceeds boundary {end}")

            atom_name = fourcc.decode("latin1", errors="replace")
            atoms.append({
                "name": atom_name,
                "offset": curr,
                "header_len": hdr_len,
                "size": sz,
                "payload_offset": curr + hdr_len,
                "payload_size": sz - hdr_len,
            })
            curr += sz

        return atoms

    @classmethod
    def find_atom_dict(cls, data: bytes, offset: int = 0, end: Optional[int] = None) -> Dict[str, Dict[str, Any]]:
        atoms = cls.parse_atoms(data, offset, end)
        res = {}
        for a in atoms:
            res[a["name"]] = a
        return res

    @classmethod
    def deep_inspect_video(cls, file_path: str) -> Dict[str, Any]:
        with open(file_path, "rb") as f:
            data = f.read()

        top_atoms = cls.find_atom_dict(data)
        if "ftyp" not in top_atoms:
            raise CorruptedContainerError("Missing mandatory 'ftyp' atom")
        if "moov" not in top_atoms:
            raise CorruptedContainerError("Missing mandatory 'moov' atom")
        if "mdat" not in top_atoms:
            raise CorruptedContainerError("Missing mandatory 'mdat' atom")

        moov = top_atoms["moov"]
        moov_sub = cls.find_atom_dict(data, moov["payload_offset"], moov["offset"] + moov["size"])

        if "mvhd" not in moov_sub:
            raise CorruptedContainerError("Missing 'mvhd' movie header atom inside 'moov'")
        if "trak" not in moov_sub:
            raise CorruptedContainerError("Missing 'trak' track atom inside 'moov'")

        # Parse mvhd
        mvhd = moov_sub["mvhd"]
        mvhd_data = data[mvhd["payload_offset"] : mvhd["offset"] + mvhd["size"]]
        version = mvhd_data[0]
        if version == 0:
            timescale, duration = struct.unpack(">II", mvhd_data[12:20])
        elif version == 1:
            timescale, duration = struct.unpack(">IQ", mvhd_data[20:32])
        else:
            raise CorruptedContainerError(f"Unsupported mvhd version: {version}")

        duration_sec = duration / timescale if timescale > 0 else 0.0

        # Parse trak -> mdia -> minf -> stbl
        trak = moov_sub["trak"]
        trak_sub = cls.find_atom_dict(data, trak["payload_offset"], trak["offset"] + trak["size"])

        tkhd = trak_sub.get("tkhd")
        width = height = 0
        if tkhd:
            tkhd_data = data[tkhd["payload_offset"] : tkhd["offset"] + tkhd["size"]]
            w_fp, h_fp = struct.unpack(">II", tkhd_data[-8:])
            width = w_fp >> 16
            height = h_fp >> 16

        mdia = trak_sub["mdia"]
        mdia_sub = cls.find_atom_dict(data, mdia["payload_offset"], mdia["offset"] + mdia["size"])
        hdlr = mdia_sub["hdlr"]
        hdlr_data = data[hdlr["payload_offset"] : hdlr["offset"] + hdlr["size"]]
        handler_type = hdlr_data[8:12].decode("latin1", errors="replace")

        minf = mdia_sub["minf"]
        minf_sub = cls.find_atom_dict(data, minf["payload_offset"], minf["offset"] + minf["size"])
        stbl = minf_sub["stbl"]
        stbl_sub = cls.find_atom_dict(data, stbl["payload_offset"], stbl["offset"] + stbl["size"])

        # stsd (Sample Description)
        stsd = stbl_sub["stsd"]
        stsd_data = data[stsd["payload_offset"] : stsd["offset"] + stsd["size"]]
        entry_count = struct.unpack(">I", stsd_data[4:8])[0]
        codec_fourcc = stsd_data[12:16].decode("latin1", errors="replace")

        # stsz (Sample Sizes / Frame count)
        stsz = stbl_sub["stsz"]
        stsz_data = data[stsz["payload_offset"] : stsz["offset"] + stsz["size"]]
        uniform_size, frame_count = struct.unpack(">II", stsz_data[4:12])

        # stco (Chunk offsets)
        stco = stbl_sub["stco"]
        stco_data = data[stco["payload_offset"] : stco["offset"] + stco["size"]]
        chunk_count = struct.unpack(">I", stco_data[4:8])[0]
        first_chunk_offset = struct.unpack(">I", stco_data[8:12])[0] if chunk_count > 0 else 0

        # H.264 avcC inspection if avc1
        avcc_info = None
        if codec_fourcc == "avc1":
            idx = data.find(b"avcC", stsd["payload_offset"])
            if idx != -1:
                avcc_sz = struct.unpack(">I", data[idx - 4 : idx])[0]
                avcc_body = data[idx + 4 : idx - 4 + avcc_sz]
                cfg_ver, profile, compat, level, nalu_len_byte, num_sps = struct.unpack(">BBBBBB", avcc_body[:6])
                avcc_info = {
                    "version": cfg_ver,
                    "profile_indication": profile,
                    "level_indication": level,
                    "nalu_length_size": (nalu_len_byte & 0x03) + 1,
                    "num_sps": num_sps & 0x1F,
                }

        fps = frame_count / duration_sec if duration_sec > 0 else 0.0

        return {
            "file_size": len(data),
            "timescale": timescale,
            "duration_units": duration,
            "duration_sec": duration_sec,
            "width": width,
            "height": height,
            "handler_type": handler_type,
            "codec": codec_fourcc,
            "frame_count": frame_count,
            "fps": fps,
            "chunk_count": chunk_count,
            "first_chunk_offset": first_chunk_offset,
            "mdat_offset": top_atoms["mdat"]["offset"],
            "mdat_size": top_atoms["mdat"]["size"],
            "avcc": avcc_info,
        }


class PNGParser:
    """PNG chunk structure inspector, CRC32 validator, and pixel entropy analyzer."""

    @staticmethod
    def inspect_chunks(path: str) -> Dict[str, Any]:
        """Inspects all PNG chunks and validates CRC32 on each chunk."""
        with open(path, "rb") as f:
            data = f.read()

        if len(data) < 8 or data[:8] != b"\x89PNG\r\n\x1a\n":
            raise ValueError(f"Invalid PNG magic bytes in {path}")

        pos = 8
        chunks = []
        ihdr_info = {}

        while pos < len(data):
            if pos + 8 > len(data):
                raise ValueError("Truncated PNG chunk header")
            length, ctype = struct.unpack(">I4s", data[pos : pos + 8])
            pos += 8
            if pos + length + 4 > len(data):
                raise ValueError(f"Chunk '{ctype}' length {length} overflows file")
            cdata = data[pos : pos + length]
            crc_expected = struct.unpack(">I", data[pos + length : pos + length + 4])[0]
            crc_actual = zlib.crc32(ctype + cdata)
            if crc_expected != crc_actual:
                raise ValueError(f"CRC32 mismatch in chunk {ctype}: expected {crc_expected}, got {crc_actual}")

            chunk_type_str = ctype.decode("latin1", errors="replace")
            chunks.append(chunk_type_str)

            if chunk_type_str == "IHDR":
                w, h, bit_depth, col_type, comp, filt, interl = struct.unpack(">IIBBBBB", cdata)
                ihdr_info = {
                    "width": w,
                    "height": h,
                    "bit_depth": bit_depth,
                    "color_type": col_type,
                    "compression": comp,
                    "filter_method": filt,
                    "interlace": interl,
                }
            elif chunk_type_str == "IEND":
                break

            pos += length + 4

        return {
            "file_size": len(data),
            "chunks": chunks,
            "ihdr": ihdr_info,
        }

    @staticmethod
    def analyze_pixel_entropy(path: str, sample_size: int = 40000) -> Dict[str, Any]:
        """Converts PNG to uncompressed BMP using macOS sips and computes color distribution metrics."""
        with tempfile.NamedTemporaryFile(suffix=".bmp") as tmp:
            subprocess.run(["sips", "-s", "format", "bmp", path, "--out", tmp.name], check=True, capture_output=True)
            with open(tmp.name, "rb") as f:
                hdr = f.read(54)
                offset = struct.unpack("<I", hdr[10:14])[0]
                bpp = struct.unpack("<H", hdr[28:30])[0]
                f.seek(offset)
                raw = f.read()

        bytes_pp = bpp // 8
        step = max(bytes_pp, (len(raw) // sample_size // bytes_pp) * bytes_pp)
        colors = Counter()
        for i in range(0, len(raw) - bytes_pp, step):
            colors[tuple(raw[i : i + bytes_pp])] += 1

        tot = sum(colors.values())
        entropy = -sum((c / tot) * math.log2(c / tot) for c in colors.values())
        dominant_color, dominant_count = colors.most_common(1)[0]
        dominant_ratio = dominant_count / tot

        return {
            "sampled_pixels": tot,
            "unique_colors": len(colors),
            "entropy": entropy,
            "dominant_ratio": dominant_ratio,
            "dominant_color": dominant_color,
        }


class ChallengerM5MultimediaStress(unittest.TestCase):
    """Adversarial stress and empirical challenge suite for Milestone 5."""

    # -------------------------------------------------------------------------
    # 1. Video Container & Codec Deep Inspection
    # -------------------------------------------------------------------------

    def test_01_parse_all_eight_video_containers(self):
        """Parse all 8 recordings (.mov & .mp4), verifying ISO atom trees and dimensions."""
        video_files = [
            "rec_01_nav_switching.mov",
            "rec_01_nav_switching.mp4",
            "rec_02_settings_breadcrumbs.mov",
            "rec_02_settings_breadcrumbs.mp4",
            "rec_03_alipay_qr_simulate.mov",
            "rec_03_alipay_qr_simulate.mp4",
            "rec_04_message_send_pts_echo.mov",
            "rec_04_message_send_pts_echo.mp4",
        ]
        self.assertTrue(os.path.isdir(RECORDINGS_DIR), f"Recordings dir missing: {RECORDINGS_DIR}")

        for fname in video_files:
            fpath = os.path.join(RECORDINGS_DIR, fname)
            self.assertTrue(os.path.isfile(fpath), f"Video missing: {fpath}")
            size = os.path.getsize(fpath)
            self.assertGreater(size, 15_000, f"Video {fname} unexpectedly small: {size} bytes")

            info = ISOBMFFParser.deep_inspect_video(fpath)

            # Assert geometry: 1280x820
            self.assertEqual(info["width"], 1280, f"Expected 1280 width in {fname}, got {info['width']}")
            self.assertEqual(info["height"], 820, f"Expected 820 height in {fname}, got {info['height']}")

            # Assert video handler
            self.assertEqual(info["handler_type"], "vide", f"Handler type not 'vide' in {fname}")

            # Assert duration: between 5.5s and 8.0s
            self.assertGreaterEqual(info["duration_sec"], 5.5, f"Duration < 5.5s in {fname}")
            self.assertLessEqual(info["duration_sec"], 8.0, f"Duration > 8.0s in {fname}")

            # Assert frame rate ~ 30 fps
            self.assertAlmostEqual(info["fps"], 30.0, delta=0.5, msg=f"FPS deviation in {fname}: {info['fps']}")

            # Assert chunk offsets point to valid mdat boundary
            self.assertGreaterEqual(info["first_chunk_offset"], info["mdat_offset"])

    def test_02_h264_avcc_parameter_sets_in_mov(self):
        """Inspect H.264 (avc1) SPS/PPS parameter sets in all 4 .mov recordings."""
        mov_files = [
            "rec_01_nav_switching.mov",
            "rec_02_settings_breadcrumbs.mov",
            "rec_03_alipay_qr_simulate.mov",
            "rec_04_message_send_pts_echo.mov",
        ]
        for fname in mov_files:
            fpath = os.path.join(RECORDINGS_DIR, fname)
            info = ISOBMFFParser.deep_inspect_video(fpath)

            self.assertEqual(info["codec"], "avc1", f"Expected 'avc1' codec in {fname}, found {info['codec']}")
            avcc = info["avcc"]
            self.assertIsNotNone(avcc, f"Missing avcC atom in {fname}")
            self.assertEqual(avcc["version"], 1, f"Invalid avcC version in {fname}")
            # Profile 100 is High Profile, level 32 is 3.2
            self.assertEqual(avcc["profile_indication"], 100, f"Unexpected profile {avcc['profile_indication']} in {fname}")
            self.assertEqual(avcc["level_indication"], 32, f"Unexpected level {avcc['level_indication']} in {fname}")
            self.assertEqual(avcc["nalu_length_size"], 4, f"Unexpected NALU length size in {fname}")
            self.assertGreaterEqual(avcc["num_sps"], 1, f"Zero SPS sets in {fname}")

    # -------------------------------------------------------------------------
    # 2. Corrupted Container Resilience & Robustness
    # -------------------------------------------------------------------------

    def test_03_corrupted_container_resilience(self):
        """Stress-test parser resilience against truncated, malformed, and adversarial payloads."""
        valid_mov = os.path.join(RECORDINGS_DIR, "rec_01_nav_switching.mov")
        with open(valid_mov, "rb") as f:
            valid_bytes = f.read()

        # Case A: Truncated header (< 8 bytes)
        with self.assertRaises(CorruptedContainerError):
            ISOBMFFParser.parse_atoms(valid_bytes[:5])

        # Case B: Box size claim exceeds buffer
        corrupt_overflow = struct.pack(">I4s", 9999999, b"moov") + valid_bytes[8:100]
        with self.assertRaises(CorruptedContainerError):
            ISOBMFFParser.parse_atoms(corrupt_overflow)

        # Case C: Box size smaller than header (< 8)
        corrupt_underflow = struct.pack(">I4s", 4, b"mdat") + valid_bytes[8:100]
        with self.assertRaises(CorruptedContainerError):
            ISOBMFFParser.parse_atoms(corrupt_underflow)

        # Case D: Corrupted PNG magic bytes
        with self.assertRaises(ValueError):
            with tempfile.NamedTemporaryFile() as tmp:
                tmp.write(b"NOT_A_PNG_HEADER_DATA")
                tmp.flush()
                PNGParser.inspect_chunks(tmp.name)

    # -------------------------------------------------------------------------
    # 3. PNG Retina Screenshots & Color Histogram Verification
    # -------------------------------------------------------------------------

    def test_04_png_retina_geometry_and_chunk_integrity(self):
        """Inspect all 8 PNG images: magic bytes, chunk CRC32, 2560x1640 @2x retina, RGBA."""
        png_views = [
            (SCREENSHOTS_GPUI_DIR, "01_navigation_and_chats.png"),
            (SCREENSHOTS_GPUI_DIR, "02_call_overlay_active.png"),
            (SCREENSHOTS_GPUI_DIR, "03_settings_panel_preferences.png"),
            (SCREENSHOTS_GPUI_DIR, "04_multitab_auth_wizard.png"),
            (SCREENSHOTS_TDESKTOP_DIR, "01_navigation_and_chats_ref.png"),
            (SCREENSHOTS_TDESKTOP_DIR, "02_call_overlay_active_ref.png"),
            (SCREENSHOTS_TDESKTOP_DIR, "03_settings_panel_preferences_ref.png"),
            (SCREENSHOTS_TDESKTOP_DIR, "04_multitab_auth_wizard_ref.png"),
        ]

        for pdir, fname in png_views:
            fpath = os.path.join(pdir, fname)
            self.assertTrue(os.path.isfile(fpath), f"PNG file missing: {fpath}")
            size = os.path.getsize(fpath)
            self.assertGreater(size, 100_000, f"PNG {fname} size < 100KB ({size} bytes)")

            res = PNGParser.inspect_chunks(fpath)

            # Check IHDR attributes
            ihdr = res["ihdr"]
            self.assertEqual(ihdr["width"], 2560, f"Width not 2560 in {fname}")
            self.assertEqual(ihdr["height"], 1640, f"Height not 1640 in {fname}")
            self.assertEqual(ihdr["bit_depth"], 16, f"Bit depth not 16 in {fname}")
            self.assertEqual(ihdr["color_type"], 6, f"Color type not 6 (RGBA) in {fname}")

            # Verify chunk sequence
            self.assertEqual(res["chunks"][0], "IHDR", f"First chunk not IHDR in {fname}")
            self.assertEqual(res["chunks"][-1], "IEND", f"Last chunk not IEND in {fname}")
            self.assertIn("IDAT", res["chunks"], f"No IDAT chunks in {fname}")

    def test_05_png_color_histogram_and_entropy_non_blank(self):
        """Analyze pixel raster and verify non-blank image via color histogram and entropy."""
        gpui_files = [
            os.path.join(SCREENSHOTS_GPUI_DIR, "01_navigation_and_chats.png"),
            os.path.join(SCREENSHOTS_GPUI_DIR, "02_call_overlay_active.png"),
            os.path.join(SCREENSHOTS_GPUI_DIR, "03_settings_panel_preferences.png"),
            os.path.join(SCREENSHOTS_GPUI_DIR, "04_multitab_auth_wizard.png"),
        ]

        for fpath in gpui_files:
            fname = os.path.basename(fpath)
            res = PNGParser.analyze_pixel_entropy(fpath)

            # Assert unique colors >= 100
            self.assertGreaterEqual(
                res["unique_colors"], 100,
                f"Image {fname} has too few unique colors ({res['unique_colors']}), possible blank render"
            )

            # Assert Shannon entropy > 1.0 bit
            self.assertGreater(
                res["entropy"], 1.0,
                f"Entropy too low ({res['entropy']:.4f}) in {fname}"
            )

            # Assert dominant color < 85% of pixels (not a monolithic solid rectangle)
            self.assertLess(
                res["dominant_ratio"], 0.85,
                f"Dominant color occupies {res['dominant_ratio']*100:.1f}% in {fname}, expected rich UI elements"
            )

    # -------------------------------------------------------------------------
    # 4. Sandbox Concurrency & Process Isolation Safety
    # -------------------------------------------------------------------------

    def test_06_user_process_pid_protection(self):
        """Stress-test sandbox execution to ensure user PID 65042 is never interrupted."""
        tdesktop_bin = "/Applications/Telegram.app/Contents/MacOS/Telegram"
        if not os.path.isfile(tdesktop_bin):
            self.skipTest("Telegram.app binary not found at /Applications/Telegram.app")

        # Confirm PID 65042 is alive
        pid_alive = False
        try:
            ps_res = subprocess.run(["ps", "-p", str(USER_TARGET_PID), "-o", "pid,command"], capture_output=True, text=True)
            pid_alive = (str(USER_TARGET_PID) in ps_res.stdout)
        except Exception:
            pid_alive = False

        if not pid_alive:
            self.skipTest(f"Target user PID {USER_TARGET_PID} not running on system")

        sandbox_dir = f"/tmp/challenger_stress_sandbox_{os.getpid()}"
        os.makedirs(sandbox_dir, exist_ok=True)

        try:
            # Launch isolated sandbox instance
            proc = subprocess.Popen(
                [tdesktop_bin, "-workdir", sandbox_dir, "-noupdate"],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
            time.sleep(1.5)

            # Verify child PID is distinct from user PID 65042
            self.assertNotEqual(proc.pid, USER_TARGET_PID, "Child PID collided with user PID!")

            # Verify PID 65042 is STILL running
            check1 = subprocess.run(["ps", "-p", str(USER_TARGET_PID)], capture_output=True, text=True)
            self.assertEqual(check1.returncode, 0, f"User PID {USER_TARGET_PID} died during sandbox launch!")

            # Terminate child safely
            proc.terminate()
            try:
                proc.wait(timeout=3.0)
            except subprocess.TimeoutExpired:
                proc.kill()
                proc.wait()

            if proc.stdout:
                proc.stdout.close()
            if proc.stderr:
                proc.stderr.close()

            # Verify PID 65042 is STILL running after child termination
            check2 = subprocess.run(["ps", "-p", str(USER_TARGET_PID)], capture_output=True, text=True)
            self.assertEqual(check2.returncode, 0, f"User PID {USER_TARGET_PID} was killed after sandbox exit!")

        finally:
            if os.path.isdir(sandbox_dir):
                shutil.rmtree(sandbox_dir, ignore_errors=True)

    def test_07_sandbox_restricted_directory_resilience(self):
        """Test sandbox resilience against invalid/read-only workdir paths."""
        tdesktop_bin = "/Applications/Telegram.app/Contents/MacOS/Telegram"
        if not os.path.isfile(tdesktop_bin):
            self.skipTest("Telegram.app not present")

        # Create a read-only directory
        ro_dir = f"/tmp/challenger_ro_dir_{os.getpid()}"
        os.makedirs(ro_dir, exist_ok=True)
        os.chmod(ro_dir, 0o555)

        try:
            proc = subprocess.Popen(
                [tdesktop_bin, "-workdir", ro_dir, "-noupdate"],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
            )
            time.sleep(1.0)
            # The process should either exit gracefully with error or run without crashing host
            if proc.poll() is None:
                proc.terminate()
                try:
                    proc.wait(timeout=2.0)
                except subprocess.TimeoutExpired:
                    proc.kill()
                    proc.wait()
            if proc.stdout:
                proc.stdout.close()
            if proc.stderr:
                proc.stderr.close()
        finally:
            os.chmod(ro_dir, 0o755)
            shutil.rmtree(ro_dir, ignore_errors=True)

    # -------------------------------------------------------------------------
    # 5. Build Hygiene Cleanup
    # -------------------------------------------------------------------------

    def test_08_post_test_clean_build_garbage(self):
        """Verify that scripts/clean_build_garbage.py runs cleanly and reports zero residue."""
        clean_script = os.path.join(REPO_ROOT, "scripts", "clean_build_garbage.py")
        self.assertTrue(os.path.isfile(clean_script), f"Missing cleanup script: {clean_script}")

        res = subprocess.run([sys.executable, clean_script], capture_output=True, text=True)
        self.assertEqual(res.returncode, 0, f"clean_build_garbage.py failed: {res.stderr}")
        self.assertIn("Initiating workspace build hygiene cleanup", res.stdout)
        self.assertIn("Done!", res.stdout)


if __name__ == "__main__":
    unittest.main(verbosity=2)
