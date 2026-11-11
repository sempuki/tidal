# Design of the current front end

This document describes the front end in `frontend/src/` as it is after
front-end round 24 (`53577f27`, the last commit that changed `src/`).
It names the stages, the data that each stage passes on, and the files
that hold each page's rules. Decision 0060 asks the rewrite for a
design document of its own. This document is the description of what
the rewrite replaces. `README.md` gives the use of the binary, the
harness, and some procedures (the conditional operator, templates,
bytes), and this document does not repeat them.

The source is 51375 lines of Rust in 28 files. No file has unit tests.
The conformance harness (`src/conform.rs`, `tidal-fe conform`) is the
only test.

## Stages

```text
source file
   │  lexer.rs   lex_file: the preprocessing slice and the tokens
   ▼
Vec<Token>
   │  parser::P  parse_translation_unit (decl2.rs): parsing and
   │             semantic analysis in one pass over the tokens
   │     ├── templ.rs    replay: the tokens of a template, parsed again
   │     │               with its parameters bound
   │     ├── cons.rs     constraint checks, by replay of each atom
   │     └── consteval.rs  constant evaluation, on the interpreter
   ▼
P: entities, scopes, typed trees, templates, diagnostics
   │  main.rs    outcome_of: accept, reject, or unsupported
   ▼  (tidal-fe run only, after each unit is accepted)
interp/run.rs   run_explored: link the units, initialize the statics,
   │            run main, then run again at each choice point
   ▼
RunOutcome: result, unspecified, undefined, terminated, unsupported
```

| Stage | Entry point | Files | Takes | Gives |
|---|---|---|---|---|
| Lexing | `lexer::lex_file(src, path, &mut Diags)` | `lexer.rs` | the source text and its path | `Vec<Token>`, and diagnostics for literals (PRIM) and includes (INC) |
| Parsing and semantic analysis | `P::new(toks, diags)`, then `P::parse_translation_unit()` | every file outside `interp/`, except `conform.rs` and `main.rs` | the tokens | one `P` |
| Constant evaluation | `P::const_eval`, `const_int`, `const_eval_bool`, `const_addr` | `consteval.rs`, and `interp/` with `constant` set | a typed tree (`ast::Expr`) and the `P` that holds it | a value, or a `Stop` that names the rule that failed |
| Interpretation | `interp::run::run_explored(&[&P])` | `interp/` | every accepted unit | a `RunOutcome`, the other outcomes, and a note when the exploration is partial |

### Lexing

