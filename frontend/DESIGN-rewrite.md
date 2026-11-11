# Design of the rewrite

This document is the design of the front end's rewrite (decision 0060,
decision 0063 item RD). It gives the stages and their data, a module
for each page of the specification, the place of each section of
`TEMPLATE-MODEL.md`, the order of the build, and the hook that lets a
Propose design be prototyped. `DESIGN-current.md` describes the code
that the rewrite replaces, and this document calls that code "the old
front end".

Decision 0063 starts no rewrite code before the gate of decisions 0062
and 0063 holds. This document has no code. The structures below are
sketches, and the Rust types are fixed when the code is written.

## 1. Where the rewrite lives

The new front end is a crate of its own beside the old one:

```text
frontend/
  src/                the old front end (tidal-fe)
  next/               the new front end (tidal-next)
    Cargo.toml
    src/
      main.rs         the driver
      source/ diag/ lex/ syntax/ sem/ target/ machine/ conform/   core
      foundations/ expressions/ statements/ declarations/
      classes/ overloading/ templates/ exceptions/                pages
      proposals/                                                  prototypes
```

`tidal-next` takes the subcommands and exit statuses of `tidal-fe`
(`check`, `run`, `conform`, `dump`), so the harness and
`tools/frontend_check.py` read both binaries the same way. The crate
uses no crate that encodes knowledge of C++ (decision 0035).

## 2. Stages and their data

```text
source file
   │  lex            preprocessing slice, then tokens
   ▼
Vec<Token>           each token with a Span (file, line, column)
   │  syntax         page syntax parts; asks the Names oracle
   ▼                 one declaration at a time
syn tree             one node per use of a production
   │  analysis       page check parts, in three passes:
   │                   declare  entities and scopes, members in order
   │                   bodies   deferred bodies after their class (NAM-6)
   │                   end      specializations at their points (INST-31)
   │                 constant evaluation on demand, on the machine
   ▼
Unit                 entities, typed trees with dependence marks,
   │                 specializations, diagnostics; read-only from here
   │  driver         accept, reject, or unsupported
   ▼  (run only, after every unit is accepted)
machine              page run parts: link, static initialization,
   │                 main, then a run for each other choice
   ▼
RunOutcome
```

The syntax stage and the analysis stage alternate. Analysis asks the
parser for the next declaration, the parser returns its tree, and
analysis declares what the tree declares before it asks for the next
one. A class body works the same way member by member. A member
function body, a default argument, and a default member initializer
are kept as a token range, and the parser makes their trees when the
class is complete (NAM-6). Each token range is parsed once.

The parser does no lookup. At the points where the grammar needs to
know what a name is, it asks the `Names` oracle, which analysis
implements:

| Question | Where the grammar needs it | Rules |
|---|---|---|
| Is this name a type? | a declaration or an expression statement; a cast or a parenthesized expression | STMT-6, DEP-19, DEP-20 |
| Is this name a template, and of which kind? | `<` after a name | DEP-24, DEP-26, TARG rules |
| Is this name a namespace or a class? | `::` after a name | NAM rules |

Under a dependent qualifier, the oracle answers by the reading that the
syntax chose (`typename`, `template`, or a context of DEP-20). In the
current instantiation, it answers from the members known so far
(section 4, model section 2). The parser's unit tests use a stub
oracle.

### The data

```text
Token     { kind, text, span: Span, lit: Option<Lit> }        as now, plus Span
Span      { file: FileId, line, col }
syn::*    Decl | Stmt | Expr | TypeId | Declarator | ... | Ext(ProposalId, node)
          each node names its production and carries a Span
Unit      { entities: [Entity], scopes: [Scope], types: TypeTable,
            templates: [Template], specializations: [Specialization],
            bodies: EntityId -> Stmt, diags: [Diag] }
Type      Known(kind, cv) | Dep(DepType) | Error              (model section 1)
Expr      { kind, ty, cat, dep: Marks, span }                  Marks: model section 1
Stmt      Expr | Decl | Block | If | ... | Break { target } | Continue { target }
          | Ext(ProposalId, node) | Unsupported(why)
Subst<T>  Ok(T) | Failure(why)                                 model section 4
Diag      { sev, rule, cause, msg, span }
```

`Break` and `Continue` name the statement they act on, which analysis
chose by JMP-1 and JMP-2. The machine then needs no search for the
innermost loop. Section 7 uses this.

`Type::Error` is the type of a construct after an error was reported.
A check does not fire on it, so one error does not give a cascade. A
construct the front end does not model gives `Stmt::Unsupported` or an
unsupported entity, and the outcome is `unsupported` (decision 0060
keeps that result). A declaration whose type is unknown for any other
reason is a fault of the front end and shows as one.

### How each stage differs from the old front end

