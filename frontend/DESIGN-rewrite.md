# Design of the rewrite

This document is the design of the front end's rewrite (decision 0060,
decision 0063 item RD). It gives the stages and their data, a module
for each page of the specification, the place of each section of
`TEMPLATE-MODEL.md`, the order of the build, and the hook that lets a
Propose design be prototyped. `DESIGN-current.md` describes the code
that the rewrite replaces, and this document calls that code "the old
front end".

Decision 0064 starts the rewrite as item RW. Build steps 1 and 2 are in
`frontend/next/`: the crate, `source`, `diag`, `lex`, the driver, the
`syntax` core, the `syntax.rs` part of each page with a Syntax segment,
and the declare pass that answers the parser's questions. Decision 0065
(item RW2) brings build step 3 in part: `sem`, `target`, `machine`,
`conform`, the `check.rs` and `run.rs` parts of the step 3 pages for
constructs outside classes and templates, the node-check and `Ext` node
points, and the P22 prototype. Section 5 gives what step 3 still lacks.
The structures of the later steps below are sketches, and their Rust
types are fixed when their code is written.

## 1. Where the rewrite lives

The new front end is a crate of its own beside the old one:

```text
frontend/
  src/                the old front end (tidal-fe)
  next/               the new front end (tidal-next)
    Cargo.toml        Cargo.lock and target/ of its own
    src/
      main.rs         the driver
      source/ diag/ lex/ syntax/ sem/ target/ machine/ conform/   core
      foundations/ expressions/ statements/ declarations/
      classes/ overloading/ templates/ exceptions/                pages
      proposals/                                                  prototypes
```

`tidal-next` takes the subcommands and exit statuses of `tidal-fe`
(`check`, `run`, `conform`, `dump`), so the harness and
`tools/frontend_check.py` read both binaries the same way. Both
binaries have `lex FILE...`,
which prints one token a line (kind, text, line in the unit's main
file, literal) and the lexer's diagnostics, and `tidal-next` has
`parse FILE... [--all]`, which prints `tree` or the stop for each file,
and with `--all` every other place where the grammar derives no form.
Since build step 3, `check` gives the outcome of analysis, `run` runs a
program of one translation unit, and `conform` runs the harness. The
crate uses no crate that encodes knowledge of C++ (decision 0035), and
so far no crate at all.

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
| Is a member template of this name known? | `<` after a name after `.` or `->` with no `template` | DEP-24 |
| Which class is the current instantiation? | a template-id with dependent arguments before `::` | DEP-15, model section 2 |

Under a dependent qualifier, the oracle answers by the reading that the
syntax chose (`typename`, `template`, or a context of DEP-20). In the
current instantiation, it answers from the members known so far
(section 4, model section 2). The parser's unit tests use a stub
oracle.

The parser tells the oracle what each declaration declares as soon as
its declarator, head, or parameter is read, and opens and closes the
scopes of namespaces, classes, blocks, template heads, and function
prototypes. In step 2 the oracle is `sem::names`, a declare pass that
records names and their kinds and knows no types. So its answer to the
fourth question is "a member template of this name is declared in some
class": the object's type, which DEP-24 needs, comes with analysis
(section 10).

### What analysis reads

The parser makes one node type, `syntax::Node { prod, ext, span, kids }`,
and analysis reads that node. There is no second, typed form of the syn
tree. Each page's `check.rs` part reads the nodes of its own page's
productions by their production names (`n.child("type_id")`), and makes
the typed tree of `sem` (`sem::tree`), which is the only typed tree.
The reasons:

- The node names its production, so a page reads only what its own
  Syntax segment gives, and a grammar change touches the page's
  `syntax.rs` and `check.rs` and no shared type.
- The nonterminal point and the `Ext` node point stay uniform: a
  proposal's node is a `Node` with `ext` set, and its check gets the
  same node.

In build step 3, analysis runs after the parser has read the whole unit
with the step 2 oracle, and it reads the trees in order. For the pages
of step 3 the result is the same as the alternation that this section
describes. The alternation comes with the class and template pages,
where the oracle needs analysis (section 10).

### The data

