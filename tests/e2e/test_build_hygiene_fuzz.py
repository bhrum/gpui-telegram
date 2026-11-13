#!/usr/bin/env python3
"""
tests/e2e/test_build_hygiene_fuzz.py
Integration entry point for Milestone 5 build hygiene & lifecycle adversarial fuzzing test suite.
"""

import os
import sys
import unittest

REPO_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", ".."))
if REPO_ROOT not in sys.path:
    sys.path.insert(0, REPO_ROOT)

from scripts.challenger_m5_hygiene_fuzz_tests import TestBuildHygieneFuzzSuite


def suite():
    return unittest.defaultTestLoader.loadTestsFromTestCase(TestBuildHygieneFuzzSuite)


if __name__ == "__main__":
    runner = unittest.TextTestRunner(verbosity=2)
    res = runner.run(suite())
    sys.exit(0 if res.wasSuccessful() else 1)
