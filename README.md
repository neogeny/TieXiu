[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/neogeny/TieXiu?utm_source=badge)

# 铁修 TieXiu

A high-performance port of **TatSu** to Rust.

**TieXiu** (铁修) is a PEG (Parsing Expression Grammar) engine that implements the flexibility and power of the original **[TatSu][] ** lineage into a memory-safe, high-concurrency architecture optimized for modern CPU caches.

[TatSu]: https://tatsu.readthedocs.io/en/stable/

**TieXiu** is a tool that takes grammars in extended `EBNF`_ as input, and
outputs `memoizing`_ (`Packrat`_) `PEG`_ parsers as a Rust model. The classic
variations of EBNF_ (Tomassetti, EasyExtend, Wirth) and `ISO EBNF`_ are
supported as input grammar formats.

The [TatSu Documentation][] provides a vision of where the **TieXiu** project is heading. A copy of the grammar syntax can can be accessed locally in the [SYNTAX](SYNTAX.md) document.

[TatSu Documentation]: https://tatsu.readthedocs.io/

**TieXiu** is foremost a Rust library that is also published as a Python library with the help of PyO3/Maturin. The Rust API may return objects of types in the internal parser or tree model. The Python API has strings as input and `json.dumps()` compatible Python objects as output.

**TatSu** is a mature project with an important user base so it's difficult to make certain changes even if they are improvements or fixes for long-standing quirks (as well known within experienced software engineers, a long-lived quirk becomes a feature). **TieXiu** is an opportunity to start from scratch, with a modern approach, even if the grammar syntax and its semantics are preserved.

## Non-Features

Most features of **TatSu** are available in **TieXiu**. Some features have not yet been implemented, and a few never will:

* [ ] Generation of synthetic classes from grammar parameters will not be implemented in Rust.
* [ ] Generation of source code with an object model for deinitions in the grammar may be implemented if a way is found to make the parser or postprocessing bind the Tree output of a parse to the model ([serde_json][] provides the infrastructure for trying).
* [ ] Code generation of a parser recently moved in **TatSu** to the loading of a model of the Grammar and using it as parser. Although the generated procedural parser may produce 1.3x increased throughput in Python, supporting generated code is hard and it complicates the internal interfaces. For Rust, **TieXiu** alreay knows how to load _fast_ a Grammar model from **TatSu** JSON, which it can already produce. and a generated model constructor would be precompiled.
* [ ] Parsing of boolean and numeric values happens in **TatSu** through synthetic models, which call the constructors for those types passing the parsed strings. 

[serde_json]: https://docs.rs/serde_json/latest/serde_json/ 

## API

The needs of most users are met by parsing input with the rules in a grammar and reciving the structure output as a JSON-compatible value. For other use cases, **TieXiu** exposes its internal model and APIs (to be docummented).


## The Python API

The return values of `Any` are of the basic Python types, as defined in the `json` module documentation (see [Encoders and Decoders][] ). 

[Encoders and Decoders]: https://docs.python.org/3/library/json.html#json-to-py-table

| JSON          | Python |
|---------------|--------|
| object        | dict   |
| array         | list   |
| string        | str    |
| number (int)  | int    |
| number (real) | float  |
| true          | True   |
| false         | False  |
| null          | None   |

Keyword arguments can be passed for runtime configuration. The only recognized argument is `trace=`.

These functions are available from package `tiexiu`.

```python
def parse(grammar: str, text: str, **kwargs: Any) -> Any
def parse_grammar(grammar: str, **kwargs: Any) -> Any:
def parse_grammar_to_json(grammar: str, **kwargs: Any) -> Any:
def parse_to_json(grammar: str, text: str, **kwargs: Anyt) -> Any:
def pretty(grammar: str, **kwargs: Any) -> str:
def compile_to_json(grammar: str, **kwargs: Any) -> Any:
```

## The Rust API