```text
Token     { kind, text, span: Span, lit: Option<Lit> }        as now, plus Span
Span      { file: FileId, line, col }
syntax::Node  { prod, ext: Option<ProposalId>, span, kids }  one type for every production
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
keeps that result). An error that the front end reports after the
first unmodelled construct of a unit can rest on that construct, so it
gives no verdict, and the unit is `unsupported` (decision 0066). An
error reported earlier, also on the construct's own line, still makes
the unit a reject. A declaration whose type is unknown for any other
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
| `syntax.rs` | syntax | the productions of the page's Syntax segment, one function each; `expressions::primary` keeps its lexical productions in `lexical.rs`, which `lex` calls |
| `check.rs` | analysis, constant evaluation | the page's rules that analysis checks, and the limits of constant evaluation that the page states |
| `run.rs` | interpretation | the page's rules that the machine follows |
| `tests.rs` | unit tests | tests of the other parts |

A page with many rules splits `check.rs` by principle (for example
`check/ranking.rs`), so that no file passes about 2000 lines. The
module stays the page.

A production marked `shape` (style guide, section 8) names a form that
a general production also derives. Its function is a recognizer over
the tree that the general production made, such as `copy_ctor(node,
tokens, class)`, and the productions that only a shape uses are
recognizers too. The `preprocessing` productions of `foundations::odr`
are functions over a directive line, which `lex` calls.

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
module, and a chapter module with productions keeps them in its own
`syntax.rs`.

### The core

The core holds data and machinery. It names no rule of a page, except
where the table says so.

| Module | Holds | From the old front end |
|---|---|---|
| `source` | files, spans, the include stack | parts of `lexer.rs` |
| `diag` | `Diag`, severities, the kind of each rule (read from `specification/`, as `conform.rs` does now), the overrides of a proposal (section 7) | `diag.rs` |
| `lex` | the token stream and the preprocessing slice; it calls `foundations::odr` (INC), `foundations::keywords` (KW), and `expressions::primary` (literals) | `lexer.rs` |
| `syntax` | the cursor, the table of nonterminals, error recovery (it reports TR-12 through `foundations::translation`), the `Names` oracle trait, the syn node types | none |
| `sem` | the arenas, entities, scopes, the type table with `DepType`, the typed tree with its marks, `Subst`, the `Check` context, `Unit` | written new in step 3 (`sem::types`, `sem::tree`, `sem::check`); `DepType`, the marks, and `Subst` come with step 5 |
| `target` | the LP64 and ILP32 tables: sizes, alignments, class layout | written new in step 3 from TYP-1 and TYP-2; class layout comes with step 4 |
| `machine` | `Machine`, `Val`, `Ptr`, `Obj`, `Frame`, `Stop`, `Choice`, the byte view's tables, the x87 format | written new in step 3 for the step 3 constructs; it names no rule, and calls the `run.rs` part of a page for each operation whose rule can stop a run. The x87 format is the run part of `foundations::types`. The byte view and the exploration of choice points are not in it yet |
| `conform` | the conformance harness | `conform.rs`, with the same statuses and output, without the exploration of choice points |
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
| `foundations/constant-evaluation` | `foundations::constant_evaluation` | CE | S, A, C | `foundations::objects`, `foundations::storage` |
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
| `classes/copy-move` | `classes::copy_move` | CPY | S, A, R | `classes::constructors`, `overloading::overload_resolution` |
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
| `templates/instantiation` | `templates::instantiation` | INST | S, A | `templates::dependent_names`, `templates::specialization`, `templates::deduction` |
| `templates/specialization` | `templates::specialization` | TSP | S, A | `templates::arguments`, `templates::deduction`, `templates::ordering` |
| `templates/deduction` | `templates::deduction` | DED | A | `foundations::conversions`, `templates::arguments`, `templates::dependent_names` |
| `templates/ordering` | `templates::ordering` | ORD | A | `templates::deduction`, `templates::constraints` |
| `templates/concepts` | `templates::concepts` | CPT | S, A, C | `templates::constraints` |
| `templates/constraints` | `templates::constraints` | CNS | A, C | `foundations::constant_evaluation`, `templates::instantiation` |
| `templates/class-template-deduction` | `templates::class_template_deduction` | CTAD | S, A | `templates::deduction`, `overloading::overload_resolution`, `classes::constructors` |
| `exceptions/throw` | `exceptions::throw` | THR | S, A, R | `declarations::initialization::forms`, `exceptions::unwinding` |
| `exceptions/handlers` | `exceptions::handlers` | TRY | S, A, R | `foundations::conversions`, `classes::derived`, `exceptions::unwinding` |
| `exceptions/unwinding` | `exceptions::unwinding` | UNW | R | `foundations::objects` |

The chapter modules:

| Index page | Chapter module | Syntax it holds |
|---|---|---|
| `foundations/index` | `foundations` | none |
| `expressions/index` | `expressions` | the table of levels; each level's production sits on its page |
| `statements/index` | `statements` | `statement`, `plain_stmt` |
| `declarations/index` | `declarations` | `translation_unit`, `declaration`, `block_decl`, `simple_decl`, `init_declarator`, `decl_specifiers`, `decl_specifier`, `type_specifiers`, `type_specifier`, `simple_type` |
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
| 6. The 38 tests | section 5 of this document | the marks file |
| 7. What the model keeps | `templates::deduction` (DED-P3), `templates::ordering` (ORD-P2), `templates::specialization` (TSP-P2), `templates::constraints` (CNS-P1) | `DepType` with its new forms |
| 8. Rules that can change | section 10 of this document | none |

Model section 5 needs no pass of its own. A node the marks call
independent goes through its page's check at once. So IF-22 is the
check of `statements::if_stmt` that already runs on a discarded
statement outside a template, and `o.missing` in
`other-specialization-constant-argument-member-missing.cpp` is the
member check of `expressions::postfix`.

## 5. The tests the rewrite takes over first

The rewrite takes over the tests of the marks file
(`reports/clean-room/rust-frontend/rewrite-marks.tsv`) by model section
(decision 0064). The marks file is the list. After front-end round 28 it
holds 38 tests: 17 for step 5, 15 for step 6, and 6 for step 7. The
table gives the tests. Paths are under `tests/`.

| Model section | Count | Tests |
|---|---|---|
| 1 | 4 | `templates/dependent-names/`: `other-specialization-constant-argument-member-missing`, `other-specialization-constant-argument-no-typename`, `sizeof-value-dependent-operand-call-bound-early`, `sizeof-value-dependent-operand-no-typename` |
| 2 | 6 | `templates/dependent-names/`: `current-instantiation-enclosing-base`, `member-template-current-instantiation-pointer`, `member-template-current-instantiation-qualifier-dependent-base`, `member-template-non-dependent-base`, `member-template-through-current-instantiation-object`; `classes/copy-move/class-template-ctor-by-value` |
| 3 | 15 | `templates/instantiation/`: `class-point-not-later-member-type`, `class-point-not-later-type`, `class-point-not-later`, `class-point-static-const-member`, `class-point-static-constexpr-made-at-use`, `class-static-constexpr-member-initializer-waits`, `class-static-constexpr-member-calls-own-member`, `static-inline-member-incomplete-argument`, `class-point-after-enclosing-class-not-later`, `local-class-default-argument`, `local-class-exception-specification`, `recursion-endless-function`; `templates/specialization/partial-no-primary-variable`; `templates/packs/member-template-pack-length`; `programs/roadmap/r14-fsm-second-try` |
| 4 | 6 | `templates/deduction/`: `substitution-explicit-specifier`, `substitution-lambda-body`, `substitution-lambda-default`, `substitution-member-wrong-kind` |; `templates/arguments/argument-makes-parameter-invalid`; `templates/dependent-names/type-only-positions-more`
| 5 | 7 | `statements/if/constexpr-template-nondependent-call-checked`; `templates/constraints/`: `out-of-class-other-clause`, `out-of-class-without-clause`; `templates/dependent-names/`: `binary-operator-pack-never-fits`, `operator-pack-never-fits`; `templates/packs/`: `fold-in-outer-pattern`, `size-operator-in-outer-pattern` |

Each name ends in `.cpp`. A round that marks a test adds it to the
marks file, and the step of its model section takes it over.

### What `tools/frontend_check.py` measures

Every run builds both front ends. Each test of the takeover list takes
the new front end's status in the "0 worse" comparison, in `--update`,
and on the completion line, which ends with "O owned by the new front
end"; every other test keeps the old front end's status (decision 0066,
item TK). With `--next`, the tool also prints these lines after them:

```text
next: unit: K tests pass
next: lex: T/N files give the old front end's tokens
next: parse: N/M tests parse as their outcome needs
next: parse: R of the D disagreements are reject tests that stop on the line of the old front end's first error
next: shadow: step 3 plain: P/N pass their check, Q of the old front end's O
next: shadow: step 3: P/N pass their check, Q of the old front end's O; R pass on the new front end only
next: shadow: all: P/N pass their check; U unsupported
next: owned: O tests (takeover list)
```

- **Unit tests** (step 1). `cargo test` in `frontend/next/`. A failing
  unit test fails the check.
- **The lex count** (step 1). Both binaries run `lex` on every test
  file and extra translation unit, and the dumps must be equal.
- **The parse count** (step 2). A test needs a tree for each of its
  units when it expects `accept`, `run`, `run-fail`, or `link-fail`,
  or when it expects `reject` and the old front end's first error names
  a rule other than TR-12. It needs a stop when its first error names
  TR-12. A `reject` test with no error that names a rule is not judged.
  The tool lists each disagreement with every place where the grammar
  derives no form (`tidal-next parse --all`, which skips each failed
  statement, member, or declaration and goes on), and counts the reject
  tests whose first stop is on the line of the old front end's first
  error (section 10).
- **The takeover list** (read from step 1). `takeover.tsv` beside
  `conformance.tsv` names the tests that the new front end owns: a test
  path, the build step, and the commit. The tool reports a row whose
  test does not exist, a row of the wrong form, a test listed twice, and
  an owned test whose status on the new front end is not a passing
  status, and each of these fails the check (decision 0065: ownership
  moves test by test, and a listed test that the new front end then
  fails is a worse status). Every run, with or without `--next`, reports
  these problems, so `tools/check_all.sh` holds the owned tests.
- **A shadow run** (step 3). The tool also runs `tidal-next conform` on
  every test and prints the shadow lines above. "step 3" is the tests
  whose rules sit on the pages of the step 3 chapters (the prefixes of
  section 3's page table), and "step 3 plain" is those that use no
  keyword of a later step's pages. The tool lists each step 3 plain test
  that the old front end passes and the new one does not. The shadow run
  changes no status and no baseline.

### The order of the build

Each step keeps the existing line "0 worse" of `frontend_check.py`.
Each step adds unit tests for the modules it writes. The Propose hook
of section 7 grows with the steps, so that a point exists before the
pages it reaches are written.

| Step | Work | Takes over | Measure |
|---|---|---|---|
| 0 | This design, kept to the marks file | none | the completion line, with the marked tests apart |
| 1 | The crate, `source`, `diag`, `lex`, and the driver with `--proposal` and an empty registry; `--next` in the tool | none | `next: lex` reaches N/N; `next: unit` |
| 2 | `syntax` and the `syntax.rs` part of each page; the oracle over a declare pass that only records names and their kinds; the nonterminal point, with P22's syntax part as its unit test | none | `next: parse`; each nonterminal has one function |
| 3 | The node-check point of section 7 first; then `sem`, `target`, `machine`, `conform`; the foundations, declarations, expressions, and statements pages outside classes and templates, with the non-class parts of the two overloading pages; constant evaluation; the `Ext` node point with a proposal's own rule IDs; at its end the first takeover and the P22 prototype (X24) | the tests of the step 3 plain scope that both front ends pass (decision 0065) | `next: shadow` on the step 3 plain scope and the step 3 scope, against the old front end's passes; the P22 impact diff |
| 4 | The classes, overloading, and exceptions pages; the rule override point | none | `next: shadow` |
| 5 | The templates pages with the parts the model keeps (model section 7); model sections 1, 2, and 5, with the definition-check point | the 17 tests of sections 1, 2, and 5 | `next: owned` grows by 17; the marked count on the completion line goes from 38 to 21 |
| 6 | Model section 3 | the 15 tests of section 3 | the marked count goes from 21 to 6 |
| 7 | Model section 4 | the 6 tests of section 4 | the marked count goes from 6 to 0 |
| 8 | Page by page, the new front end takes over each page whose tests it passes wherever the old one passes | the tests of each page | `next: owned` grows by the page's tests, 0 worse |
| 9 | The old front end is removed when the new one owns every test that the old one passed (decision 0060) | all | the completion line counts the new front end only |

Steps 1 and 2 are in place. Every nonterminal of the Syntax segments
has its function, and the parse count's disagreements are listed by
cause in the front end's round report.

Step 3 is in place in part (items RW2 and RW3, `reports/clean-room/rust-frontend/report-rw3.md`
and the commits of item RW3). The new front end passes the check of
1819 of the 1903 step 3 plain tests (1813 of them tests that the old
front end passes too), and it owns 1557 of them. Of the 84 plain tests
that it does not pass, 74 are unsupported, 68 of them for a lambda, and
10 have the status EDG-AGREES where the old front end has EDG-ONLY.
These parts are not in it yet: lambdas (their closure is a class), the
linking of several translation units, the exploration of choice points,
the byte view of objects, extended floating-point literals, and an
array with more elements than the machine holds.

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

The page modules offer five points, and a prototype attaches only
there:

| Point | Owner | What a prototype can do |
|---|---|---|
| Nonterminals | `syntax` | add an alternative to any nonterminal; the parser tries it at the start of that nonterminal and makes a `syn::Ext` node |
| `Ext` nodes | `sem`, `machine` | check its own syn node into ordinary typed nodes, or into a `sem::Ext` node with a `run` function of its own; name the design's own rules (`X24:new-1`) in its diagnostics |
| Rule overrides | `diag` | change the kind of a rule by its ID (as from `recommended` to `reject`), or turn a check off |
| Node checks | `sem` | run a check of its own on each typed node of a named kind, after the page's own check; for a dependent node, again at substitution (DEP-28) |
| Definition checks | `templates::dependent_names` | add a check to the list that runs on a template's definition (model section 5) |

Each point lands with a build step: the `--proposal` flag and the
registry in step 1, the nonterminal point in step 2, the node-check
point before step 3 writes the expression pages, the `Ext` node point
with the design's own rule IDs in step 3, rule overrides of existing
rules in step 4, and definition checks in step 5 (section 5). In the
crate, the node-check point is `sem::check::NodeCheck`, which
`expressions::check::expr` runs on each typed node after the page's own
check, and the `Ext` node point is `sem::check::Hooks::ext_stmt`, which
`statements::check` calls for a statement node that a proposal's
alternative made. A prototype's registry line gives its alternatives
and a function that returns its hooks.

The kind of each of a design's own rules comes from the design's change
table, the table that `tools/propose_check.py --strict` reads. The
prototype lists its rules with their kinds, and a unit test reads the
Propose report's change table and checks each kind, so the prototype
cannot drift from the design. The node-check point serves a design that adds a
rejection to an existing construct, such as P26 on the comma and P29 on
equality, which no rule override can express. It must exist before
step 3, because a point added later would need edits to the pages it
reaches.

A page module keeps public only the queries that other modules need,
and a prototype uses those queries. No page module imports
`proposals`.

```text
proposals::p22  ──alternative at jump_stmt──►  syntax (nonterminal table)
      │                                         └ statements::jumps::syntax (unchanged)
      ├──check of its Ext node──►  sem; reads statements::jumps::check::enclosing()
      │                                 (the loops and switches with their labels)
      └──gives Break(target) / Continue(target)
                                       └► machine and statements::jumps::run (unchanged)
