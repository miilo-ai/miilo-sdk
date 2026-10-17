import tempfile
from pathlib import Path
import unittest
from check_architecture import violations


class DependencyRules(unittest.TestCase):
    def test_reversed_adapter_import_is_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'core.rs').write_text('use reqwest::Client;\n')
            policy = {'layers': [{'name': 'core', 'paths': ['*.rs'], 'forbid': [r'\breqwest\b']}]}
            self.assertTrue(violations(root, policy))
            (root / 'core.rs').write_text('pub trait Source {}\n')
            self.assertEqual(violations(root, policy), [])

    def test_size_exception_cannot_silently_grow(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / 'core.rs').write_text('// line\n' * 202)
            policy = {'layers': [{'name': 'core', 'paths': ['*.rs']}], 'existingSizeExceptions': {'core.rs': 201}}
            self.assertTrue(violations(root, policy))
            (root / 'core.rs').write_text('// line\n' * 201)
            self.assertEqual(violations(root, policy), [])


if __name__ == '__main__':
    unittest.main()
