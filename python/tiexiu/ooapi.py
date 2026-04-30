# Copyright (c) 2026 Juancarlo Añez (apalala@gmail.com)
# SPDX-License-Identifier: MIT OR Apache-2.0

from typing import Protocol, Optional, Any


class TieXiu(Protocol):
    """Object-oriented API for TieXiu parser generator."""

    def get(self, grammar: str) -> str:
        """Get cached grammar."""
        ...

    def get_or_compile(self, grammar: str) -> str:
        """Get cached or compile new grammar."""
        ...

    def parse_grammar(self, grammar: str, **kwargs: Any) -> Any:
        """Parse grammar string to AST."""
        ...

    def parse_grammar_to_json(self, grammar: str, **kwargs: Any) -> Any:
        """Parse grammar to JSON AST."""
        ...

    def compile(self, grammar: str, **kwargs: Any) -> Any:
        """NOT IMPLEMENTED - returns Grammar"""
        raise NotImplementedError("compile returns Grammar, not yet implemented")

    def compile_to_json(self, grammar: str, **kwargs: Any) -> Any:
        """Compile grammar to JSON."""
        ...

    def load(self, json: str, **kwargs: Any) -> Any:
        """NOT IMPLEMENTED - returns Grammar"""
        raise NotImplementedError("load returns Grammar, not yet implemented")

    def load_tree(self, json: str, **kwargs: Any) -> Any:
        """NOT IMPLEMENTED - returns Tree"""
        raise NotImplementedError("load_tree returns Tree, not yet implemented")

    def boot_grammar(self, **kwargs: Any) -> Any:
        """NOT IMPLEMENTED - returns Grammar"""
        raise NotImplementedError("boot_grammar returns Grammar, not yet implemented")

    def boot_grammar_to_json(self, **kwargs: Any) -> Any:
        """Get bootstrap grammar as JSON."""
        ...

    def boot_grammar_pretty(self, **kwargs: Any) -> str:
        """Get bootstrap grammar as pretty-printed string."""
        ...

    def load_boot(self, **kwargs: Any) -> Any:
        """NOT IMPLEMENTED - returns Grammar"""
        raise NotImplementedError("load_boot returns Grammar, not yet implemented")

    def load_boot_as_json(self, **kwargs: Any) -> Any:
        """Load bootstrap grammar as JSON."""
        ...

    def pretty(self, grammar: str, **kwargs: Any) -> str:
        """Pretty print grammar."""
        ...

    def parse(self, grammar: str, text: str, **kwargs: Any) -> Any:
        """Parse text with grammar."""
        ...

    def parse_to_json(self, grammar: str, text: str, **kwargs: Any) -> Any:
        """Parse text with grammar to JSON."""
        ...