| Stage | Old front end (`DESIGN-current.md`) | Rewrite |
|---|---|---|
| Lexing | `lex_file`; a token has a line | the same slice and tokens; a token has a file, a line, and a column; the literal checks sit in `expressions::primary` |
| Parsing | no stage of its own; parsing and analysis are one pass over the tokens | a tree for each declaration; one function per production, named after it; the parser asks only the oracle |
| Disambiguation | a look-ahead (`is_decl_start`) and tentative parses with the diagnostics muted | the grammar with the oracle's answers; no tentative parse mutes a diagnostic |
| Analysis | methods of `P`, which has 395 public fields; flags set for the next call | functions of the page modules over a `Check` context; a fact a caller passes is a parameter |
| Templates | the tokens, parsed again for each specialization (`replay`), with facts kept under token positions | the typed tree of the definition, and a substitution into it (model sections 1 and 3) |
| Substitution failure | any muted error during a replay | a `Subst::Failure` value; an error outside the immediate context is a diagnostic (model section 4) |
| Unknown constructs | `Ty::Unknown` and uncertain scopes silence checks | `Type::Error` after an error only; an unmodelled construct is `unsupported` |
| Constant evaluation | the machine with `constant` set | the same |
| Interpretation | the machine over fully resolved trees | the same contract; the rules move to the run parts of their pages |
| Diagnostics | a line | a span; the same printed form, so the harness and the first-error count read both |
| Tests | the conformance harness only | the harness, and unit tests in each module |

## 3. A module for each page

### The parts of a page module

A page module is a directory `src/<chapter>/<page>/` with up to four
files. Each part sits in one stage:

| File | Stage | Holds |
|---|---|---|
| `syntax.rs` | syntax | the productions of the page's Syntax segment, one function each |
| `check.rs` | analysis, constant evaluation | the page's rules that analysis checks, and the limits of constant evaluation that the page states |
| `run.rs` | interpretation | the page's rules that the machine follows |
| `tests.rs` | unit tests | tests of the other parts |

A page with many rules splits `check.rs` by principle (for example
`check/ranking.rs`), so that no file passes about 2000 lines. The
module stays the page.

A function that implements a rule names the rule ID in its doc comment.
A module names only the rule IDs of its own page. When it needs another
page's rule, it calls that page's function. A check of the crate can
then list each rule ID by the module that names it, and a reader goes
from a rule to one module.

A chapter's index page has no rules. Its chapter module
(`src/<chapter>/mod.rs`) holds the index page's Syntax segment, such as
`translation_unit` and `declaration` (`declarations/index.md`) or
`statement` (`statements/index.md`), and calls the page modules. No two
rule pages share a module. The 10 index pages each give one chapter
module.

### The core

The core holds data and machinery. It names no rule of a page, except
where the table says so.

| Module | Holds | From the old front end |
|---|---|---|
| `source` | files, spans, the include stack | parts of `lexer.rs` |
| `diag` | `Diag`, severities, the kind of each rule (read from `specification/`, as `conform.rs` does now), the overrides of a proposal (section 7) | `diag.rs` |
| `lex` | the token stream and the preprocessing slice; it calls `foundations::odr` (INC), `foundations::keywords` (KW), and `expressions::primary` (literals) | `lexer.rs` |
| `syntax` | the cursor, the table of nonterminals, error recovery (it reports TR-12 through `foundations::translation`), the `Names` oracle trait, the syn node types | none |
| `sem` | the arenas, entities, scopes, the type table with `DepType`, the typed tree with its marks, `Subst`, the `Check` context, `Unit` | `types.rs`, `ast.rs`, the records of `parser.rs` |
| `target` | the LP64 and ILP32 tables: sizes, alignments, class layout | `layout.rs`, the size functions of `types.rs` |
| `machine` | `Machine`, `Val`, `Ptr`, `Obj`, `Frame`, `Stop`, `Choice`, the byte view's tables, the x87 format | `interp/mod.rs`, `interp/f80.rs`, parts of `interp/bytes.rs` |
| `conform` | the conformance harness | `conform.rs` |
| `driver` (`main.rs`) | the command line, the outcome, the run of a program, the `--proposal` flag | `main.rs`, parts of `interp/run.rs` |

### The page table

Stages: S syntax, A analysis, C constant evaluation, R interpretation.
L marks a page with a part that `lex` calls. "Depends on" lists page
modules. Every module also uses the core.

