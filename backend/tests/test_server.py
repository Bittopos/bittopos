import json
import tempfile
import unittest
from pathlib import Path

from backend import server


class ApiPersistenceTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.previous_path = server.DB_PATH
        server.DB_PATH = Path(self.temp.name) / "test.sqlite3"
        server.init_db()

    def tearDown(self):
        server.DB_PATH = self.previous_path
        self.temp.cleanup()

    def test_state_starts_with_empty_workspace(self):
        state = server.get_state()
        self.assertEqual(state["workspace"], {"content": {"instances": []}})
        self.assertEqual(state["resources"], [])
        self.assertEqual(state["revision"], 0)

    def test_resource_keeps_unknown_fields_and_toml_source(self):
        source = 'id = "r1"\ncustom = { keep = true }\n\n[info]\ntitle = "Proyecto"\ntags = ["x"]\n'
        resource = server.import_toml(source)
        state = server.get_state()
        self.assertEqual(resource["custom"], {"keep": True})
        self.assertEqual(state["resources"][0]["id"], "r1")
        self.assertEqual(server.resource_toml("r1"), source)

    def test_workspace_preserves_instance_fields_and_checks_revision(self):
        workspace = {"content": {"instances": [{"id": "i1", "source": {"id": "r1"}, "custom": {"keep": 7}}]}}
        result = server.save_workspace(workspace, 0)
        self.assertEqual(result["revision"], 1)
        self.assertEqual(server.get_state()["workspace"]["content"]["instances"][0]["custom"], {"keep": 7})
        with self.assertRaisesRegex(RuntimeError, "REVISION_CONFLICT"):
            server.save_workspace(workspace, 0)

    def test_workspace_rejects_duplicate_instance_ids(self):
        workspace = {"content": {"instances": [
            {"id": "same", "source": {"id": "r1"}},
            {"id": "same", "source": {"id": "r2"}},
        ]}}
        with self.assertRaisesRegex(ValueError, "duplicado"):
            server.save_workspace(workspace, 0)

    def test_resource_id_must_match_path(self):
        server.create_resource({"id": "r1", "info": {"title": "Uno"}})
        with self.assertRaisesRegex(ValueError, "coincidir"):
            server.save_resource("other", {"id": "r1", "info": {"title": "Uno"}})


    def test_profiles_and_projects_are_structured_and_isolated(self):
        profile = server.create_profile({"id": "profile-1", "label": "Personal"})
        other = server.create_profile({"id": "profile-2"})
        project = server.create_project(profile["id"], {
            "id": "project-1", "profile_id": profile["id"], "name": "Estudio",
            "description": "Notas"
        })
        self.assertEqual(server.list_profiles(), [profile, other])
        self.assertEqual(server.list_projects(profile["id"]), [project])
        self.assertEqual(server.list_projects(other["id"]), [])
        loaded = server.get_project(project["id"])
        self.assertEqual(loaded["project"], project)
        self.assertEqual(loaded["workspace"], server.DEFAULT_WORKSPACE)
        self.assertEqual(loaded["revision"], 0)

    def test_project_requires_existing_profile_and_matching_profile_id(self):
        with self.assertRaisesRegex(KeyError, "profile:missing"):
            server.create_project("missing", {
                "id": "project-1", "profile_id": "missing", "name": "Proyecto"
            })
        server.create_profile({"id": "profile-1"})
        with self.assertRaisesRegex(ValueError, "no coincide"):
            server.create_project("profile-1", {
                "id": "project-1", "profile_id": "profile-2", "name": "Proyecto"
            })

    def test_project_workspace_isolated_and_uses_revision_control(self):
        server.create_profile({"id": "profile-1"})
        server.create_project("profile-1", {
            "id": "project-1", "profile_id": "profile-1", "name": "Uno"
        })
        server.create_project("profile-1", {
            "id": "project-2", "profile_id": "profile-1", "name": "Dos"
        })
        workspace = {"content": {"instances": [
            {"id": "instance-1", "source": {"id": "resource-1"},
             "custom": {"retained": True}}
        ]}}
        saved = server.save_project_workspace("project-1", workspace, 0)
        self.assertEqual(saved["revision"], 1)
        self.assertEqual(server.get_project("project-1")["workspace"], workspace)
        self.assertEqual(server.get_project("project-2")["workspace"], server.DEFAULT_WORKSPACE)
        with self.assertRaisesRegex(RuntimeError, "REVISION_CONFLICT"):
            server.save_project_workspace("project-1", workspace, 0)

    def test_duplicate_profile_and_project_ids_are_rejected(self):
        server.create_profile({"id": "profile-1"})
        with self.assertRaises(FileExistsError):
            server.create_profile({"id": "profile-1"})
        project = {"id": "project-1", "profile_id": "profile-1", "name": "Uno"}
        server.create_project("profile-1", project)
        with self.assertRaises(FileExistsError):
            server.create_project("profile-1", project)


if __name__ == "__main__":
    unittest.main()
