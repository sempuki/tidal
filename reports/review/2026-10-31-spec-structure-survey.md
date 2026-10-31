<!--
Survey summary, 2026-10-31: how other language specifications present
principles and relate them to rules, with advice for Tidal.
Written by: a reader agent (a source reader under AGENTS.md) that read
the public specifications it names. It did not read the ISO C++ draft or
cppreference for this task and did not write or change any file in this
repository; the writer copied this summary here.
Checked before use: tools/similarity.py against cplusplus/draft n4950
(0 flagged), no code, no quotes, no section names.
-->

# How other language specifications put principles ahead of rules

Role of the author: source reader. This report summarizes, in the author's own words, how a range of language specifications introduce their underlying model and relate that model to detailed rules, then advises Tidal. It contains no quoted text, no code, and no section identifiers from any source.

## What the complaint is about

The objection is about causation. A reader of a Tidal page sees facts that implementations obey, each with tests, but cannot tell which facts are primitive and which are consequences. A good manual lets a reader predict a rule they have not read yet. To do that it names the few concepts the rules are built from (objects, values, scopes, lifetimes, types, evaluation order) and states the general laws about those concepts. Most itemized rules then become a law applied to a construct, and the reader can check them instead of memorizing them.

The surveyed specifications fall into three families: narrative references with a strong conceptual front section (Go, Java, Rust Reference, Dafny, Lean), references with a fixed clause skeleton (Ada, Ferrocene, C, ECMAScript, WebAssembly), and formal definitions where the model is the specification (Standard ML, with Scheme and Haskell in between).

## The specifications

### Go

The Go specification is short and unnumbered. After a brief overview and grammar notation, it spends a long stretch on foundations before any statement or expression appears. Types are described kind by kind, and a dedicated part defines the properties that later rules depend on: type identity, assignability, representability, and method sets. Each is defined once and every later rule that needs it links back by anchor rather than restating it. Scope rules are a single numbered list in one place.

Examples are plain code blocks whose comments state the value or say the line is invalid. Implementation latitude is marked by a fixed phrase at the start of a paragraph. Findability rests on headings, anchors, and dense hyperlinking of defined terms. Go reads as principled because its rules are stated in terms of a handful of centrally defined relations, and those definitions come first.

### Java Language Specification

The JLS is the model of a numbered, prose-first reference. Its first chapter is a roadmap that explains the ordering: lexical structure, types and values, conversions, names, then packages, classes, interfaces, exceptions, and execution, and only then statements and expressions. Foundational ideas (variables as typed storage, the compile-time versus run-time error split) are set up early and reused.

Rationale, intuition, and advice are set in smaller indented text that is explicitly non-normative. Examples are complete and runnable. Every section has a dotted number used in cross references. Its weakness for Tidal's purposes is that principles are stated as introductory prose and then re-derived in each chapter, so the document is long and repetitive. Its strength is that the reader never doubts which text is binding.

### R7RS Scheme

The Scheme report leads with a semantic overview that states the properties everything else follows from: static scoping, types attached to values rather than variables, unlimited extent for objects, mandatory proper tail recursion, first-class procedures and continuations, call by value, and exact versus inexact numbers. Each is a genuine principle and later entries rarely restate them. The report then defines its requirement vocabulary carefully, distinguishing a situation where an error must be signaled from one that is merely an error, and defining unspecified. The modal words apply only to implementations, never to programmers.

Feature descriptions use a fixed entry format with a header naming the category (syntax, auxiliary syntax, procedure) and a template. Naming conventions do work that would otherwise need rules: a parameter's name constrains its type, and suffix characters mark predicates and mutators. Scheme reads as principled because its overview is short, strong, and actually used.

### The Definition of Standard ML

This is the extreme case. The Definition's preface argues that a language is characterized by its semantic objects rather than its syntax, so a definition must be given in terms of those objects. Each semantics chapter first defines the semantic objects (environments, types, values, exception names) as sets built by a few constructions, then gives one inference rule per syntactic form. Static and dynamic semantics are separate chapters with the same shape. Rules are numbered sequentially, side conditions sit beside the rule they qualify, and commentary and change history go to appendices.