| Page | Module | Prefixes | Stages | Depends on |
|---|---|---|---|---|
| `foundations/translation` | `foundations::translation` | TR | L, S, A, R | `foundations::odr`, `declarations::storage_linkage` |
| `foundations/keywords` | `foundations::keywords` | KW | L | none |
| `foundations/names` | `foundations::names` | NAM | A | none (searches the scope graph of `sem`, bases included) |
| `foundations/odr` | `foundations::odr` | INC, ODR | L, A, R | `foundations::names` |
| `foundations/types` | `foundations::types` | TYP | A | none (uses `target`) |
| `foundations/values` | `foundations::values` | VC | A, R | `foundations::types` |
| `foundations/conversions` | `foundations::conversions` | CONV | A, R | `foundations::types`, `foundations::values` |
| `foundations/objects` | `foundations::objects` | OBJ | A, R | `foundations::types` |
| `foundations/storage` | `foundations::storage` | STG | A, R | `foundations::objects`, `foundations::types` |
| `foundations/evaluation-order` | `foundations::evaluation_order` | SEQ | R | `foundations::objects` |
| `foundations/constant-evaluation` | `foundations::constant_evaluation` | CE | A, C | `foundations::objects`, `foundations::storage` |
| `foundations/language-support` | `foundations::language_support` | LSUP | A, R | `foundations::types`, `foundations::storage` |
| `expressions/primary` | `expressions::primary` | PRIM | L, S, A, R | `foundations::names`, `foundations::values`, `templates::packs` |
| `expressions/postfix` | `expressions::postfix` | PFX | S, A, R | `foundations::names`, `foundations::conversions`, `classes::access`, `overloading::overload_resolution` |
| `expressions/unary` | `expressions::unary` | UN | S, A, R | `foundations::conversions`, `classes::operators` |
| `expressions/casts` | `expressions::casts` | CAST | S, A, R | `foundations::conversions`, `foundations::storage`, `classes::derived`, `classes::virtual` |
| `expressions/member-pointers` | `expressions::member_pointers` | MPTR | S, A, R | `classes::members`, `classes::derived` |
| `expressions/multiplicative` | `expressions::multiplicative` | MUL | S, A, R | `foundations::conversions`, `classes::operators` |
| `expressions/additive` | `expressions::additive` | ADD | S, A, R | `foundations::conversions`, `foundations::storage`, `classes::operators` |
| `expressions/shift` | `expressions::shift` | SHF | S, A, R | `foundations::conversions`, `classes::operators` |
| `expressions/comparison` | `expressions::comparison` | CMP | S, A, R | `foundations::conversions`, `classes::operators`, `classes::comparisons` |
| `expressions/bitwise` | `expressions::bitwise` | BIT | S, A, R | `foundations::conversions`, `classes::operators` |
| `expressions/logical` | `expressions::logical` | LOG | S, A, R | `foundations::conversions`, `classes::operators` |
| `expressions/conditional` | `expressions::conditional` | COND | S, A, R | `foundations::conversions`, `overloading::conversion_sequences`, `overloading::overload_resolution` |
| `expressions/assignment` | `expressions::assignment` | ASN | S, A, R | `foundations::conversions`, `classes::operators`, `classes::copy_move` |
| `expressions/comma` | `expressions::comma` | COMMA | S, A, R | `foundations::evaluation_order`, `classes::operators` |
| `expressions/decltype` | `expressions::decltype` | DTY, UNEV | S, A | `foundations::values`, `templates::dependent_names` |
| `expressions/lambdas` | `expressions::lambdas` | LAM | S, A, R | `foundations::names`, `classes::members`, `templates::declarations` |
| `expressions/new-delete` | `expressions::new_delete` | NEW | S, A, R | `foundations::storage`, `declarations::alignment`, `declarations::initialization::forms`, `overloading::overload_resolution` |
| `statements/blocks` | `statements::blocks` | STMT | S, A, R | `foundations::names` |
| `statements/if` | `statements::if_stmt` | IF | S, A, R | `foundations::conversions`, `foundations::constant_evaluation`, `templates::instantiation` |
| `statements/switch` | `statements::switch` | SW | S, A, R | `foundations::conversions`, `foundations::constant_evaluation`, `statements::blocks` |
| `statements/loops` | `statements::loops` | LOOP | S, A, R | `foundations::conversions`, `statements::blocks` |
| `statements/range-for` | `statements::range_for` | RFOR | S, A, R | `statements::loops`, `declarations::initialization::forms`, `overloading::overload_resolution` |
| `statements/jumps` | `statements::jumps` | JMP | S, A, R | `statements::blocks` |
| `declarations/declarators` | `declarations::declarators` | CV, DCL | S, A | `foundations::types`, `foundations::names`, `foundations::constant_evaluation` |
| `declarations/type-names` | `declarations::type_names` | TYN | S, A | `foundations::names`, `classes::members` |
| `declarations/attributes` | `declarations::attributes` | ATTR | S, A | none |
| `declarations/alignment` | `declarations::alignment` | ALN | S, A | `foundations::types`, `foundations::constant_evaluation` |
| `declarations/auto` | `declarations::auto` | AUTO | S, A | `templates::deduction`, `declarations::initialization::list` |
| `declarations/constexpr` | `declarations::constexpr` | CEV, CINIT, CX | A, C | `foundations::constant_evaluation`, `declarations::functions::definitions` |
| `declarations/enumerations` | `declarations::enumerations` | ENUM | S, A, R | `foundations::conversions`, `foundations::constant_evaluation` |
| `declarations/exception-specifications` | `declarations::exception_specifications` | EXS | S, A, R | `foundations::constant_evaluation`, `exceptions::throw` |
| `declarations/functions/definitions` | `declarations::functions::definitions` | FN | S, A, R | `declarations::declarators` |
| `declarations/functions/default-arguments` | `declarations::functions::default_arguments` | DEFARG | S, A | `declarations::initialization::forms` |
| `declarations/functions/deleted` | `declarations::functions::deleted` | DEL | A | `overloading::overload_resolution` |
| `declarations/functions/return-deduction` | `declarations::functions::return_deduction` | RET | A | `declarations::auto` |
| `declarations/initialization/forms` | `declarations::initialization::forms` | INIT | S, A, R | `foundations::conversions`, `classes::constructors`, `overloading::overload_resolution` |
| `declarations/initialization/list` | `declarations::initialization::list` | LIST | S, A, R | `declarations::initialization::forms`, `overloading::conversion_sequences`, `classes::initializer_lists` |
| `declarations/initialization/references` | `declarations::initialization::references` | REF | A, R | `foundations::conversions`, `foundations::values`, `overloading::conversion_sequences` |
| `declarations/initialization/static` | `declarations::initialization::static_init` | SINIT | A, R | `foundations::constant_evaluation`, `declarations::initialization::forms` |
| `declarations/inline` | `declarations::inline` | INL | A | `foundations::odr` |
| `declarations/linkage-specifications` | `declarations::linkage_specifications` | LSP | S, A, R | `declarations::storage_linkage` |
| `declarations/namespaces` | `declarations::namespaces` | NS | S, A | `foundations::names` |
| `declarations/static-assert` | `declarations::static_assert` | SA | S, A | `foundations::constant_evaluation`, `templates::instantiation` |
| `declarations/storage-linkage` | `declarations::storage_linkage` | LNK, STO | S, A, R | `foundations::names`, `foundations::odr` |
| `declarations/structured-bindings` | `declarations::structured_bindings` | SB | S, A, R | `declarations::initialization::forms`, `declarations::initialization::references`, `templates::arguments` |
| `declarations/volatile` | `declarations::volatile` | VOL | A, R | `foundations::types`, `foundations::storage` |
| `classes/members` | `classes::members` | CLS | S, A, R | `foundations::names`, `declarations::declarators` |
| `classes/access` | `classes::access` | ACC | S, A | `classes::members`, `classes::derived` |
| `classes/derived` | `classes::derived` | DER | S, A, R | `classes::members` |
| `classes/virtual` | `classes::virtual` | VIRT | S, A, R | `classes::derived` |
| `classes/constructors` | `classes::constructors` | CTOR, DTOR | S, A, R | `classes::members`, `declarations::initialization::forms`, `overloading::overload_resolution`, `exceptions::unwinding` |
| `classes/copy-move` | `classes::copy_move` | CPY | A, R | `classes::constructors`, `overloading::overload_resolution` |
| `classes/conversion-functions` | `classes::conversion_functions` | CVF | S, A | `overloading::overload_resolution` |
| `classes/initializer-lists` | `classes::initializer_lists` | ILC | A, R | `classes::constructors`, `overloading::overload_resolution` |
| `classes/operators` | `classes::operators` | OPR | S, A | `classes::members`, `overloading::overload_resolution` |
| `classes/comparisons` | `classes::comparisons` | DCMP | S, A, R | `classes::operators`, `foundations::language_support` |
| `classes/unions` | `classes::unions` | BF, UNI | S, A, R | `classes::members`, `foundations::storage` |
| `overloading/conversion-sequences` | `overloading::conversion_sequences` | ICS | A | `foundations::conversions`, `foundations::values`, `classes::derived` |
| `overloading/overload-resolution` | `overloading::overload_resolution` | OVL | A | `overloading::conversion_sequences`, `templates::deduction`, `templates::constraints`, `templates::ordering` |
| `templates/declarations` | `templates::declarations` | TPL | S, A | `foundations::names`, `templates::dependent_names` |
| `templates/arguments` | `templates::arguments` | TARG | S, A | `foundations::conversions`, `foundations::constant_evaluation` |
| `templates/dependent-names` | `templates::dependent_names` | DEP | S, A | `foundations::names`, `classes::derived`, `templates::packs` |
| `templates/packs` | `templates::packs` | PACK | A | `templates::arguments` |
| `templates/instantiation` | `templates::instantiation` | INST | A | `templates::dependent_names`, `templates::specialization`, `templates::deduction` |
| `templates/specialization` | `templates::specialization` | TSP | S, A | `templates::arguments`, `templates::deduction`, `templates::ordering` |
| `templates/deduction` | `templates::deduction` | DED | A | `foundations::conversions`, `templates::arguments`, `templates::dependent_names` |
| `templates/ordering` | `templates::ordering` | ORD | A | `templates::deduction`, `templates::constraints` |
| `templates/concepts` | `templates::concepts` | CPT | S, A, C | `templates::constraints` |
| `templates/constraints` | `templates::constraints` | CNS | A, C | `foundations::constant_evaluation`, `templates::instantiation` |
| `templates/class-template-deduction` | `templates::class_template_deduction` | CTAD | A | `templates::deduction`, `overloading::overload_resolution`, `classes::constructors` |
| `exceptions/throw` | `exceptions::throw` | THR | S, A, R | `declarations::initialization::forms`, `exceptions::unwinding` |
| `exceptions/handlers` | `exceptions::handlers` | TRY | S, A, R | `foundations::conversions`, `classes::derived`, `exceptions::unwinding` |
| `exceptions/unwinding` | `exceptions::unwinding` | UNW | R | `foundations::objects` |

