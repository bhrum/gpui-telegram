"""Verify cleanup CLI dispatch without letting any cleanup touch actual files."""
import contextlib
import importlib.util
import io
from pathlib import Path
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location(
    "cleanup", Path(__file__).with_name("clean_build_garbage.py")
)
cleanup = importlib.util.module_from_spec(spec)
spec.loader.exec_module(cleanup)


class CleanupScopeTests(unittest.TestCase):
    def invoke(self, flags):
        names = (
            "clean_redundant_workspace_targets", "clean_target_dir",
            "clean_test_sqlite_databases", "clean_tantivy_indexes",
            "clean_chunk_upload_staging",
        )
        with contextlib.ExitStack() as stack:
            mocks = [stack.enter_context(patch.object(cleanup, n, return_value=(0, 0)))
                     for n in names]
            stack.enter_context(patch("sys.argv", ["cleanup", *flags]))
            stack.enter_context(contextlib.redirect_stdout(io.StringIO()))
            stack.enter_context(contextlib.redirect_stderr(io.StringIO()))
            code = None
            try:
                cleanup.main()
            except SystemExit as exc:
                code = exc.code
            return [m.call_count for m in mocks], code

    def test_default_cleans_both_categories(self):
        self.assertEqual(self.invoke([]), ([1, 1, 1, 1, 1], None))

    def test_target_only_never_touches_fixture_cleaners(self):
        self.assertEqual(self.invoke(["--target-only"]), ([1, 1, 0, 0, 0], None))

    def test_temp_only_never_touches_compiler_artifacts(self):
        self.assertEqual(self.invoke(["--temp-only"]), ([0, 0, 1, 1, 1], None))

    def test_conflicting_flags_do_nothing(self):
        self.assertEqual(self.invoke(["--target-only", "--temp-only"]),
                         ([0, 0, 0, 0, 0], 2))


if __name__ == "__main__":
    unittest.main()