`lex_file` reads the file and the files it includes with `#include
"file"`. Its preprocessing is a slice: the conditional groups of the
ODR page's grammar, object-like `#define` and `#undef`, and `#pragma
once` (`Pp`, `Cond`). It then makes the tokens. A token is a kind
(`TK`: identifier, punctuator, integer, floating, character, string,
end of file), its text, its line, and, for a literal, its value
(`Lit`). The literal checks name PRIM rules, and the alternative
operator spellings follow LOG-13. Each token vector ends with an
end-of-file token.

The token vector is the only form of the program before parsing. No
later stage reads the source text.

### Parsing and semantic analysis

One pass over the tokens parses the program and checks it. There is no
syntax tree between the two. Each parse function declares entities,
looks up names, forms types, converts operands, chooses overloads, and
reports errors as it goes. The functions build typed trees
(`ast::Expr`, `ast::Stmt`) for the parts that later stages run:
function bodies and initializers.

`parser::P` holds the whole state of a unit: the token vector and the
position, the entity tables, the scope stack, the function contexts,
the template records, the diagnostics, and side tables. `P` has 395
public fields. The parse functions are methods of `P` spread over the
files of the stage, with one `impl P` block or more in each file.

A parse function returns `PR<T>`, which is `Result<T, PErr>`.
`parse_declaration_recover` turns a `PErr` into a TR-12 error, or into
an `unsupported` note when the front end knows that it does not model
the construct (`parse_known_unsupported`). The scope of a failed
declaration is then marked `uncertain`, and a failed lookup there is
not an error.

The pass has three later parts:

- **Deferred bodies.** A class body records each member function body,
  default argument, and default member initializer as a `Deferred`
  (token range and scope stack). `run_deferred` parses them after the
  class is complete, so that a body sees the whole class (NAM-6). The
  operand of a member's `noexcept` is a `Deferred` too when its first
  parse needs a later member (CLS-2, EXS-1). Until then the member is
  potentially throwing.
- **Lazy bodies of instances.** A member function body of a class
  specialization waits for a use (INST-5, INST-7) in `lazy_bodies`.
  `use_fn` makes it at once for a function that constant evaluation
  can call, or else at the end of the unit (INST-31). The parameter and
  return types of such a member are made complete with the body
  (FN-12, INST-1), so the class argument can be incomplete when the
  class is made.
- **The end of the unit.** `finish_lazy` makes the definitions that a
  use needs, with their diagnostics, then the other definitions for
  the interpreter, with no diagnostics. `check_main` and
  `check_internal_uses` follow.

Tentative parsing mutes the diagnostics (`Diags::muted`). The muted
errors are counted, and the count tells the template code that a
substitution failed (see "The template record").

| File | Lines | What it holds |
|---|---|---|
| `parser.rs` | 2396 | `P` and its records: `Decl`, `DK`, `VarInfo`, `FuncInfo`, `FieldInfo`, `ClassInfo`, `EnumInfo`, `ScopeInfo`, `FnCtx`; scopes; unqualified, qualified, and member lookup |
| `decl.rs` | 3154 | decl-specifiers (`DeclSpec`), declarators (`Declarator`, `DOp`), type-ids, the current-instantiation test of DEP-15 |
| `decl2.rs` | 7190 | declarations at namespace, class, and block scope; functions and their definitions; template heads; explicit specializations and instantiations; deferred bodies; `parse_translation_unit` |
| `class.rs` | 1615 | class and enumeration specifiers, base lists, virtual functions |
| `stmt.rs` | 1896 | statements, jumps, `switch`, range-`for`, statement attributes |
| `expr.rs` | 7726 | expression parsing by precedence, the built-in operators, member access, lambdas, `new` and `delete`, the conditional operator |
| `expr2.rs` | 2829 | calls, casts, initialization in each form, list initialization, `auto` deduction |
| `ics.rs` | 1582 | implicit conversion sequences (`Ics`) and their ranking; `ics_select` for calls |
| `ovl.rs` | 1437 | overload resolution for operators, constructors, calls on class objects, and the address of an overload set |
| `special.rs` | 1083 | implicit special member functions: declared, deleted, trivial |
| `layout.rs` | 421 | class layout on the matrix targets |
| `dcmp.rs` | 313 | bodies of defaulted comparisons, made at the first use |
| `templ.rs` | 4310 | the template record, argument lists, replay, selection of specializations, deduction, partial ordering, packs, lazy bodies |
| `cons.rs` | 1594 | concepts and constraints: normal forms, satisfaction, subsumption, requires expressions, abbreviated templates |
| `ctad.rs` | 1153 | class template argument deduction through guides |
| `types.rs` | 529 | `Ty`, `QTy`, `FnTy`, the dependent forms `Dep`, `TArg`, `TVal`, `CVal`, sizes and alignments |
| `ast.rs` | 408 | `Expr`, `EK`, `Conv`, `Cat`, `Stmt`, `Handler` |
| `diag.rs` | 59 | `Diag`, `Diags`, `Severity` |

### Constant evaluation

Constant evaluation runs the interpreter's machine (`interp::Machine`)
with `constant` set, over the trees of the unit being parsed. The
machine then stops with `Stop::Limit` at each operation that the CE
rules forbid, and with `Stop::Undefined` at undefined behavior. Either
stop makes the expression not constant, and the error names the rule
that needs the constant and the rule that the evaluation broke
(`Diag::rule`, `Diag::cause`).

The parse functions call constant evaluation where a rule needs a
constant, such as a `static_assert` (`decl2.rs`), an array bound
(`decl.rs`), a `case` label (`stmt.rs`), an enumerator value
(`class.rs`), a constant template argument (`templ.rs`), and an atomic
constraint (`cons.rs`). Eight files call it. The parse of a pattern
does not call it for a `static_assert` (SA-5). Constant evaluation
reads the entity tables of `P` and changes nothing in them.
A function that constant evaluation calls must have its body, so
`use_fn` makes such a body at once (INST-31).

### Interpretation

`tidal-fe run` checks each unit, and runs the program only when every
unit is accepted. `run_explored` makes a `Machine` over all the units
(`ps`), links each declaration to its definition in another unit
(`link`, TR-4 and the LNK rules), runs the static initialization and
`main` (`run_all`), and ends the static objects. The first run takes
the first option at each choice point. Further runs take the other
options at SEQ-1, SEQ-8, PRIM-9, CMP-8, CMP-20, and MPTR-7, up to 64
runs (`RUN_BOUND`, decision 0048).

The interpreter does no lookup and no overload resolution. A call in
the tree names its callee (`EK::Call` with a `DeclId`), a conversion
is a node of its own (`EK::Conv` with a `Conv` kind), and a
constructor call is `EK::Construct`. The interpreter does choose the
final overrider of a virtual call from the dynamic type.

| File | Lines | What it holds |
|---|---|---|
| `interp/mod.rs` | 1479 | `Machine`, `Val`, `Ptr`, `Obj`, `Frame`, `Cleanup`, `Stop`, `Choice`, the catch stack |
| `interp/expr.rs` | 4815 | expressions: places, values, conversions, operators, calls, initialization, `new`, `delete`, `throw`, `dynamic_cast` |
| `interp/stmt.rs` | 996 | statements, blocks and their cleanups, `goto`, `switch`, try blocks, static locals |
| `interp/bytes.rs` | 1156 | the byte view of objects, addresses, `reinterpret_cast`, the access check of STG-14 |
| `interp/run.rs` | 538 | linking, the run of a program, the exploration of choice points |
| `interp/f80.rs` | 360 | the x87 format of `long double` |
| `consteval.rs` | 310 | constant evaluation as a client of the machine |

## The data between the stages

### Tokens

```text
Token { kind: TK, text: String, line: u32, lit: Option<Lit> }
Lit   = Int { value, decimal, unsigned, size }
      | Float { value, suffix } | Char { prefix, value, multi }
      | Str { prefix, units } | UserDefined { suffix, base, raw }
