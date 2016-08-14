from twitch import *
import unittest

class TestApi(unittest.TestCase):

    def setUp(self):
        self.api = Api()

    def test_new_api_counter(self):
        self.assertEqual(self.api.counter, 0)

    # TODO
    # def test_make_bad_request(self):
    #    self.assertRaises(InvalidSchema, self.api.get("htp:/malfomed.unreachable"))

    def test_incr_counter(self):
        self.api.get("https://jsonplaceholder.typicode.com/posts")
        self.assertEqual(self.api.counter, 1)

class TestFunctions(unittest.TestCase):
    
    def test_forge_request_default(self):
        self.assertEqual(forge_request("test_key"), "https://EUW.api.pvp.net/observer-mode/rest/featured?api_key=test_key")

    def test_forge_request_params(self):
        self.assertEqual(forge_request("test_key", "eune", "featured"), "https://eune.api.pvp.net/observer-mode/rest/featured?api_key=test_key")

    def test_extract_nicknames_invalid_json(self):
        invalid_json = "{lifjleiu;':;:';}"
        # wtf lambda
        self.assertRaises(ValueError, lambda: extract_nicknames(invalid_json))

if __name__ == '__main__':
    unittest.main()