The Definition is unreadable as a manual and was never meant to be one. It matters because it shows the strong form of the principle: once the semantic objects are fixed, every rule is a statement about them, and a rule that cannot be stated in those terms is suspect.

### Ada Reference Manual and its Annotated edition

Ada is the most directly relevant model. Every clause has the same skeleton of named segments in a fixed order: syntax, name resolution rules, legality rules, static semantics, post-compilation rules, dynamic semantics, bounded errors, erroneous execution, implementation requirements, documentation requirements, metrics, implementation permissions, implementation advice, then notes and examples. Two parts of that skeleton answer the complaint directly. A clause may open with a segment on language design principles, which states the goals the rules serve and is marked as not a rule. The static semantics segment defines the compile-time meaning of a construct (what it denotes, what it introduces), and the legality rules then refer to those meanings.

The Annotated edition interleaves labeled annotations with the normative text: reason (why the rule exists), ramification (a consequence that follows from the rule, often non-obvious), discussion, proof (why a stated consequence really follows), to be honest (an admission that the wording falls short of the intent), implementation note, and correction. Annotations are numbered as a letter suffix on the paragraph they follow, so the normative numbering is undisturbed. Ada reads as principled because the ramification and proof annotations make the derivation of a rule from its neighbors explicit, and the fixed skeleton tells the reader where each kind of fact lives.

### WebAssembly

The core spec separates the language into phases (structure, validation, execution), each with its own chapter and conventions page. The validation conventions define the context, the form of a typing judgment, and the meaning of a rule before any rule appears. Each rule is then given twice: a prose bullet list of constraints and a formal inference rule. The document says the prose alone is enough, and the formal rules are for concision and proof. Both are declarative; a checking algorithm is an appendix. Appendix indexes list every type, instruction, and rule.

This is the best model for a spec that wants both readability and rigor. The principle is the conventions page; the rules are instances of the judgment form it defines, and the reader can check that any rule has the right shape.

### Rust Reference and the Ferrocene Language Specification

The Rust Reference is narrative, meant to be read in any order, and heavily cross-linked. Each rule paragraph carries a dotted identifier mirroring the heading hierarchy, and some rules link to the compiler tests that exercise them. Examples, notes, warnings, and edition differences are labeled callouts. The destructors chapter is a good example of principle-first structure. It opens with what a destructor is and the situations in which one runs, defines a type's destructor recursively over its parts, introduces drop scopes as the single organizing idea, lists what creates a scope and how scopes nest, and only then works through the detailed cases (parameters, bindings, temporaries, lifetime extension). Exceptions come last. A reader who has absorbed the scope model can predict most of the cases.

Ferrocene recasts the Reference content in the Ada skeleton: syntax, legality rules, dynamic semantics, undefined behavior, implementation requirements, examples. Every paragraph has a positional identifier. It distinguishes rules (obligations on the programmer, written normatively) from facts (properties of the tool, written descriptively), and says that where the language is unclear it describes what the compiler does. Ferrocene is a direct precedent for turning a narrative reference into a numbered, segmented one, and a warning: the model is noticeably harder to learn from Ferrocene than from the Reference, because the connective prose was removed.

### ECMAScript

ECMAScript is algorithmic. The front chapters define notation, the specification-internal data types (completion records, references, environment records), and a library of named abstract operations. Each syntax section then gives a grammar block, early errors (static checks), static semantic operations, and a runtime evaluation written as numbered steps that call the abstract operations. Notes are labeled informative blocks. Clauses are dotted and every heading has a stable anchor.

The principle lives in the abstract operations and specification types. A rule about an if statement is a few steps that call conversion and evaluation operations defined once. This gives unambiguous behavior but reads as a program rather than a theory, and offers no help with why.

### Haskell Report

Haskell states its model in the first chapter: a small kernel with a simple denotational reading, and every convenient construct defined by translation into the kernel. Each translation is given where the construct's syntax is introduced. The reader is told that the meaning of any construct is the meaning of its translation, so an itemized rule about a sugared form is a consequence of the translation plus the kernel. Identifying errors with nontermination disposes of a whole class of rules in one sentence. Prose, translation rules, and examples are visually distinct.