```

A template keeps its tokens as an `Rc<Vec<Token>>` copy of its range,
with an end-of-file token added (`P::copy_toks`). A generic lambda and
a constraint atom keep their tokens in the same way.

### Entities and scopes

Every entity is a `Decl` in the vector `P::decls`, and a `DeclId` is
its index. Classes, enumerations, and scopes have vectors of their
own, and a `usize` index names each.

```text
P.decls[DeclId]   Decl { name, k: DK, line, scope }
                  DK = Var(VarInfo) | Func(FuncInfo) | Field(FieldInfo)
                     | Type(QTy) | Class(c) | Enum(e) | Enumerator(e, value)
                     | Namespace(s) | Template(TemplKind) | TypeParam
P.classes[c]      ClassInfo { name, key, scope, complete, fields, bases,
                              ctors, dtor, has_virtual, template, opaque, ... }
P.enums[e]        EnumInfo { name, scoped, fixed, underlying, enumerators, ... }
P.scopes[s]       ScopeInfo { kind: SK, names: name -> [DeclId], parent,
                              using_ns, uncertain }
                  SK = Namespace | Class(c) | Block | Params | Template | Enum(e)
P.stack           the open scopes, innermost last
P.fns             FnCtx of each open function: return type, labels,
                  loops, switches, lambda captures, ...
```

A name maps to a list of `DeclId`s, so an overload set is the list.
Lookup (`parser.rs`) walks `P.stack` outward, with using-directives
(NAM-23), inline namespaces, and the bases of a class (NAM-17). A class
scope that has an unknown base, or a declaration that failed, makes a
failed lookup uncertain.

A type is a `QTy`: a `Ty` with `const` and `volatile`. `Ty` has a
variant for each fundamental type and for pointer, reference, array,
function (`FnTy`), enumeration, class, and member pointer. Two variants
stand for what the front end cannot name:

- `Ty::Unknown`: a dependent type that has no structured form, or the
  type of a construct the front end does not understand. No check
  fires on an unknown type.
- `Ty::Dep(Rc<Dep>)`: a dependent type in the form that deduction takes
  apart (DED-P3): a parameter by depth and index, a template-id with
  dependent arguments, an array with a dependent bound, a member
  pointer with a dependent class, or `Opaque`.

Facts that are not in these records sit in side tables of `P` keyed by
`DeclId` or class index: access (`member_access`), attributes
(`entity_attrs`, `class_attrs`), friends (`friend_sets`,
`hidden_friends`), the captures of each closure (`closure_caps`),
virt-specifiers, linkage, and others. Other fields of `P` are parse
state: flags that a caller sets for the next call, such as
`implicit_tn` (DEP-20) or `contextual` (CVF-3), and the parse state
that a replay saves and puts back.

### Typed trees

```text
Expr { k: EK, ty: QTy, cat: Cat, line, paren, qualified }
EK   = Int | Float | Str | Var(id) | Func(id) | Overloads([id])
     | Unary | Binary | Assign | Cond | Comma | Call(callee, args, id)
     | Conv(Conv, e) | Member(e, id) | Index | InitList | Construct(id, args)
     | New { .. } | Delete | Throw | Closure | Destroy | Reinterpret
     | MemberPtr | PtrMem | TemplId | Unknown(why) | ...
Stmt = Expr | Decl([id]) | Block | If | While | Do | For | Switch | Case
     | Return | Goto | Label | Try(body, handlers) | Unsupported(why) | ...