The chapter modules:

| Index page | Chapter module | Syntax it holds |
|---|---|---|
| `foundations/index` | `foundations` | none |
| `expressions/index` | `expressions` | the table of levels; each level's production sits on its page |
| `statements/index` | `statements` | `statement`, `plain_stmt` |
| `declarations/index` | `declarations` | `translation_unit`, `declaration` |
| `declarations/functions/index` | `declarations::functions` | none |
| `declarations/initialization/index` | `declarations::initialization` | none |
| `classes/index` | `classes` | none |
| `overloading/index` | `overloading` | none |
| `templates/index` | `templates` | none |
| `exceptions/index` | `exceptions` | none |

### Cycles between pages

Some pages use each other. Overload resolution needs the constructors
and conversion functions of a class, and those pages need overload
resolution to choose. The class pages put their candidates in the class
record of `sem` (`ctors`, `conversion_fns`, `operators`), and
`overloading::overload_resolution` reads the record. So the edge runs
one way in the table. `foundations::names` searches the bases through
the class record too, and depends on no class page. Where the table
still has a cycle (`templates::instantiation` and
`templates::constraints`), the two modules meet through one function
each: instantiation calls `constraints::satisfied`, and constraints
calls `instantiation::substitute`.

## 4. Where the template model lives

| Model section | Modules | Data in the core |
|---|---|---|
| 1. A typed tree with dependence | `templates::dependent_names` computes type and value dependence (DEP-1 to DEP-4) and the binding of a name (DEP-5, DEP-7, DEP-8, DEP-40). `templates::packs` computes the unexpanded packs (PACK-8, PACK-13, PACK-19), and `expressions::primary` removes the pack of a fold (PRIM-27). Each expression page calls these when it makes a node. | `sem::Expr::dep` (the marks), `sem::DepType` |
| 2. The current instantiation | `templates::declarations` makes the `CurrentInst` record as the class template's body is read. `templates::dependent_names` holds the names of DEP-15 and the lookup procedure (DEP-11, DEP-14, DEP-16 to DEP-18, DEP-24, DEP-36, DEP-39). `foundations::names` does the search of the members and the independent bases. | `sem::CurrentInst`; the oracle answers from it (section 2) |
| 3. A specialization by substitution | `templates::instantiation` holds the `Specialization` record, its points (INST-30, INST-31, INST-33), its states, the order of INST-P2, `busy`, and the nesting limit (INST-15, with TR-13 from `foundations::translation`). The substitution walks the definition's tree. It copies a node with no dependence, and it calls the page's check again for a dependent node with the arguments in place (DEP-28). `templates::specialization` chooses the source (TSP-13 to TSP-16) and links a partial specialization to its primary when it is declared (TSP-31). | `sem::Specialization` |
| 4. Substitution failure and the immediate context | `templates::deduction` decides where an error falls (DED-24 to DED-29). `templates::dependent_names` holds the kind table of `Member(T, name, reading)` (DEP-19, DEP-22, DEP-26). | `sem::Subst<T>`; an error outside the immediate context goes to `diag` |
| 5. Checks at the definition | each row of the model's table sits in the page of its rule: DEP-6, DEP-17, DEP-24, DEP-29, DEP-30, and DEP-37 in `templates::dependent_names`; IF-22 in `statements::if_stmt`; PACK-8 in `templates::packs`; CNS-29 in `templates::constraints`; SA-5 in `declarations::static_assert`. An independent node is checked when it is made, by the same code as a node outside a template. | the list of definition checks, which a proposal can extend (section 7) |
| 6. The 30 tests | section 5 of this document | the marks file |
| 7. What the model keeps | `templates::deduction` (DED-P3), `templates::ordering` (ORD-P2), `templates::specialization` (TSP-P2), `templates::constraints` (CNS-P1) | `DepType` with its new forms |
| 8. Rules that can change | section 10 of this document | none |