```

### A worked example: P22, labeled `break` and `continue`

P22 (`reports/clean-room/roadmap-7/portfolio.md`) lets `break name;`
and `continue name;` act on the loop or `switch` that carries the label
`name`. X24 (`reports/clean-room/X24/report.md`) gives its design and
its change table. Today JMP-17 rejects both forms through the grammar,
because `jump_stmt` has no place for a label after `break` or
`continue` (TR-12).

The prototype `src/proposals/p22.rs` has two parts:

1. **Syntax.** It adds the alternatives `"break" identifier ";"` and
   `"continue" identifier ";"` to `jump_stmt` (X24's amended
   `jump_stmt`, with JMP-17 withdrawn). The parser tries them at the
   start of `jump_stmt` and makes a `jump_stmt` node with `ext` set to
   P22.
2. **Check.** A label marks the statement after it, also through
   further labels, so the loops and switches carry the labels that mark
   them (`statements::jumps::check::enclosing()`, the stack that JMP-1
   and JMP-2 use). The check finds the innermost enclosing loop or
   `switch` that the label marks. It reports X24:new-1 when no such
   statement exists in the jump's own function: a label on a block, on a
   sibling loop, at the end of a block, or in the enclosing function of
   a lambda body (X24's JMP-14 amendment gives the same-function
   condition). It reports X24:new-2 when the label of a `continue` marks
   a `switch`. Otherwise it gives an ordinary `Break(target)` or
   `Continue(target)`.

**Run.** X24:new-3 is the run-time rule, and it needs no code: the
machine passes a `Break` or `Continue` to the statement it names, ends
the objects of each block that the jump leaves (JMP-9), and goes on with
a loop as after a normal pass (LOOP-9). This holds for the variables of
a statement's own scope (a `for` initializer, a condition, a range
`for`'s hidden variables) too: item F7 (b511838d) answered X24-1, and
OBJ-3, LOOP-3, and RFOR-4 now end them after the body's objects on any
exit. Today the machine ends them with the block of their statement,
and no owned test checks the order yet (their tests print from
destructors, which need build step 4).

| Rule (X24 section 2) | Kind of change | Where the prototype acts |
|---|---|---|
| `jump_stmt` (Syntax), JMP-17 (withdrawn) | the grammar admits a label after `break` and `continue` | the alternative at `jump_stmt` |
| X24:new-1 (new, `reject`) | the label marks a loop or `switch` that encloses the jump in its own function | the check |
| X24:new-2 (new, `reject`) | the label of a `continue` marks a loop | the check |
| JMP-14 (amended) | a labeled jump names a label of its own function; a lambda body is a separate function | the check, through X24:new-1 |
| X24:new-3 (new, `required`), JMP-1, JMP-2 (amended) | `break L` ends the statement `L` marks; `continue L` ends the pass of its loop | the check's target; the machine unchanged |
| JMP-9, LOOP-9, LOOP-1, LOOP-2, SW-1, CE-17 (unchanged, state effect) | none | the machine and constant evaluation, unchanged |
| OBJ-3, LOOP-3 (amended), RFOR-4 | the end of a statement's own variables after any jump | answered by F7 (b511838d) |
| DEP-30 (X24's row says unchanged; RV10's repairs, 88d14ea2, added the jump conditions to DEP-30's list, Q-0451) | X24:new-1 and X24:new-2 in a template that no specialization uses fall under DEP-30, as X24 section 1.8's alternative says | the definition-check point, step 5 |

The files that change are `src/proposals/p22.rs` and one line of
`src/proposals/mod.rs`. `statements::jumps` does not change, because
its `Break` and `Continue` already name their target (section 2).

The impact count comes from the harness. `tidal-next conform ../tests`
runs with and without `--proposal P22`, and the tests whose status
differs are the "Tests affected" column of the change table. At the end
of item RW2 the base, the tests that `tidal-next` passes without the
flag, is 1819 of 7148 tests and 1 of the 178 program tests. Exactly the
two tests that JMP-17 cites change, `statements/jumps/break-with-label.cpp`
and `continue-with-label.cpp`, from a pass to an accept that their
`reject` expectation counts as a missed reject, and `f()` gives 5 and 6
in them as X24 says. No program changes in the base. Until step 8 the
base is smaller than the suite, so the count is a lower bound.

X24 section 2 names 17 new tests and 2 changed ones for the prototype.
No directory beside the Propose report holds them yet
(`reports/clean-room/X24/tests/`, with `X24:new-n` IDs, which the harness
would take under the flag). The prototype's unit tests hold X24's
examples meanwhile: `find` of section 1.1 as a constant, `sum_rows` as a
run, the chain of section 1.3, and the rejected forms of X24:new-1 and
X24:new-2.

A prototype of X22's checked marker uses the fifth point. It adds one
check to the definition checks, as the designer view of roadmap round
12 (item 8) describes, and it adds a rule override for the marker.

## 8. What carries over from `frontend/src`

The old front end's files, by what the rewrite does with them.
"Moved" means the code moves with small changes; build step 3 wrote `sem` and `machine` new (report-rw3), so the rows that name them give the plan, and the new code carries over no file of `interp/`. "Rewritten" means the
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

- the `next:` lines of `frontend_check.py`, and the parse count's
  disagreements by cause,
- the owned tests by page,
- the marked tests that remain, by model section,
- the rule IDs that a module names outside its own page, which should
  be none.

## 10. Open points

- The crate's place and the binary's name (`frontend/next/`,
  `tidal-next`) are provisional.
- The parser is written by hand from the Syntax segments, one function
  per production. Decision 0035 also allows a generated parser. The
  unit test that each production has one function, and that a function
  documented as a production is named after it, would hold either way.
- The oracle's questions may not cover every ambiguity of the grammar.
  A tree with both readings is the fallback for a point the oracle
  cannot answer when the parser reaches it.
- A member name followed by `<` after `.` or `->` with no `template`
  is a template-id only when the object's type is not dependent and its
  class has the member template (DEP-24). The parser has no types, so
  the step 2 oracle answers "a member template of this name is declared
  in some class". Analysis can answer the question when the object's
  type is known, as the parse of a declaration waits for its oracle.
- The step 2 oracle does not see through an alias template to the
  current instantiation, and it reads a template-id whose arguments are
  value-dependent only through `sizeof` as dependent (model section 1).
  The parse count lists the tests these touch.
- Analysis runs after the whole unit is parsed with the step 2 oracle
  (section 2). The alternation of parser and analysis, and analysis as
  the oracle, come with the class and template pages.
- A page's `check.rs` part names, as the rule of a failed conversion or
  initialization, the rule of the context, which can be another page's
  (such as INIT-7 or LIST-2 in a return statement). At the end of item
  RW2, 64 rule IDs in the `check.rs` and `run.rs` parts belong to
  another page by the prefixes of section 3. Passing the rule from the
  caller, as `copy_init` takes its rule, would bring the count to none.
- The parser keeps member function bodies as token ranges until the
  outermost class is complete (NAM-6). Default arguments and default
  member initializers are parsed in place.
- The parse count judges a `reject` test by the old front end's first
  error. Where the grammar does not derive the form and the old front
  end names a more specific rule first (KW-1, DEL-3 with another rule,
  register), the new parser's stop is a disagreement; where the grammar
  derives the form and another rule rejects it (DEL-3, DTY-5, DEP-26),
  the old front end's TR-12 makes the tree a disagreement. The count of
  reject tests that stop on the old first error's line separates the
  first kind.
- The unit of takeover after step 7 is the page. A page whose tests the
  new front end passes in part could pass test by test instead.
- The first-error target for the new code is open. The old front end
  names a cited or linked rule first in 77% of its passing rejects
  (report 25). A target at or above that rate for owned tests fits the
  implementer view of roadmap round 12 (process change 5).
- `TEMPLATE-MODEL.md` section 8 lists Q-0427, Q-0432, Q-0433, Q-0434,
  Q-0441, and RV10-R-1 and RV10-R-3, which item F7 (b511838d) answered. An answer that changes a rule there changes the
  module that section 4 of this document names for it.
- The kind of each rule comes from `specification/` at run time, as in
  the old harness. A table built into the crate would make the binary
  stand alone and need a rebuild for each text change.
- A prototype runs under a run-time flag. Whether prototypes stay in
  the crate after their exercise, or move to a branch, is open.
- A done design that changes run-time rules (X25 on UNI-14 and LSUP-10,
  X1, X3, X4) would need a sixth point, a run override by rule ID in the
  machine and in constant evaluation. No ready proposal needs it.
- While both front ends exist, a fix in the old front end for a test
  the new one owns changes no result. Rounds then fix owned tests in the
  new code only. A round that changes `lexer.rs`, `diag.rs`, `main.rs`,
  or `conform.rs` makes the same change in the new crate, and the lex
  count shows a lexer change that reaches one copy only.