### Dafny and Lean reference manuals

Both are narrative manuals with a tutorial flavor. Dafny opens with a complete worked program, then proceeds bottom-up through lexical rules, modules, types, members, specifications, statements, and expressions, with the precise grammar collected in one late chapter and linked from the prose. Lean states its architecture up front (a small trusted kernel, elaboration producing kernel terms, tactics written in the language), and the early chapters on elaboration and the type system are the model the later chapters assume. Neither separates normative from informative text strictly. They read as principled because the opening architecture chapters are real and each later chapter places its topic in that architecture.

### The C standard compared with the C++ standard

The C standard uses a fixed skeleton within each clause: syntax, constraints, semantics, and for library functions description and returns. The constraints segment holds exactly the conditions a translator must diagnose, so a rule's diagnosability is a property of its position. Its foundational chapter is a conceptual model: translation and execution environments, the abstract machine, and the definition of the behavior terms. The C++ standard, in general terms, is running numbered paragraphs with bracketed notes and examples, less fixed structure per clause, and a conceptual model spread over several early chapters. Tidal's skeleton is already closer to C than to C++, which is right for a spec that wants rules to be classifiable and testable. The gap is that C's front-chapter model has no Tidal equivalent.

## Patterns across the survey

1. The model comes first and is small. Scheme, Haskell, Standard ML, WebAssembly, and Lean state their core model in a few pages. Go and Java do the same at greater length by defining relations (identity, assignability, conversion) centrally.
2. A fixed clause skeleton tells the reader what kind of fact they are reading. The position of a rule carries meaning.
3. Explanation is separate from rules but adjacent. The annotation categories that connect rules to principles are reason, ramification, and proof.
4. Every rule has a stable address: paragraph identifiers, clause numbers, rule numbers, or anchors. Rust also links rules to tests.
5. Grammar is a table of contents, not the organizing principle. Concept chapters (scopes, assignability, destructors) own the general rules; syntax chapters refer to them.

## Advice for Tidal

### Priority 1: add a foundations part with its own pages

Tidal has pages for statements, expressions, declarations, and classes, plus a glossary, and no page that states the model. A glossary defines terms but cannot say how they relate. A foundations part should come first in the index, with roughly these pages, each a short conceptual description plus a small number of tested principles:

- Translation and execution. Phases of translation, constant evaluation as a separate evaluation mode, what an implementation must diagnose and when, and the meaning of each behavior class Tidal uses. This moves the rule classes from the format document into the specification itself.
- Objects, storage, and lifetime. What an object is, that every object has a storage duration and a lifetime with a start and end, that temporaries are objects tied to a full expression, and the general reverse-order destruction rule. Most dynamic-semantics rules on the if page and the value-categories page are instances of this page.
- Values, expressions, and value categories. The model that an expression yields either a value or a reference to an object, the three categories as answers to two questions (does it name an object, and may it be moved from), and materialization as the single operation that turns a value into a temporary. The current page lists which expressions are in which category; the foundations page should say why the categories exist and what each permits, so the lists become consequences.
- Types and conversions. The type taxonomy, qualification, the shape of implicit conversion sequences, and contextual conversion to bool as a named operation every condition uses.
- Names, scopes, and lookup. Declarative regions, visibility from point of declaration to end of region, hiding by nested regions, and the prohibition on redeclaring a name in the same region. Several if-page rules about where a condition variable is visible and where it may be redeclared follow once the scopes an if statement introduces are listed.
- Evaluation order and sequencing. Tidal's sequencing page belongs here.
- Constant evaluation and immediate contexts. The constexpr and consteval rules on the if page are instances.

Each foundations page is testable. A lifetime principle is tested by destructor order; a scope principle by whether a name is found. The tests on a foundations page exercise the principle in its simplest setting, and topic pages cite the principle rather than re-testing it.

### Priority 2: give each topic page a model section between the summary and the grammar