```rust
pub fn parse_grammar(grammar: &str, cfg: &CfgA) -> Result<Tree>;
pub fn parse_grammar_to_json(grammar: &str, cfg: &CfgA) -> Result<serde_json::Value>;
pub fn parse_grammar_to_json_string(grammar: &str, cfg: &CfgA) -> Result<String>;
pub fn parse_grammar_with<U>(cursor: U, cfg: &CfgA) -> Result<Tree>
pub fn parse_grammar_to_json_with<U>(cursor: U, cfg: &CfgA) -> Result<serde_json::Value>
pub fn compile(grammar: &str, cfg: &CfgA) -> Result<Grammar>;
pub fn compile_to_json(grammar: &str, cfg: &CfgA) -> Result<serde_json::Value>;
pub fn compile_to_json_string(grammar: &str, cfg: &CfgA) -> Result<String>;
pub fn compile_with<U>(cursor: U, cfg: &CfgA) -> Result<Grammar>
pub fn compile_to_json_with<U>(cursor: U, cfg: &CfgA) -> Result<serde_json::Value>
pub fn load(json: &str, _cfg: &CfgA) -> Result<Grammar>;
pub fn load_to_json(json: &str, cfg: &CfgA) -> Result<serde_json::Value>;
pub fn load_tree(json: &str, _cfg: &CfgA) -> Result<Tree>;
pub fn load_tree_to_json(json: &str, cfg: &CfgA) -> Result<serde_json::Value>;
pub fn grammar_pretty(grammar: &str, cfg: &CfgA) -> Result<String>;
pub fn pretty_tree(tree: &Tree, _cfg: &CfgA) -> Result<String>;
pub fn pretty_tree_json(tree: &Tree, _cfg: &CfgA) -> Result<String>;
pub fn parse(grammar: &str, text: &str, cfg: &CfgA) -> Result<Tree>;
pub fn parse_to_json(grammar: &str, text: &str, cfg: &CfgA) -> Result<serde_json::Value>;
pub fn parse_to_json_string(grammar: &str, text: &str, cfg: &CfgA) -> Result<String>;
pub fn parse_input(parser: &Grammar, text: &str, cfg: &CfgA) -> Result<Tree>;
pub fn parse_input_to_json(parser: &Grammar, text: &str, cfg: &CfgA) -> Result<serde_json::Value>;
pub fn parse_input_to_json_string(parser: &Grammar, text: &str, cfg: &CfgA) -> Result<String>;
```

## Roadmap

The project is functionally complete, as described before. Comments about the implementation strategies and possible improvements are now in [RODADMAP](ROADMAP.md).

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless explicitly stated otherwise, any contribution intentionally submitted for inclusion in the work, as defined in the Apache-2.0 license, shall be dual-licensed as above, without any additional terms or conditions.

## Grammar Syntax

