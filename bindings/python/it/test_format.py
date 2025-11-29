import pytest # noqa
import md2data
import json

def test_convert_to_json():
    result = md2data.convert("# Hello world\n\nThis is **bold**.", 'json')
    parsed = json.loads(result)
    assert parsed is not None
    # TODO: Add assertions about the structure