Model section 5 needs no pass of its own. A node the marks call
independent goes through its page's check at once. So IF-22 is the
check of `statements::if_stmt` that already runs on a discarded
statement outside a template, and `o.missing` in
`other-specialization-constant-argument-member-missing.cpp` is the
member check of `expressions::postfix`.

## 5. The tests the rewrite takes over first

The rewrite takes over the 30 tests of the marks file
(`reports/clean-room/rust-frontend/rewrite-marks.tsv`) first, by model
section. Paths are under `tests/`.

| Model section | Count | Tests |
|---|---|---|
| 1 | 4 | `templates/dependent-names/`: `other-specialization-constant-argument-member-missing`, `other-specialization-constant-argument-no-typename`, `sizeof-value-dependent-operand-call-bound-early`, `sizeof-value-dependent-operand-no-typename` |
| 2 | 5 | `templates/dependent-names/`: `current-instantiation-enclosing-base`, `member-template-current-instantiation-pointer`, `member-template-current-instantiation-qualifier-dependent-base`, `member-template-non-dependent-base`, `member-template-through-current-instantiation-object` |
| 3 | 10 | `templates/instantiation/`: `class-point-not-later-member-type`, `class-point-not-later-type`, `class-point-not-later`, `class-point-static-const-member`, `class-point-static-constexpr-made-at-use`, `class-static-constexpr-member-initializer-waits`, `local-class-default-argument`, `local-class-exception-specification`, `recursion-endless-function`; `templates/specialization/partial-no-primary-variable` |
| 4 | 4 | `templates/deduction/`: `substitution-explicit-specifier`, `substitution-lambda-body`, `substitution-lambda-default`, `substitution-member-wrong-kind` |
| 5 | 7 | `statements/if/constexpr-template-nondependent-call-checked`; `templates/constraints/`: `out-of-class-other-clause`, `out-of-class-without-clause`; `templates/dependent-names/`: `binary-operator-pack-never-fits`, `operator-pack-never-fits`; `templates/packs/`: `fold-in-outer-pattern`, `size-operator-in-outer-pattern` |