```

A tree is fully resolved: each node has its type and value category,
each conversion is explicit, and each call names its function. A
function's body is `FuncInfo::body: Option<Rc<Stmt>>`, and a
variable's initializer is `VarInfo::init`. An `EK::Unknown` or a
`Stmt::Unsupported` node marks a part that the front end could not
form. The interpreter stops as unsupported when it reaches one.

### Diagnostics

```text
Diag { sev: Error | Unsupported | Warning, rule: Option<&str>,
       cause: Option<&str>, msg, line }
```

`main::outcome_of` reads the list: an error rejects, else an
`unsupported` note makes the outcome unsupported, else the unit is
accepted. Errors of the body of a member of an instance wait in
`body_errors` until a use needs the body (INST-7).

### The template record

Each template is a `TemplInfo` in `P::templs`. `decl_tid` and
`class_tid` map a declaration or a class to its template.

```text
TemplInfo { name, kind: Class | Func | Var | Alias | Concept,
            params: [TParam], depth, src: Option<Source>,
            explicit: [(args, Inst)],        explicit specializations
            partials: [Partial],             partial specializations
            insts:    [(args, Inst, partial)], specializations made
            members:  [Member],              members defined outside the class
            cons: Option<CNorm>, ... }
Source    { toks: Rc<[Token]>, stack, ctx, access,
            limit,           declarations visible at the definition (DEP-5)
            vid, base }      where the tokens came from
TParam    { name, kind: Type | Const(QTy) | Auto | Templ([TParam]),
            pack, default: Option<tokens>, ... }
Inst      = Class(c) | Func(id) | Var(id) | Type(QTy) | Busy | Fail
```

A template has two forms in the front end:

- **The pattern.** The declaration is parsed once, with each parameter
  as `Ty::Dep(Param(depth, index))` or `Ty::Unknown`. The pattern's
  parse runs the checks that need no arguments, and gives the patterns
  of deduction (DED-P3), selection (TSP-14), and partial ordering
  (ORD-P2). A pattern class is a `ClassInfo` with `template` set, and
  its member bodies are parsed as patterns too.
- **A specialization.** `replay` parses the template's tokens again.
  It saves the parse state (`Saved`, 43 fields), swaps in the
  template's token vector, opens a scope of kind `SK::Template`, and
  declares each parameter there as its argument (`bind_params`). A
  type parameter becomes an alias of its argument, a constant
  parameter a `constexpr` variable with the argument's value, and a
  template template parameter a second name of its argument. The
  declaration that the parse makes is the specialization (`InstReq`,
  `InstOut`).

The argument lists are `Vec<TArg>`. A `TArg` is a type, a value
(`TVal`: a parameter, a constant `CVal`, or `Opaque`), a template, a
pack, or an expansion. `inst_of` and `spec_of` map each specialization
back to its template and arguments.

The two parses share nothing but the token positions. When the
pattern's parse learns a fact that the specialization's parse needs,
it records the fact in `pattern_facts` under the token's origin
(`vid`, index), and the replay reads it with `has_fact`. Five facts
exist: a qualified name read as a value (DEP-19), a `<` read as the
less-than operator (DEP-24), an independent call (DEP-3, DEP-5),
`template` with no list (DEP-26), and a parameter type read as a value
(DEP-19).

A function specialization is made by `func_spec`: the constraints
first (CNS-11), then a replay of the declaration with the diagnostics
muted. Any muted error during the replay makes the specialization a
substitution failure (`Inst::Fail`, DED-24). The body is replayed
later, when a use needs it (`func_body`), with its errors reported
(DED-29). A class specialization is made by `class_inst` and
`new_inst_class`, after `select_partial` chooses the explicit or
partial specialization (TSP-13 to TSP-16). Nesting stops at the TR-13
limit (`DEPTH_LIMIT`, 256 by default) or after 8000 specializations.

A constraint is a `CNorm` tree of conjunctions and disjunctions over
`Atom`s (`cons.rs`). An atom keeps its tokens, its scope stack, and its
parameter mapping. Its check replays the tokens with the mapping bound.

`frontend/TEMPLATE-MODEL.md` describes the limits of this record and
the model that the rewrite needs.

### Values and places of the interpreter

```text
Machine { ps: [&P], tu, links, constant,
          objs: [Obj], frames: [Frame], globals, blocks, temps,
          static_dtors, choices, catch_stack, ... }
Obj     { val: Val, alive, own, volatile, is_static, konst, heap, ty, tu }
Ptr     { obj, path: [usize], off, array, bytes }      a place
Val     = Int(i128) | Float(f64) | LD(F80) | Ptr(Ptr) | FnPtr(tu, id)
        | Null | Agg([Val]) | Uninit | Inactive | Void
        | NotUsable | Addr(u64) | Member(id, path)