```console
start ●─grammar─■

grammar ●─ title=(`TATSU`) ──┬→─────────────────────────────────┬── rules+=(rule) ──┬→─────────────────────────────┬──⇥＄
                             ├→──┬─ directives+=(directive) ─┬──┤                   ├→──┬─ rules+=(rule) ───────┬──┤
                             │   └─ keywords+=(keyword) ─────┘  │                   │   └─ keywords+=(keyword) ─┘  │
                             └─────────────────────────────────<┘                   └─────────────────────────────<┘

directive ●─"@@"─ !["keyword"] ✂ ──┬─ name=(──┬─"comments"─────┬─)  ✂ "::" ✂  value=(regex) ────────────┬─ ✂ ─■
                                   │          └─"eol_comments"─┘                                        │
                                   ├─ name=("whitespace")  ✂ "::" ✂  value=(──┬─regex───┬─) ────────────┤
                                   │                                          ├─string──┤               │
                                   │                                          ├─"None"──┤               │
                                   │                                          ├─"False"─┤               │
                                   │                                          └─`None`──┘               │
                                   ├─ name=(──┬─"nameguard"──────┬─)  ✂ ──┬─"::" ✂  value=(boolean) ─┬──┤
                                   │          ├─"ignorecase"─────┤        └─ value=(``) ─────────────┘  │
                                   │          ├─"left_recursion"─┤                                      │
                                   │          ├─"parseinfo"──────┤                                      │
                                   │          └─"memoization"────┘                                      │
                                   ├─ name=("grammar")  ✂ "::" ✂  value=(word) ─────────────────────────┤
                                   └─ name=("namechars")  ✂ "::" ✂  value=(string) ─────────────────────┘

keywords ●───┬─keyword─┬───■
             └────────<┘

keyword ●─"@@keyword" ✂ "::"──┬─ +=(──┬─word───┬─) ─ ![──┬─":"─┬─]─┬───■
                              │       └─string─┘         └─"="─┘   │
                              └───────────────────────────────────<┘

params ●─ +=(first_param) ──┬→────────────────────────────┬───■
                            ├→"," +=(literal) ─ !["="] ✂ ─┤
                            └────────────────────────────<┘

first_param ●───┬─path────┬──■
                └─literal─┘

kwparams ●───┬─"," │ pair─┬───■
             └───────────<┘

the_params_at_last ●───┬─ kwparams=(kwparams) ────────────────────────┬──■
                       ├─ params=(params) "," ✂  kwparams=(kwparams) ─┤
                       └─ params=(params) ────────────────────────────┘

paramdef ●───┬─"[" ✂  >(the_params_at_last) "]"─┬──■
             ├─"(" ✂  >(the_params_at_last) ")"─┤
             └─"::" ✂  params=(params) ─────────┘

rule ●─ decorators=(──┬→──────────┬──)  name=(name)  ✂ ──┬─→ >(paramdef) ─┬───┬─→"<" ✂  base=(known_name) ─┬─ ∅ /=|::=|:=?/─ ✂  exp=(expre) ENDRULE ✂ ─■
                      ├→decorator─┤                      └─→──────────────┘   └─→──────────────────────────┘
                      └──────────<┘

ENDRULE ●───┬─DEDENT─┬─
            ├─BLANK──┤
            ├─";"────┤ ─┘
            └─⇥＄

DEDENT ●─EOL/\S/──■

BLANK ●─EOLEOL─■

EOL ●─/(?m)[ \t]*$/─/(?m)(?:\r?\n|\r)?/──■

decorator ●─"@"─ !["@"] ✂  =(──┬─"override"─┬─) ─■
                               ├─"name"─────┤
                               ├─"isname"───┤
                               └─"nomemo"───┘

pair ●─ +=(word) "=" ✂  +=(literal) ─■

expre ●───┬─choice───┬──■
          └─sequence─┘

choice ●───┬─→"|" ✂ ─┬─ +=(option) ──┬─"|" ✂  +=(option) ─┬───■
           └─→───────┘               └───────────────────<┘

option ●─sequence─■

sequence ●───┬── &[element","]──┬─"," │ element─┬───┬──■
             │                  └──────────────<┘   │
             └───┬── ![ENDRULE]element─┬────────────┘
                 └────────────────────<┘

element ●───┬─named────────┬──■
            ├─term─────────┤
            ├─override─────┤
            └─rule_include─┘

rule_include ●─">" ✂  =(known_name) ─■

named ●───┬─named_list───┬──■
          └─named_single─┘

named_list ●─ name=(name) /\+[:=]/─ ✂  exp=(term) ─■

named_single ●─ name=(name) /[:=]/─ ✂  exp=(term) ─■

override ●───┬─override_list──────────────┬──■
             ├─override_single────────────┤
             └─override_single_deprecated─┘

override_list ●─/\+=|@\+:/─ ✂  =(term) ─■

override_single ●─/=|@:/─ ✂  =(term) ─■

override_single_deprecated ●─"@" ✂  =(term) ─■

term ●───┬─gather─────────────┬──■
         ├─join───────────────┤
         ├─left_join──────────┤
         ├─right_join─────────┤
         ├─empty_closure──────┤
         ├─positive_closure───┤
         ├─closure────────────┤
         ├─optional───────────┤
         ├─atom───────────────┤
         ├─void───────────────┤
         ├─skip_to────────────┤
         ├─lookahead──────────┤
         ├─negative_lookahead─┤
         ├─cut────────────────┤
         └─cut_deprecated─────┘

group ●── !["(?:"]"(" ✂  =(expre) ")" ✂ ─■

skip ●─"(?:" ✂  =(expre) ")" ✂ ─■

gather ●── &[atom".{"] ✂ ──┬─positive_gather─┬──■
                           └─normal_gather───┘

positive_gather ●─ sep=(atom) ".{" exp=(expre) "}"/(?!\+=)[+-]/─ ✂ ─■

normal_gather ●─ sep=(atom) ".{" ✂  exp=(expre) "}"──┬─→"*" ✂ ─┬─ ✂ ─■
                                                     └─→───────┘

join ●── &[atom"%{"] ✂ ──┬─positive_join─┬──■
                         └─normal_join───┘

positive_join ●─ sep=(atom) "%{" exp=(expre) "}"/(?!\+=)[+-]/─ ✂ ─■

normal_join ●─ sep=(atom) "%{" ✂  exp=(expre) "}"──┬─→"*" ✂ ─┬─ ✂ ─■
                                                   └─→───────┘

left_join ●─ sep=(atom) "<{" ✂  exp=(expre) "}"/(?!\+=)[+-]/─ ✂ ─■

right_join ●─ sep=(atom) ">{" ✂  exp=(expre) "}"/(?!\+=)[+-]/─ ✂ ─■

positive_closure ●───┬─"{" =(expre) "}"/(?!\+=)[+-]/─ ✂ ─┬──■
                     └─ =(atom) /(?!\+=)[+]/─ ✂ ─────────┘

closure ●───┬─"{" =(expre) "}"──┬─→"*"─┬─ ✂ ─┬──■
            │                   └─→────┘     │
            └─ =(atom) "*" ✂ ────────────────┘

empty_closure ●─"{}" ✂  =( ∅ ) ─■

optional ●───┬─"[" ✂  =(expre) "]" ✂ ───────────┬──■
             └─ =(atom) ─ ![──┬─"?\""─┬─]"?" ✂ ─┘
                              ├─"?'"──┤
                              └─"?/"──┘

lookahead ●─"&" ✂  =(term) ─■

negative_lookahead ●─"!" ✂  =(term) ─■

skip_to ●─"->" ✂  =(term) ─■

atom ●───┬─token────┬──■
         ├─call─────┤
         ├─dot──────┤
         ├─pattern──┤
         ├─group────┤
         ├─eol──────┤
         ├─eof──────┤
         ├─skip─────┤
         ├─alert────┤
         └─constant─┘

call ●─word─■

void ●─"()" ✂ ─■

fail ●─"!()" ✂ ─■

cut ●─"~" ✂ ─■

cut_deprecated ●─">>" ✂ ─■

known_name ●─name ✂ ─■

name ●─word─■

constant ●── &["`"]──┬─/(?ms)```((?:.|\n)*?)```/──┬──■
                     ├─"`" =(literal) "`"─────────┤
                     └─/`(.*?)`/──────────────────┘