Each name ends in `.cpp`. A round that marks the three P3 tests of
model section 6 for section 3 adds them to the section 3 row.

### What `tools/frontend_check.py` measures

The tool now runs one front end and prints the completion line with the
marked tests apart. The rewrite needs three more things from it. Each
comes with the step that first needs it, and none changes the meaning
of the lines the tool prints now.

- **A shadow run.** With `--next`, the tool also runs `tidal-next` on
  every test and prints its counts beside the old ones. The shadow run
  changes no status and no baseline.
- **A takeover list.** A file beside `conformance.tsv` names the tests
  that the new front end owns. For an owned test, the tool takes the
  new front end's status. A test that either front end makes worse
  fails the check, so "0 worse" holds for both (decision 0060).
- **A parse count.** The tool counts the tests that `tidal-next`
  parses as their outcome needs: each test whose expected outcome is
  not a parse error gives a tree, and each test that cites TR-12 alone
  stops in the parser.

```text
next: lex: T/T tests give the old front end's tokens
next: parse: N/T tests parse as their outcome needs
next: shadow: P/T pass, Q of the old front end's passes; R tests pass on the new front end only
next: owned: O tests, O pass their check, 0 worse
frontend: completion (decision 0060): N tests fail their check, of which M need the template model ...
```

### The order of the build

Each step keeps the existing line "0 worse" of `frontend_check.py`.
Each step adds unit tests for the modules it writes.

| Step | Work | Takes over | Measure |
|---|---|---|---|
| 0 | This design. No code until the gate of decisions 0062 and 0063 holds. | none | the completion line: at most 50 tests outside the marks file |
| 1 | The crate, `source`, `diag`, `lex`, and the driver; `--next` in the tool | none | `next: lex` reaches T/T |
| 2 | `syntax` and the `syntax.rs` part of each page; the oracle over a declare pass that only records names and their kinds | none | `next: parse` |
| 3 | `sem`, `target`, `machine`; the foundations, declarations, expressions, and statements pages outside templates; constant evaluation | none | `next: shadow` by chapter, against the old front end's passes on those chapters |
| 4 | The classes, overloading, and exceptions pages | none | `next: shadow` |
| 5 | The templates pages with the parts the model keeps (model section 7); model sections 1, 2, and 5 | the 16 tests of sections 1, 2, and 5 | `next: owned` 16; the marked count on the completion line goes from 30 to 14 |
| 6 | Model section 3 | the 10 tests of section 3 | the marked count goes from 14 to 4 |
| 7 | Model section 4 | the 4 tests of section 4 | the marked count goes from 4 to 0 |
| 8 | Page by page, the new front end takes over each page whose tests it passes wherever the old one passes | the tests of each page | `next: owned` grows by the page's tests, 0 worse |
| 9 | The old front end is removed when the new one owns every test that the old one passed (decision 0060) | all | the completion line counts the new front end only |

Model sections 1, 2, and 5 come first because section 2 needs the marks
of section 1, and section 5 needs both. Section 3 needs the marks for
its substitution, and section 4 needs section 3, because an
instantiation during deduction goes through section 3. The program
tests under `tests/programs/` pass to the new front end with the pages
whose rules they use, in step 8.

A test the new front end passes and the old one fails, outside the
marks file, is a takeover of step 8 too. The shadow line counts such
tests as "pass on the new front end only".

## 6. Unit tests

Each page module's `tests.rs` tests its parts on small inputs:

| Part | A unit test gives | and checks |
|---|---|---|
| `syntax.rs` | a token string and a stub oracle | the tree, or the TR-12 stop |
| `check.rs` | a short program through the declare pass, or a built `Check` context | the typed node, its marks, and the diagnostics with their rule IDs |
| `run.rs` | a typed tree and a machine | the value, or the `Stop` with its rule |

