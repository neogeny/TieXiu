[![CodSpeed](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/neogeny/TieXiu?utm_source=badge)
# 铁修 TieXiu

A high-performance port of **TatSu** to Rust.

**TieXiu** (铁修) is a PEG (Parsing Expression Grammar) engine that implements the flexibility and power of the original **[TatSu][]** lineage into a memory-safe, high-concurrency architecture optimized for modern CPU caches.

[TatSu]: https://tatsu.readthedocs.io/en/stable/

## Roadmap

The [TatSu Documentation][] provides a vision of where the **TieXiu** projects is heading. A copy of the grammar syntax can can be accessed locally in the [SYNTAX](SYNTAX.md) document.

**TatSu** is a mature project with an important user base so it's difficult to make certain changes even if they are improvements or fixes for long-standing quirks (as well known within experienced software engineers, a long-lived quirk becomes a feature). **TieXiu** is an opportunity to start from scratch, with a modern approach, even if the grammar syntax and its semantics are preserved.

[TatSu Documentation]: https://tatsu.readthedocs.io/

## Current Project Status: Alpha

Implementation is complete for the core execution engine, the grammar model, the parsing engine, and support for left-recursive grammars. Development is currently advancing toward high-level tree construction and model synthesis.

### Lean Parsing Context

**TieXiu** uses the runtime stack as the parsing state stack. A state
change has a 16-byte stack footprint consisting of two  pointers: one for the *Volatile State* (Input Cursor) and one for the *Heavy Stated* (Grammar + Memoization Cache). This allows for deep recursive descent with
minimal stack pressure and $O(1)$ branching costs. The `Cursor`
implementation for parsing text (`StrCursor`) uses 16 bytes (`&str` +
`usize`) and has copy-on-write semantics during a parse (grammar elements
that don't advance over the input share the same cursor).


### Copy-on-Write State Transitios

Backtracking in **TieXiu** is *lazy*. Cloning a context/state only increments reference counts. The engine leverages the Rust runtime stack to handle state changes. Branching at choice points is a *16-byte* clone of the state, with a *16-byte* (for text) allocation only when the state is mutated. Failed parses restore the cursor position and register the furthest failure position for error reporting. The state returned occupies the same space as the original state.

A CST may use *64-bytes* per atomic node plus space proportional to the input matched, but CST are only kept for the currently successful path on the parse, and are dropped as soon as an option fails. CST are compacted on the boundary of the successful parse of a grammar rule node.

The failure path returns the furthest position reached in the input and a message about the error encountered there. The same error value is passed back during backtracking until a branch point is reached and another path can be tried. Atr branching the error value belonging to the furthes position in the input is chosen to pass back. The error value also passes back the _cut_ state so branches can commit to a failed alternative if it was fixed with a cut.

### Complete Parsing Engine

The core parsing logic is fully implemented, providing a robust execution environment for PEG rules. Choice points, sequences, and lookaheads are handled with high efficiency, leveraging the CoW context for seamless backtracking.

### Left Recursion Support

TieXiu features a complete implementation for handling left-recursive
grammars. A pre-pass *analysis* identifies and marks recursive cycles, while the *runtime* includes the necessary logic to grow the recursive content by iteration instead of recursion.

### Complete Grammar Model

The building blocks for grammar models are implemented with a clear chain of ownership. The `Grammar` acts as the root container owning the `Rule` map, while each `Rule` owns its `Model` definition. This hierarchy eliminates reference proliferation and simplifies lifetime management.

### Milestone: From CST to AST

 The algebra for creating **Concrete Syntax Trees (CST)** was ported from
 **TatSu** to **TieXiu** with optimizations. Instead of computing the
 resulting CST during parsing, the engine generates unoptimized trees that
 are destilled into their concrete versions at rule boundaries. **TieXiu**
 uses the **TatSu** semantics for **Abstract Syntax Tree (AST)**, in which
 named elements in a rule definition force the result to be a mapping of
 names to parsed elements. Rust doesn't allow the creation of synthetic
 types at runtime, so parsing to native types will require code generation
 for the desired model and deserialization of the JSON-compatible result of
 a parse into the desired model nodes.

### Packrat & Memoization

All branches in a parse use a shared *Memoization Cache* to achieve the `O(N) ` complexity of packrat parsers. The cache is pruned at `cut` points to place a bound on memory use and make the lookups more efficient.

### The Bootstrap Plan

A critical upcoming milestone is the **Bootstrap Process**. The roadmap
includes **Self-Hosting** through the implementation of a **TieXiu** grammar that describes its own EBNF. Grammars, including the grammar for grammars, will be passed to **TieXiu** using **TatSu**'s JSON export format for deseriallization into models.

After the initial bootstrap (**TieXiu** parsing grammars in its own grammar
language) either **TieXiu** or **TatSu** will generate the Rust code
necessary for faster parser bootstrap.

At the moment **TieXiu** is capable of reading the JSON representation of grammars that **TatSu** generates. In a two-step process the JSON is read and parsed to a model close to its structure. In a second step the intermediate model is translated to a grammar model that can be use for parsing.

* [x] The model pretty-prints to the original grammar text in TatSu-compatibleEBNF
* [x] **TieXiu** is capable of parsing the JSON representation of the **TatSu** EBNF grammar language
* [ ] **TieXiu** should be capable of parsing the EBNF text for the **TatSu** grammar language
* [ ] Any EBNF grammar should be parsed by the tool and applied to input in the language described by the gramar

## Features

* [x] **16-byte Handle Size**: Optimized for L1 cache and deep recursion.
* [x] **32-byte State Changes**: Efficient CoW allocation on the Rust runtime stack.
* [x] **Complete Parsing Engine**: Core PEG execution logic is fully implemented.
* [x] **Left Recursion**: Both analysis and runtime support are complete.
* [x] **Complete Grammar Model**: Rules and Models are fully defined and owned.
* [x] **Thread-Safe Grammar**: Static grammar structure can be shared across multiple threads.
* [x] **Efficient Memoization**: Global cache consistency across backtracking branches.
* [x] **Object-Safe Cursors**: Abstract `Cursor` trait allows for string, byte, or custom stream inputs.
* [ ] **Self-Hosting Bootstrap**: (Planned) Engine parsing its own EBNF.

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

    grammar[Grammar] ●─ [title](`TATSU`)──┬→───────────────────────────────────┬── [`rules`]+(rule)──┬→───────────────────────────────┬──⇥＄
                                          ├→──┬─ [`directives`]+(directive)─┬──┤                     ├→──┬─ [`rules`]+(rule)───────┬──┤
                                          │   └─ [`keywords`]+(keyword)─────┘  │                     │   └─ [`keywords`]+(keyword)─┘  │
                                          └───────────────────────────────────<┘                     └───────────────────────────────<┘

    directive ●─'@@'─ !['keyword'] ✂ ───┬─ [name](──┬─'comments'─────┬─) ✂ ─'::' ✂ ─ [value](regex)────────────┬─ ✂ ──■
                                        │           └─'eol_comments'─┘                                         │
                                        ├─ [name]('whitespace') ✂ ─'::' ✂ ─ [value](──┬─regex───┬─)────────────┤
                                        │                                             ├─string──┤              │
                                        │                                             ├─'None'──┤              │
                                        │                                             ├─'False'─┤              │
                                        │                                             └─`None`──┘              │
                                        ├─ [name](──┬─'nameguard'──────┬─) ✂ ───┬─'::' ✂ ─ [value](boolean)─┬──┤
                                        │           ├─'ignorecase'─────┤        └─ [value](`True`)──────────┘  │
                                        │           ├─'left_recursion'─┤                                       │
                                        │           ├─'parseinfo'──────┤                                       │
                                        │           └─'memoization'────┘                                       │
                                        ├─ [name]('grammar') ✂ ─'::' ✂ ─ [value](word)─────────────────────────┤
                                        └─ [name]('namechars') ✂ ─'::' ✂ ─ [value](string)─────────────────────┘

    keywords ●───┬─keywords─┬───■
                 └─────────<┘

    keyword ●─'@@keyword' ✂ ─'::' ✂ ───┬→──────────────────────────────────┬───■
                                       ├→ @+(──┬─word───┬─)─ ![──┬─':'─┬─]─┤
                                       │       └─string─┘        └─'='─┘   │
                                       └──────────────────────────────────<┘

    the_params_at_last ●───┬─ [kwparams](kwparams)─────────────────────────┬──■
                           ├─ [params](params)',' ✂ ─ [kwparams](kwparams)─┤
                           └─ [params](params)─────────────────────────────┘

    paramdef ●───┬─'[' ✂ ─ >(the_params_at_last) ']'─┬──■
                 ├─'(' ✂ ─ >(the_params_at_last) ')'─┤
                 └─'::' ✂ ─ [params](params)─────────┘

    rule[Rule] ●─ [decorators](──┬→──────────┬──) [name](name) ✂ ───┬─→ >(paramdef) ─┬───┬─→'<' ✂ ─ [base](known_name)─┬───┬─'='──┬─ ✂ ─ [exp](expre)ENDRULE ✂ ──■
                                 ├→decorator─┤                      └─→──────────────┘   └─→───────────────────────────┘   ├─':='─┤
                                 └──────────<┘                                                                             └─':'──┘

    ENDRULE ●───┬── &[UNINDENTED]──────┬──■
                ├─EMPTYLINE──┬─→';'─┬──┤
                │            └─→────┘  │
                ├─⇥＄                  │
                └─';'──────────────────┘

    UNINDENTED ●─/(?=\s*(?:\r?\n|\r)[^\s])/──■

    EMPTYLINE ●─/(?:\s*(?:\r?\n|\r)){2,}/──■

    decorator ●─'@'─ !['@'] ✂ ─ @(──┬─'override'─┬─)─■
                                    ├─'name'─────┤
                                    └─'nomemo'───┘

    params ●─ @+(first_param)──┬→────────────────────────────┬───■
                               ├→',' @+(literal)─ !['='] ✂ ──┤
                               └────────────────────────────<┘

    first_param ●───┬─path────┬──■
                    └─literal─┘

    kwparams ●───┬→────────────┬───■
                 ├→',' ✂ ─pair─┤
                 └────────────<┘

    pair ●─ @+(word)'=' ✂ ─ @+(literal)─■

    expre ●───┬─choice───┬──■
              └─sequence─┘

    choice[Choice] ●───┬─→'|' ✂ ──┬─ @+(option)──┬─'|' ✂ ─ @+(option)─┬───■
                       └─→────────┘              └───────────────────<┘

    option[Option] ●─ @(sequence)─■

    sequence[Sequence] ●───┬── &[element',']──┬→───────────────┬───┬──■
                           │                  ├→',' ✂ ─element─┤   │
                           │                  └───────────────<┘   │
                           └───┬── ![ENDRULE]element─┬─────────────┘
                               └────────────────────<┘

    element ●───┬─rule_include─┬──■
                ├─named────────┤
                ├─override─────┤
                └─term─────────┘

    rule_include[RuleInclude] ●─'>' ✂ ─ @(known_name)─■

    named ●───┬─named_list───┬──■
              └─named_single─┘

    named_list[NamedList] ●─ [name](name)'+:' ✂ ─ [exp](term)─■

    named_single[Named] ●─ [name](name)':' ✂ ─ [exp](term)─■

    override ●───┬─override_list──────────────┬──■
                 ├─override_single────────────┤
                 └─override_single_deprecated─┘

    override_list[OverrideList] ●─'@+:' ✂ ─ @(term)─■

    override_single[Override] ●─'@:' ✂ ─ @(term)─■

    override_single_deprecated[Override] ●─'@' ✂ ─ @(term)─■

    term ●───┬─void───────────────┬──■
             ├─gather─────────────┤
             ├─join───────────────┤
             ├─left_join──────────┤
             ├─right_join─────────┤
             ├─empty_closure──────┤
             ├─positive_closure───┤
             ├─closure────────────┤
             ├─optional───────────┤
             ├─skip_to────────────┤
             ├─lookahead──────────┤
             ├─negative_lookahead─┤
             ├─cut────────────────┤
             ├─cut_deprecated─────┤
             └─atom───────────────┘

    group[Group] ●─'(' ✂ ─ @(expre)')' ✂ ──■

    gather ●── &[atom'.{'] ✂ ───┬─positive_gather─┬──■
                                └─normal_gather───┘

    positive_gather[PositiveGather] ●─ [sep](atom)'.{' [exp](expre)'}'──┬─'+'─┬─ ✂ ──■
                                                                        └─'-'─┘

    normal_gather[Gather] ●─ [sep](atom)'.{' ✂ ─ [exp](expre)'}'──┬─→'*' ✂ ──┬─ ✂ ──■
                                                                  └─→────────┘

    join ●── &[atom'%{'] ✂ ───┬─positive_join─┬──■
                              └─normal_join───┘

    positive_join[PositiveJoin] ●─ [sep](atom)'%{' [exp](expre)'}'──┬─'+'─┬─ ✂ ──■
                                                                    └─'-'─┘

    normal_join[Join] ●─ [sep](atom)'%{' ✂ ─ [exp](expre)'}'──┬─→'*' ✂ ──┬─ ✂ ──■
                                                              └─→────────┘

    left_join[LeftJoin] ●─ [sep](atom)'<{' ✂ ─ [exp](expre)'}'──┬─'+'─┬─ ✂ ──■
                                                                └─'-'─┘

    right_join[RightJoin] ●─ [sep](atom)'>{' ✂ ─ [exp](expre)'}'──┬─'+'─┬─ ✂ ──■
                                                                  └─'-'─┘

    positive_closure[PositiveClosure] ●───┬─'{' @(expre)'}'──┬─'-'─┬─ ✂ ──┬──■
                                          │                  └─'+'─┘      │
                                          └─ @(atom)'+' ✂ ────────────────┘

    closure[Closure] ●───┬─'{' @(expre)'}'──┬─→'*'─┬─ ✂ ──┬──■
                         │                  └─→────┘      │
                         └─ @(atom)'*' ✂ ─────────────────┘

    empty_closure[EmptyClosure] ●─'{}' ✂ ─ @( ∅ )─■

    optional[Optional] ●───┬─'[' ✂ ─ @(expre)']' ✂ ──────────┬──■
                           └─ @(atom)─ ![──┬─'?"'─┬─]'?' ✂ ──┘
                                           ├─"?'"─┤
                                           └─'?/'─┘

    lookahead[Lookahead] ●─'&' ✂ ─ @(term)─■

    negative_lookahead[NegativeLookahead] ●─'!' ✂ ─ @(term)─■

    skip_to[SkipTo] ●─'->' ✂ ─ @(term)─■

    atom ●───┬─group────┬──■
             ├─token────┤
             ├─alert────┤
             ├─constant─┤
             ├─call─────┤
             ├─pattern──┤
             ├─dot──────┤
             └─eof──────┘

    call[Call] ●─word─■

    void[Void] ●─'()' ✂ ──■

    fail[Fail] ●─'!()' ✂ ──■

    cut[Cut] ●─'~' ✂ ──■

    cut_deprecated[Cut] ●─'>>' ✂ ──■

    known_name ●─name ✂ ──■

    name ●─word─■

    constant[Constant] ●── &['`']──┬─/(?ms)```((?:.|\n)*?)```/──┬──■
                                   ├─'`' @(literal)'`'──────────┤
                                   └─/`(.*?)`/──────────────────┘

    alert[Alert] ●─ [level](/\^+/─) [message](constant)─■

    token[Token] ●───┬─string─────┬──■
                     └─raw_string─┘

    literal ●───┬─string─────┬──■
                ├─raw_string─┤
                ├─boolean────┤
                ├─word───────┤
                ├─hex────────┤
                ├─float──────┤
                ├─int────────┤
                └─null───────┘

    string ●─STRING─■

    raw_string ●─//─ @(STRING)─■

    STRING ●───┬─ @(/"((?:[^"\n]|\\"|\\\\)*?)"/─) ✂ ─────┬──■
               └─ @(/r"'((?:[^'\n]|\\'|\\\\)*?)'"/─) ✂ ──┘

    hex ●─/0[xX](?:\d|[a-fA-F])+/──■

    float ●─/[-+]?(?:\d+\.\d*|\d*\.\d+)(?:[Ee][-+]?\d+)?/──■

    int ●─/[-+]?\d+/──■

    path ●─/(?!\d)\w+(?:::(?!\d)\w+)+/──■

    word ●─/(?!\d)\w+/──■

    dot[Dot] ●─'/./'─■

    pattern[Pattern] ●─regexes─■

    regexes ●───┬→─────────────┬───■
                ├→'+' ✂ ─regex─┤
                └─────────────<┘

    regex ●───┬─'/' ✂ ─ @(/(?:[^/\\]|\\/|\\.)*/─)'/' ✂ ──┬──■
              ├─'?' @(STRING)────────────────────────────┤
              └─deprecated_regex─────────────────────────┘

    deprecated_regex ●─'?/' ✂ ─ @(/(?:.|\n)*?(?=/\?)/─)//\?+/─ ✂ ──■

    boolean ●───┬─'True'──┬──■
                └─'False'─┘

    null ●─'None'─■

    eof[EOF] ●─'$' ✂ ──■
    
```