Frame   { vars: id -> object, refs: id -> Ptr, this, func, slot, ret, ... }
Cleanup { place: Ptr, ty, item }                        an object to end
Stop    = Undefined(rule, msg) | Limit(rule, msg) | Unsupported(msg)
        | Throw(Exc) | Terminate(rule, msg) | Exit(status, statics)
```

An object is an entry of `objs`, and its value is a tree of `Val`s
that follows the type: `Agg` for a class or an array. A place is the
object and a path of subobject indexes, with an element offset for an
array pointer. A reference is a `Ptr` in the frame (`refs`). The value
view stays the record of each object. The byte view (`interp/bytes.rs`)
computes bytes from the value when a program reads them, and keeps
padding and the bytes of a scalar with no value in a shadow table.
The entities stay in each unit's `P`, so `Obj::tu` and `FnPtr` carry
the unit whose types and functions they use.

`Stop` travels up as the `Err` of `R<T>`. An exception is a
`Stop::Throw` that a handler catches. A run ends with the value of
`main`, or with the stop that ended it (`RunOutcome`).

## Where each chapter's rules sit

The front end names a rule in a comment beside the code that follows
it, and in the diagnostic that the code reports. The table counts the
rule IDs of each page in each file, comments and diagnostics together,
and lists the four files with the most. The prefix of each page comes
from `specification/rules.md`.

| Chapter | Files that name most of its rules |
|---|---|
| foundations | `interp/` for objects, storage, order, and constant evaluation; `parser.rs`, `expr.rs`, `decl.rs`, `decl2.rs` for names |
| expressions | `expr.rs` for parsing and types; `interp/expr.rs` for the run; `expr2.rs` for casts |
| statements | `stmt.rs`; `interp/stmt.rs` |
| declarations | `decl2.rs`, `decl.rs`, `expr2.rs` (initialization, `auto`) |
| classes | `decl2.rs`, `class.rs`, `special.rs`, `expr.rs`; `ovl.rs` and `dcmp.rs` for comparisons |
| overloading | `ics.rs`, `ovl.rs`, `expr2.rs` |
| templates | `templ.rs`, `decl2.rs`, `cons.rs`, `ctad.rs`; `expr.rs`, `decl.rs`, and `parser.rs` for dependent names |
| exceptions | `decl2.rs` for handlers; `interp/` for throw and unwinding |

The front end names rules from 84 of the 85 pages. The keywords page
(KW) is the one it does not name. A page's rules sit in 6 files at the
median, and one file names the rules of many pages:

| File | Pages whose rules it names |
|---|---|
| `expr.rs` | 65 |
| `decl2.rs` | 52 |
| `interp/expr.rs` | 48 |
| `parser.rs` | 46 |
| `expr2.rs` | 37 |
| `decl.rs` | 33 |

The map by page, with the number of rule IDs that each file names:

| Page | Prefixes | Files |
|---|---|---|
| `foundations/constant-evaluation` | CE | `interp/expr.rs` 44, `interp/mod.rs` 23, `consteval.rs` 17, `interp/stmt.rs` 9, and 9 more |
| `foundations/conversions` | CONV | `expr.rs` 26, `interp/expr.rs` 9, `ics.rs` 3, `types.rs` 2 |
| `foundations/evaluation-order` | SEQ | `interp/expr.rs` 14, `interp/mod.rs` 6, `interp/run.rs` 2, `stmt.rs` 2 |
| `foundations/keywords` | KW | none |
| `foundations/language-support` | LSUP | `interp/expr.rs` 10, `expr2.rs` 6, `dcmp.rs` 4, `interp/bytes.rs` 3, and 6 more |
| `foundations/names` | NAM | `parser.rs` 34, `expr.rs` 32, `decl.rs` 24, `decl2.rs` 22, and 4 more |
| `foundations/objects` | OBJ | `interp/mod.rs` 19, `interp/expr.rs` 11, `interp/stmt.rs` 8, `interp/run.rs` 5, and 1 more |
| `foundations/odr` | INC, ODR | `decl2.rs` 13, `lexer.rs` 9, `expr.rs` 4, `interp/expr.rs` 2, and 2 more |
| `foundations/storage` | STG | `interp/expr.rs` 54, `interp/bytes.rs` 38, `interp/mod.rs` 18, `expr.rs` 4, and 3 more |
| `foundations/translation` | TR | `decl2.rs` 9, `interp/run.rs` 7, `expr.rs` 4, `interp/mod.rs` 4, and 11 more |
| `foundations/types` | TYP | `interp/bytes.rs` 4, `types.rs` 3, `decl.rs` 2, `ast.rs` 1, and 4 more |
| `foundations/values` | VC | `expr.rs` 7, `interp/expr.rs` 6, `interp/mod.rs` 5, `interp/stmt.rs` 1 |
| `expressions/additive` | ADD | `interp/expr.rs` 17, `expr.rs` 7, `interp/bytes.rs` 3, `interp/mod.rs` 3 |
| `expressions/assignment` | ASN | `expr.rs` 12 |
| `expressions/bitwise` | BIT | `expr.rs` 7 |
| `expressions/casts` | CAST | `expr2.rs` 20, `interp/bytes.rs` 15, `interp/mod.rs` 5, `ast.rs` 3, and 3 more |
| `expressions/comma` | COMMA | `expr.rs` 4, `stmt.rs` 3 |
| `expressions/comparison` | CMP | `expr.rs` 43, `interp/expr.rs` 11, `interp/bytes.rs` 2, `interp/run.rs` 2, and 2 more |
| `expressions/conditional` | COND | `expr.rs` 46, `interp/expr.rs` 1 |
| `expressions/decltype` | DTY, UNEV | `decl.rs` 2, `interp/expr.rs` 1 |
| `expressions/lambdas` | LAM | `expr.rs` 57, `parser.rs` 8, `interp/expr.rs` 6, `special.rs` 2, and 5 more |
| `expressions/logical` | LOG | `expr.rs` 9, `interp/expr.rs` 2, `lexer.rs` 1 |
| `expressions/member-pointers` | MPTR | `expr.rs` 17, `interp/expr.rs` 12, `expr2.rs` 5, `decl.rs` 4, and 2 more |
| `expressions/multiplicative` | MUL | `interp/expr.rs` 14, `expr.rs` 4, `interp/bytes.rs` 2, `interp/f80.rs` 1 |
| `expressions/new-delete` | NEW | `expr.rs` 71, `interp/expr.rs` 56, `ast.rs` 9, `interp/mod.rs` 5, and 3 more |
| `expressions/postfix` | PFX | `expr.rs` 15, `interp/expr.rs` 13, `expr2.rs` 11, `interp/mod.rs` 2 |
| `expressions/primary` | PRIM | `expr.rs` 32, `lexer.rs` 31, `templ.rs` 7, `interp/expr.rs` 6, and 3 more |
| `expressions/shift` | SHF | `expr.rs` 6, `interp/expr.rs` 3 |
| `expressions/unary` | UN | `expr.rs` 28, `interp/expr.rs` 9, `interp/mod.rs` 2, `expr2.rs` 1 |
| `statements/blocks` | STMT | `stmt.rs` 6, `decl2.rs` 1 |
| `statements/if` | IF | `stmt.rs` 34, `interp/stmt.rs` 8, `decl2.rs` 4, `interp/run.rs` 1, and 1 more |
| `statements/jumps` | JMP | `stmt.rs` 19, `decl2.rs` 3, `expr.rs` 2, `special.rs` 2, and 1 more |
| `statements/loops` | LOOP | `stmt.rs` 12, `interp/stmt.rs` 1 |
| `statements/range-for` | RFOR | `stmt.rs` 17, `interp/stmt.rs` 1, `parser.rs` 1 |
| `statements/switch` | SW | `stmt.rs` 31, `interp/stmt.rs` 2, `decl2.rs` 1, `expr.rs` 1, and 1 more |
| `declarations/alignment` | ALN | `decl.rs` 15, `decl2.rs` 8, `class.rs` 6, `interp/bytes.rs` 1, and 3 more |
| `declarations/attributes` | ATTR | `decl2.rs` 22, `stmt.rs` 17, `decl.rs` 12, `parser.rs` 7, and 5 more |
| `declarations/auto` | AUTO | `expr2.rs` 27, `decl2.rs` 12, `decl.rs` 4, `expr.rs` 2, and 2 more |
| `declarations/constexpr` | CEV, CINIT, CX | `decl2.rs` 28, `expr2.rs` 6, `consteval.rs` 6, `expr.rs` 3, and 2 more |
| `declarations/declarators` | CV, DCL | `decl.rs` 40, `decl2.rs` 15, `expr.rs` 9, `interp/mod.rs` 4, and 8 more |
| `declarations/enumerations` | ENUM | `class.rs` 22, `expr.rs` 11, `decl2.rs` 5, `interp/expr.rs` 5, and 2 more |
| `declarations/exception-specifications` | EXS | `decl.rs` 7, `special.rs` 5, `expr.rs` 4, `class.rs` 2, and 1 more |
| `declarations/functions/default-arguments` | DEFARG | `decl2.rs` 21, `decl.rs` 5, `expr2.rs` 2, `parser.rs` 2, and 1 more |
| `declarations/functions/definitions` | FN | `decl2.rs` 24, `expr.rs` 2, `interp/expr.rs` 2, `interp/run.rs` 1, and 1 more |
| `declarations/functions/deleted` | DEL | `decl2.rs` 7, `expr.rs` 6, `expr2.rs` 4, `ovl.rs` 3, and 3 more |
| `declarations/functions/return-deduction` | RET | `decl2.rs` 6, `stmt.rs` 5, `expr.rs` 4 |
| `declarations/initialization/forms` | INIT | `expr2.rs` 15, `interp/expr.rs` 3, `interp/mod.rs` 3, `expr.rs` 2, and 4 more |
| `declarations/initialization/list` | LIST | `expr2.rs` 41, `expr.rs` 4, `ics.rs` 3, `interp/expr.rs` 2, and 2 more |
| `declarations/initialization/references` | REF | `expr.rs` 15, `expr2.rs` 12, `interp/expr.rs` 8, `interp/stmt.rs` 4, and 3 more |
| `declarations/initialization/static` | SINIT | `interp/run.rs` 4, `decl2.rs` 1, `interp/stmt.rs` 1, `parser.rs` 1 |
| `declarations/inline` | INL | `decl2.rs` 9, `decl.rs` 2, `parser.rs` 1 |
| `declarations/linkage-specifications` | LSP | `decl2.rs` 17, `parser.rs` 4, `interp/run.rs` 3, `ics.rs` 1, and 1 more |
| `declarations/namespaces` | NS | `decl2.rs` 27, `class.rs` 3, `parser.rs` 3 |
| `declarations/static-assert` | SA | `decl2.rs` 5, `expr.rs` 1, `stmt.rs` 1 |
| `declarations/storage-linkage` | LNK, STO | `decl2.rs` 21, `interp/expr.rs` 4, `interp/stmt.rs` 3, `interp/mod.rs` 2, and 3 more |
| `declarations/structured-bindings` | SB | `decl2.rs` 47, `expr.rs` 2, `interp/expr.rs` 2, `stmt.rs` 2, and 1 more |
| `declarations/type-names` | TYN | `class.rs` 21, `decl2.rs` 17, `decl.rs` 3, `expr.rs` 3, and 3 more |
| `declarations/volatile` | VOL | `expr.rs` 5, `interp/bytes.rs` 3, `decl2.rs` 2, `special.rs` 2, and 1 more |
| `classes/access` | ACC | `expr.rs` 24, `parser.rs` 19, `decl2.rs` 13, `expr2.rs` 10, and 5 more |
| `classes/comparisons` | DCMP | `ovl.rs` 14, `dcmp.rs` 11, `decl2.rs` 5, `interp/expr.rs` 2, and 1 more |
| `classes/constructors` | CTOR, DTOR | `decl2.rs` 42, `expr.rs` 27, `special.rs` 21, `interp/expr.rs` 10, and 9 more |
| `classes/conversion-functions` | CVF | `decl2.rs` 10, `expr2.rs` 7, `expr.rs` 6, `parser.rs` 3, and 3 more |
| `classes/copy-move` | CPY | `special.rs` 46, `expr.rs` 8, `interp/expr.rs` 5, `ics.rs` 3, and 5 more |
| `classes/derived` | DER | `class.rs` 13, `decl2.rs` 13, `expr2.rs` 8, `parser.rs` 7, and 6 more |
| `classes/initializer-lists` | ILC | `expr2.rs` 11, `ovl.rs` 8, `ics.rs` 2, `ctad.rs` 1, and 2 more |
| `classes/members` | CLS | `decl2.rs` 45, `layout.rs` 13, `expr.rs` 8, `decl.rs` 5, and 8 more |
| `classes/operators` | OPR | `expr.rs` 18, `decl2.rs` 16, `ovl.rs` 6, `lexer.rs` 4, and 3 more |
| `classes/unions` | BF, UNI | `decl2.rs` 29, `interp/expr.rs` 24, `expr.rs` 12, `special.rs` 12, and 8 more |
| `classes/virtual` | VIRT | `class.rs` 24, `decl2.rs` 21, `expr2.rs` 14, `interp/expr.rs` 12, and 2 more |
| `overloading/conversion-sequences` | ICS | `ics.rs` 94, `expr2.rs` 19, `ovl.rs` 10, `expr.rs` 5, and 3 more |
| `overloading/overload-resolution` | OVL | `ovl.rs` 65, `expr2.rs` 48, `ics.rs` 40, `expr.rs` 23, and 7 more |
| `templates/arguments` | TARG | `templ.rs` 60, `decl.rs` 6, `expr.rs` 3, `cons.rs` 2, and 4 more |
| `templates/class-template-deduction` | CTAD | `ctad.rs` 59, `decl2.rs` 8, `decl.rs` 3, `expr.rs` 3, and 3 more |
| `templates/concepts` | CPT | `cons.rs` 40, `decl2.rs` 27, `decl.rs` 8, `expr.rs` 5, and 1 more |
| `templates/constraints` | CNS | `cons.rs` 47, `decl2.rs` 24, `templ.rs` 9, `expr.rs` 6, and 4 more |
| `templates/declarations` | TPL | `decl2.rs` 72, `templ.rs` 12, `parser.rs` 7, `class.rs` 6, and 4 more |
| `templates/deduction` | DED | `templ.rs` 61, `expr.rs` 15, `decl2.rs` 13, `expr2.rs` 9, and 8 more |
| `templates/dependent-names` | DEP | `expr.rs` 37, `decl.rs` 31, `parser.rs` 17, `templ.rs` 11, and 3 more |
| `templates/instantiation` | INST | `decl2.rs` 75, `templ.rs` 15, `parser.rs` 10, `expr.rs` 6, and 6 more |
| `templates/ordering` | ORD | `templ.rs` 25, `cons.rs` 3, `ovl.rs` 3, `decl2.rs` 2, and 3 more |
| `templates/packs` | PACK | `decl.rs` 14, `expr.rs` 12, `decl2.rs` 9, `templ.rs` 9, and 3 more |
| `templates/specialization` | TSP | `decl2.rs` 47, `class.rs` 20, `templ.rs` 9, `decl.rs` 3, and 1 more |
| `exceptions/handlers` | TRY | `decl2.rs` 26, `interp/stmt.rs` 16, `interp/expr.rs` 4, `stmt.rs` 4, and 2 more |
| `exceptions/throw` | THR | `special.rs` 9, `interp/expr.rs` 8, `interp/stmt.rs` 6, `interp/mod.rs` 3, and 3 more |
| `exceptions/unwinding` | UNW | `interp/mod.rs` 20, `interp/expr.rs` 14, `interp/run.rs` 9, `interp/stmt.rs` 4, and 3 more |

## Files over 2000 lines

Decision 0060 asks the rewrite for no file longer than about 2000
lines. Seven files are longer now:

| File | Lines | Pages whose rules it names | Chapters with the most of those pages |
|---|---|---|---|
| `expr.rs` | 7726 | 65 | declarations 16, expressions 16, templates 10, classes 9 |
| `decl2.rs` | 7190 | 52 | declarations 20, templates 10, classes 9, foundations 5 |
| `interp/expr.rs` | 4815 | 48 | expressions 14, declarations 11, foundations 9, classes 9 |
| `templ.rs` | 4310 | 22 | templates 10, expressions 4, foundations 3, classes 2 |
| `decl.rs` | 3154 | 33 | declarations 10, templates 9, classes 6, foundations 4 |
| `expr2.rs` | 2829 | 37 | declarations 10, classes 10, expressions 6, foundations 5 |
| `parser.rs` | 2396 | 46 | declarations 14, templates 11, classes 10, foundations 4 |

The splits `decl.rs` and `decl2.rs`, and `expr.rs` and `expr2.rs`, are
by size. A page's rules do not follow the split.

## Facts for the rewrite

- **The grammar.** The parser predates the one grammar of decision
  0040, and no parse function names a production. The header of
  `decl.rs` still says that the grammar has no production for
  `decl_specifiers` or `type_id`, which is no longer so.
  `parse_translation_unit` matches the start symbol
  `translation_unit` of `specification/declarations/index.md`. The
  productions that the start symbol does not reach carry a
  `tidal-grammar` mark (`documentation/style-guide.md`, section 8).
- **Disambiguation.** The parser decides between a declaration and an
  expression (STMT-6) by a look-ahead over the tokens
  (`is_decl_start`, `decl_after_type` in `stmt.rs`), which asks lookup
  whether a name is a type. Other choices use tentative parses with
  the diagnostics muted. Both need to know, during the parse, whether a
  name is a type or a template.
- **Unknown types silence checks.** A check does not fire on
  `Ty::Unknown`, and an uncertain scope does not report a failed
  lookup. These two keep the front end from false rejects in code it
  does not model. They also hide missed rejects, and the harness sees
  such a test as `MISSED-REJECT` with no message.
- **One state for everything.** Parsing, checking, template replay,
  and constraint checks share `P`. A replay saves and restores 43 of
  its fields. A field that a replay should save and does not is a
  fault that only some programs show.
- **Unit tests.** None. `tidal-fe dump FILE [NAME]` prints the trees of
  one function, as a development aid.
- **The run.** `tidal-fe run` runs on a thread with a 512 MiB stack,
  because the interpreter recurses on the program's own recursion.