The core modules have unit tests of their own (`lex` on literals and
the slice, `sem` on the type table, `machine` on places and the byte
view). `cargo test` in `frontend/next/` runs them all.

## 7. The prototype hook for Propose

A Propose design (`documentation/exercises.md`, sections 4 and 9)
changes or adds rules, and names each new rule in the exercise's own
space (`X24:new-1`). A prototype of the design lives in one module,
`src/proposals/<id>.rs`, and a line in `src/proposals/mod.rs` registers
it. The prototype runs only under the flag `--proposal <id>` of
`tidal-next check`, `run`, and `conform`. It changes no page module.

### The extension points

The page modules offer four points, and a prototype attaches only
there:

| Point | Owner | What a prototype can do |
|---|---|---|
| Nonterminals | `syntax` | add an alternative to any nonterminal; the parser tries it at the start of that nonterminal and makes a `syn::Ext` node |
| `Ext` nodes | `sem`, `machine` | check its own syn node into ordinary typed nodes, or into a `sem::Ext` node with a `run` function of its own |
| Rule overrides | `diag` | change the kind of a rule by its ID (as from `recommended` to `reject`), or turn a check off; add rules with the exercise's IDs, which the diagnostics name |
| Definition checks | `templates::dependent_names` | add a check to the list that runs on a template's definition (model section 5) |

A page module keeps public only the queries that other modules need,
and a prototype uses those queries. No page module imports
`proposals`.

```text
proposals::p22  ──alternative at jump_stmt──►  syntax (nonterminal table)
      │                                         └ statements::jumps::syntax (unchanged)
      ├──check of its Ext node──►  sem; reads statements::blocks::enclosing()
      │                                 and statements::jumps::labels() (public queries)
      └──gives Break { target } / Continue { target }
                                       └► machine and statements::jumps::run (unchanged)
```

### A worked example: P22, labeled `break` and `continue`

P22 (`reports/clean-room/roadmap-7/portfolio.md`) lets `break name;`
and `continue name;` act on the loop or `switch` that carries the label
`name`. Today JMP-17 rejects both forms through the grammar, because
`jump_stmt` has no place for a label after `break` or `continue`
(TR-12).

The prototype `src/proposals/p22.rs` has three parts:

1. **Syntax.** It adds the alternatives `"break" identifier ";"` and
   `"continue" identifier ";"` to `jump_stmt`. The parser tries them at
   the start of `jump_stmt` and makes `syn::Ext(P22, LabeledJump { kind,
   label })`.
2. **Check.** It looks up the label in the function's labels
   (`statements::jumps::labels()`, the table that `goto` uses, NAM-P4).
   It walks the enclosing statements (`statements::blocks::enclosing()`,
   the stack that JMP-1 and JMP-2 use) to the statement that carries the
   label. It reports the design's new rules: a label that names no
   enclosing loop or `switch`, a `continue` whose label names a
   `switch`, and a label outside the lambda body that holds the jump
   (JMP-5, JMP-7). It gives an ordinary `Break { target }` or
   `Continue { target }`.
3. **Run.** None. The machine already ends the objects of each block
   that a jump leaves (JMP-9), and a `continue` goes on as after a
   normal pass of the body (LOOP-9) for any target.

| Rule | Change | Where the prototype acts |
|---|---|---|
| JMP-17 | the grammar admits a label after `break` and `continue` | the alternative at `jump_stmt` |
| JMP-1, JMP-2 | the target is the labeled statement when a label is given | the check's `target` |
| JMP-9, LOOP-9 | none | the machine, unchanged |
| `Xn:new-1` to `Xn:new-3` | the label's target must be an enclosing loop or `switch`, a `continue` must target a loop, and the label must be in the same function | the check |

`Xn` stands for the exercise that takes P22. The files that change are
`src/proposals/p22.rs` and one line of `src/proposals/mod.rs`.
`statements::jumps` does not change, because its `Break` and `Continue`
already name their target (section 2).

The impact count comes from the harness. `tidal-next conform ../tests`
runs with and without `--proposal P22`, and the tests whose status
differs are the "Tests affected" column of the change table. For P22,
the two tests that JMP-17 cites (`statements/jumps/break-with-label.cpp`
and `continue-with-label.cpp`) are the ones the portfolio expects to
change. The same diff over `tests/programs/` gives the counts of
sections 4.3 and 4.4 of a Propose report.

A prototype of X22's checked marker uses the fourth point. It adds one
check to the definition checks, as the designer view of roadmap round
12 (item 8) describes, and it adds a rule override for the marker.

## 8. What carries over from `frontend/src`

The old front end's files, by what the rewrite does with them.
"Moved" means the code moves with small changes. "Rewritten" means the
algorithm stays and the code is new, because it is a method of `P`
now. "Dropped" means no part carries over.