alert ●─ level=(/\^+/─)  message=(constant) ─■

token ●───┬─string─────┬──■
          └─raw_string─┘

literal ●───┬─string─────┬──■
            ├─raw_string─┤
            ├─boolean────┤
            ├─word───────┤
            ├─hex────────┤
            ├─float──────┤
            ├─int────────┤
            └─null───────┘

string ●── &[──┬─"\""─┬─]──┬─multiline_string─┬──■
               └─"'"──┘    ├─singlequoted─────┤
                           └─doublequoted─────┘

singlequoted ●─SINGLEQUOTED─■

doublequoted ●─DOUBLEQUOTED─■

raw_string ●─/r(?=["'])/─ =(STRING) ─■

STRING ●───┬─SINGLEQUOTED─┬──■
           └─DOUBLEQUOTED─┘

SINGLEQUOTED ●─/'((?:[^'\n]|\\'|\\\\)*?)/─ ✂ ─■

DOUBLEQUOTED ●─/"((?:[^"\n]|\\"|\\\\)*?)"/─ ✂ ─■

multiline_string ●───┬─/(?ms)'''((?:\\\\|\\.|(?!''').)*?)/─ ✂ ────┬──■
                     └─/(?ms)"""((?:\\\\|\\.|(?!""").)*?)"""/─ ✂ ─┘

hex ●─/0[xX](?:\d|[a-fA-F])+/──■

float ●─/[-+]?(?:\d+\.\d*|\d*\.\d+)(?:[Ee][-+]?\d+)?/──■

int ●─/[-+]?\d+/──■

path ●─/(?!\d)\w+(?:::(?!\d)\w+)+/──■

word ●─/(?!\d)\w+/──■

dot ●─"/./"─■

pattern ●─regex─■

regex ●───┬─deprecated_regex───────────────┬──■
          └── !["?/"]──┬─REGEX──────────┬──┘
                       └─"?" =(STRING) ─┘

REGEX ●── &["/"]/(?ms)/((?:[^/\\]|\\/|\\.)*)//─ ✂ ─■

deprecated_regex ●─"?/" ✂  =(/(?ms)((?:[^/\\]|\\/|\\.)*)/─)  ✂ "/?"─■

boolean ●───┬─"True"──┬──■
            └─"False"─┘

null ●─"None"─■

eof ●─"$" ✂ ─■

eol ●─"$->"─■


```