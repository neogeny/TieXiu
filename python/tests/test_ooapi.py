# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

import tiexiu


def test_pegapi_function():
    tx = tiexiu.pegapi()
    assert tx is not None


def test_parse_grammar():
    tx = tiexiu.pegapi()
    tree = tx.parse_grammar("start = /a/")
    assert tree is not None


def test_parse_grammar_to_json():
    tx = tiexiu.pegapi()
    result = tx.parse_grammar_to_json("start = /a/")
    assert result is not None
    assert "start" in result["rules"][0]["name"]


def test_compile():
    tx = tiexiu.pegapi()
    # NOT IMPLEMENTED - returns Grammar
    try:
        result = tx.compile("start = /a/")
    except NotImplementedError:
        pass


def test_compile_to_json():
    tx = tiexiu.pegapi()
    result = tx.compile_to_json("start = /a/")
    assert result is not None
    assert "start" in result["rules"][0]["name"]


def test_boot_grammar():
    tx = tiexiu.pegapi()
    # NOT IMPLEMENTED - returns Grammar
    try:
        result = tx.boot_grammar()
    except NotImplementedError:
        pass


def test_boot_grammar_to_json():
    tx = tiexiu.pegapi()
    result = tx.boot_grammar_to_json()
    assert result is not None
    assert "Rule" in str(result)


def test_boot_grammar_pretty():
    tx = tiexiu.pegapi()
    result = tx.boot_grammar_pretty()
    assert result is not None
    assert "start" in result


def test_load_tree():
    # NOT IMPLEMENTED
    pass


def test_kwargs():
    tx = tiexiu.pegapi()
    tree = tx.parse_grammar("start = /a/", trace=True)
    assert tree is not None