| Old file or part | Rewrite | New home |
|---|---|---|
| `lexer.rs` | moved; tokens gain a span | `lex`, `source`; literal checks to `expressions::primary`, includes to `foundations::odr` |
| `diag.rs` | moved; a span replaces the line | `diag` |
| `types.rs` | moved; `Dep` gains the forms of `DepType`, and `Ty::Unknown` goes | `sem` (types), `target` (sizes) |
| `ast.rs` | moved; nodes gain marks and spans | `sem` (typed tree) |
| `layout.rs` | moved | `target` |
| `conform.rs` | moved, with the same statuses and output | `conform` |
| `main.rs` | moved; adds `--proposal` | driver |
| `interp/mod.rs`, `interp/f80.rs` | moved | `machine` |
| `interp/expr.rs`, `interp/stmt.rs`, `interp/bytes.rs` | moved and split by page | the `run.rs` part of each page (`expressions::new_delete`, `foundations::storage`, `expressions::casts`, `exceptions::handlers`, ...) |
| `interp/run.rs` | moved | driver (exploration), `foundations::translation` and `declarations::storage_linkage` (linking), `declarations::initialization::static_init` |
| `consteval.rs` | moved | `foundations::constant_evaluation` |
| `templ.rs`: `deduce_ty`, `partial_order`, `select_partial` | rewritten | `templates::deduction`, `templates::ordering`, `templates::specialization` |
| `cons.rs`: normal forms, satisfaction, `subsumes` | rewritten | `templates::constraints`, `templates::concepts` |
| `ics.rs`, `ovl.rs` | rewritten | `overloading::conversion_sequences`, `overloading::overload_resolution` |
| `special.rs`, `dcmp.rs`, `ctad.rs` | rewritten | `classes::copy_move` and `classes::constructors`, `classes::comparisons`, `templates::class_template_deduction` |
| the checks in `decl.rs`, `decl2.rs`, `expr.rs`, `expr2.rs`, `stmt.rs`, `class.rs` | rewritten, one rule at a time, in the page of the rule | the `check.rs` parts |
| `parser.rs`: lookup | rewritten | `foundations::names` |
| `parser::P` as one state, and `Saved` | dropped | `Check` context and the arenas of `sem` |
| `replay`, `bind_params`, `pattern_facts`, `has_fact`, `Source::limit`, `copy_toks` | dropped | the substitution of model section 3 |
| a muted error as a substitution failure (`Diags::muted`) | dropped | `Subst` (model section 4) |
| `Ty::Unknown` and uncertain scopes as silencers | dropped | `Type::Error` and `unsupported` |
| `is_decl_start`, `decl_after_type`, tentative parses | dropped | the grammar and the oracle |

The old code's rule comments and diagnostics name a rule beside the code
that follows it (`DESIGN-current.md`, "Where each chapter's rules sit").
The rewrite uses that map to find the old check for each rule it
writes. The run of `tidal-next run` keeps the thread with a 512 MiB
stack, because the machine recurses on the program's own recursion.

## 9. What a round of the rewrite reports

A front-end round that works on the rewrite keeps the form of report
25, with these additions:

- the four `next:` lines of `frontend_check.py`,
- the owned tests by page,
- the marked tests that remain, by model section,
- the rule IDs that a module names outside its own page, which should
  be none.

## 10. Open points

- The crate's place and the binary's name (`frontend/next/`,
  `tidal-next`) are provisional.
- The parser is written by hand from the Syntax segments, one function
  per production. Decision 0035 also allows a generated parser. A check
  that each production has one function, and each function one
  production, would hold either way.
- The oracle's three questions may not cover every ambiguity of the
  grammar. A tree with both readings is the fallback for a point the
  oracle cannot answer when the parser reaches it.
- `tools/frontend_check.py` has no `--next`, no takeover list, and no
  parse count yet. Each lands with the step that needs it.
- The unit of takeover after step 7 is the page. A page whose tests the
  new front end passes in part could pass test by test instead.
- The three P3 tests (P3-6, INST-5) join model section 3 if a round
  marks them.
- The first-error target for the new code is open. The old front end
  names a cited or linked rule first in 77% of its passing rejects
  (report 25). A target at or above that rate for owned tests fits the
  implementer view of roadmap round 12 (process change 5).
- The `forward_class` gap of the RP commit (attributes after the class
  key) has no ledger row. Step 2 meets it in `classes::members` or
  `declarations::type_names`.
- `TEMPLATE-MODEL.md` section 8 lists Q-0427, Q-0432, Q-0434, P3-6,
  and RV8. An answer that changes a rule there changes the module that
  section 4 of this document names for it.
- The kind of each rule comes from `specification/` at run time, as in
  the old harness. A table built into the crate would make the binary
  stand alone and need a rebuild for each text change.
- A prototype runs under a run-time flag. Whether prototypes stay in
  the crate after their exercise, or move to a branch, is open.
- While both front ends exist, a fix in the old front end for a test
  the new one owns changes no result. Rounds then fix owned tests in the
  new code only.