A topic page should open with the question the construct answers and the foundations concepts it uses. For the if statement this is a few short paragraphs: the statement has a condition, an optional initializer, and two branches; the condition undergoes contextual conversion to bool (conversions); the statement introduces one scope for the initializer and condition that encloses both branches, and each branch is its own scope (scopes); temporaries of the condition end before a branch runs and condition variables end with the statement (lifetime); the constexpr and consteval forms differ in which branch is selected and when, and in whether the unselected branch is instantiated (constant evaluation). Then the grammar, then the rules.

The similarity tool already recognizes a principle directive with its own identifier scheme, so the mechanics exist. A principle directive should carry tests exactly as a rule does. If a principle cannot be given a test, it is either a definition (glossary or foundations page) or a design goal (labeled rationale note), and should not be stated as a principle.

### Priority 3: keep the itemized rules, but mark their relation to principles

Do not move the tested rules into an appendix. Every usable reference in the survey keeps detailed rules in the body with stable identifiers. What should change is the connective information around them:

- A derivation field on each rule naming the principle it instantiates. The build can then render, under each principle, the rules derived from it, which gives the index for free. A rule with no derivation is a primitive fact and should be rare on topic pages; a page with many underived rules signals a missing foundations concept.
- Ada-style annotation directives, limited to reason, ramification, and proof. Ramification is the most valuable, because it is where a surprising rule is shown to follow from a principle. Keep them informative, visually distinct, and short.
- A generated rules index, grouped by page and by principle, with each rule's tests. This is what the committee member probably meant by footnotes or an index. The rule text stays on the topic page so the test link survives; the index is a second view.

If rules should look subordinate on a page, a compact or collapsible rendering is enough: principles in full size, derived rules in a smaller block below, each still carrying its identifier and tests.

### Priority 4: organize by concept where the grammar is not the natural unit

Grammar is a good unit for statements and most declarations. It is a poor unit for lifetime, scopes, overload resolution, conversions, and constant evaluation, which cut across many productions. Rust's destructors chapter is the model: a concept page owns the general rules, and syntax pages say which scopes or temporaries a construct introduces and then defer. The lifetime and scope rules on the if page should become one or two sentences each pointing at the concept page, with the test files kept on the topic page as evidence that the concept rule holds for this construct.

### Priority 5: define the requirement vocabulary once, in the foundations

Scheme and Ferrocene define their modal words and error categories in the front matter and then use them without comment. Tidal's style guide does this for contributors; the specification should do it for readers. Ferrocene's split between rules (obligations on the program) and facts (properties of the implementation) maps well onto Tidal's classes and is worth stating.

## Risks

Vague principles. A slogan (objects have lifetimes) is untestable and will be ignored. A principle must have observable consequences (a temporary is destroyed at the end of its full expression unless a reference extends it, and temporaries of one full expression are destroyed in reverse order of creation). Insist on tests for principles; the tooling already treats a principle as a unit, so this can be enforced.

Principles that are false in corners. Ada's to-be-honest annotation exists because general statements about a real language almost always have exceptions. Give each principle a place to list its exceptions by rule identifier rather than weakening its wording. A principle with an explicit exception list still predicts; a hedged one does not.

Principles that reproduce another document's model. A principle is more likely than a rule to drift toward a familiar formulation, because the concepts are shared. The similarity check must run on principles with the same thresholds, and principles must be derived from observed behavior and summaries like everything else.

Loss of the test link. If rules are rendered in an index, their tests must travel with them. The derivation field keeps the link in both directions.

Two sources of truth. A principle and a derived rule can disagree after an edit. The build should check that every rule naming a principle shares a test with it, or at least that the principle's page links to the rule.

Over-formalization. Standard ML and WebAssembly show a formal model can be the whole spec, but a C++-compatible language is too large and irregular for that, and Tidal's audience is implementers and programmers. WebAssembly's compromise is right: declarative prose with the shape of a judgment, formal version optional. Aim for principles that could be made formal without making them formal.

Effort order. The foundations part is the large task and will take several pages before it pays off. Start with lifetime and scope, which the sampled pages depend on most, and retrofit the if page and the value-categories page to cite them. That pair demonstrates the structure and is a fair test of whether the complaint has been answered.
