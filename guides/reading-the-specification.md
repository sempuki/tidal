# Reading the specification

This guide shows how to answer two questions about a program with
Tidal: is the program valid, and what does the program do? The guide
traces one short program to the rules that decide it.

## The task

A reader has a program and wants the answer from the rules alone. The
reader needs to know four things:

- how a page is laid out,
- what each rule class promises,
- how to go from a construct to its rules,
- how to read the tests and the open questions behind a rule.

## The worked example

The program below has an implicit conversion, an overloaded operator,
and a temporary object. Each destructor writes the value of its object
to `trace`, so the order of destruction is visible.

```cpp
char trace[8];
int pos = 0;

struct Meters {
  int v;
  Meters(int n) : v(n) {}                     // a converting constructor
  ~Meters() { trace[pos++] = char('0' + v); } // records each destruction
};

Meters operator+(const Meters& a, const Meters& b) {
  return Meters(a.v + b.v);
}

int main() {
  Meters start(5);
  {
    const Meters& total = start + 2;
    trace[pos++] = '|';
    if (total.v != 7) return 1;
  }
  return trace[0] == '2' && trace[1] == '|' && trace[2] == '7' ? 0 : 2;
}
```

The steps below derive this answer:

- The program is valid.
- The exit status is 0.
- The temporary `Meters` made from `2` is destroyed at the end of the
  declaration of `total`.
- The result of `start + 2` is destroyed at the closing brace of the
  inner block.

## Step 1: Know the parts of a page

Each page covers one topic. The parts come in a fixed order, and a page
leaves out a part that has no content (`documentation/style-guide.md`, section 2).

| Part | What it is for | Normative |
|---|---|---|
| Title and summary | Says what the page covers. | No |
| `Depends on:` line | Names the topics that the page builds on, including topics that Tidal does not specify yet. | No |
| `Principles` | States the model that the rules follow from. Each principle lists the rules that are its evidence. | Yes |
| `Syntax` | Gives the grammar of the construct. | Yes |
| `Constraints` | Rules that an implementation checks before the program runs. | Yes |
| `Runtime Behavior` | What the program does when it runs. | Yes |
| `Undefined Behavior` | Conditions that have no requirements. | Yes |
| `Implementation Requirements` | Behavior that each implementation chooses, and errors that need no diagnostic. | Yes |
| `Recommended Practice` | Warnings and other advice to implementations. | Yes, as advice |
| `Examples` | Short code with the result in a comment. | No |
| `Evidence` | Compiler results, divergences, and links to questions. The last line, `Inputs:`, names the inputs the page was written from. | No |

Read a page in its own order. A principle gives the model, so the
rules are easier to read after it. For the example, {principle}`OPR-P1`
says that an operator with a class operand is a function call in
operator syntax. The rules {rule}`OPR-6` and {rule}`OPR-7` give the
details.

A rule is a directive with an ID, a class, and a list of tests. The ID
is the page prefix and a number, such as `OPR-6`. The class tells the
reader how strong the rule is.

## Step 2: Know what each class promises

The class of a rule says whether the rule limits the program or
describes the implementation ({principle}`TR-P1`). A valid program
breaks no rule on the program. The facts then say what the valid
program does.

| Class | Kind | Answer to "is the program valid?" | Answer to "what does the program do?" |
|---|---|---|---|
| `reject` | Rule on the program | No, if the program breaks the rule. Every implementation gives a diagnostic ({rule}`TR-1`). | Nothing. The program is not translated. |
| `no-diagnostic` | Rule on the program | No, if the program breaks the rule. An implementation need not notice. | Nothing that a reader can rely on. |
| `undefined` | Rule on the program | The program is valid until an execution reaches the operation. | No requirement on the whole execution ({rule}`TR-8`). Constant evaluation rejects the operation ({rule}`TR-10`). |
| `required` | Fact | Not affected. | Every implementation does the same thing. |
| `impl-defined` | Fact | Not affected. | Each implementation chooses and documents the choice. A table can give the choices of the matrix targets. |
| `unspecified` | Fact | Not affected. | Each implementation chooses from the set that the rule gives. The choice need not be documented or consistent. |
| `conditionally-supported` | Fact | An implementation can reject the construct. | If the implementation accepts the construct, the implementation documents the behavior. |
| `recommended` | Advice | Yes. A warning does not make the program invalid ({rule}`TR-6`). | Not affected. |

Some `required` rules sit in the `Constraints` segment. Such a rule
states what a program can do, for example {rule}`OPR-1`. A `reject` rule
next to the `required` rule states the limit, for example
{rule}`OPR-2`.

{doc}`/foundations/translation` maps these classes to the vocabulary of
the ISO standard, for readers who know that vocabulary.

## Step 3: Follow the construct through the grammar

The grammar fixes the structure of the program before any types are
considered. All pages share one grammar (decision 0040). Each
nonterminal has one name and one defining page, and the name in a
production links to that page. Section 8 of `documentation/style-guide.md` gives
the notation and the pages of the shared nonterminals.

The declaration of `total` parses as follows. Each line names the
nonterminal and the page that defines the nonterminal.

```text
simple_decl                 declarations/index.md
  decl_specifiers           const Meters
  init_declarator           declarations/index.md
    declarator              & total        declarations/declarators.md
    init                    = ...          declarations/initialization/forms.md
      assignment_expr       assignment.md
        conditional_expr    conditional.md
          ...               one level for each row of the table
            add_expr        start + 2      additive.md
              mul_expr ... primary         start
              mul_expr ... primary         2
```

The table on {doc}`/expressions/index` lists the levels of the
expression chain. The first row binds most tightly. Each level is made
of operands from the level below. To place an operator, find its row,
and then open the page in the last column. `+` is on the `add_expr`
row, so the page is {doc}`/expressions/additive`.

An overloaded operator keeps the precedence and associativity of the
built-in operator ({rule}`OPR-11`). So the parse of `start + 2` is the
same for every operand type. Only the meaning of the `+` depends on the
operand types.

## Step 4: Find the rules

Use the entry point that fits what the reader has.

| The reader has | Go to | What the place gives |
|---|---|---|
| A rule ID, such as `REF-4` | The prefix table on {doc}`/rules` | The page for each prefix. |
| An operator | The level table on {doc}`/expressions/index` | The page of the operator. |
| An operand of class type | {doc}`/classes/operators` | How the expression selects an operator function. |
| A term, such as *full expression* | The {doc}`/glossary` | A definition and the rule that gives the details. A page links a term to the glossary the first time a rule uses the term. |
| A principle | The rule index on {doc}`/rules` | Each principle with its rules and the number of tests behind each rule. |
| A suspicion of undefined behavior | The table of undefined rules on {doc}`/foundations/constant-evaluation` | Every `undefined` rule, and the compilers whose constant evaluation rejects each one ({rule}`CE-9`). |

The rule index lists the principles by chapter and page. A rule marked
*except* departs from its principle. The number after each rule is the
count of tests that support the rule.

The table of undefined rules comes from the test expectations. Each
entry says whether a compiler rejects every case, some cases, or no
case during constant evaluation. A reader can test a doubt the same
way. Put the code in a `constexpr` function and call the function in a
`static_assert`. Constant evaluation rejects an undefined operation
({rule}`TR-10`), also when the result of the operation is not used
({rule}`TR-8`).

## Step 5: Trace the example

The table follows the program in the order of execution.

| Point in the program | What the rules say | Rules |
|---|---|---|
| `start + 2` | The left operand has class type. The expression selects an operator function from the candidates. | {rule}`OPR-6` |
| The built-in `+` | The built-in `+` takes arithmetic or pointer operands. The built-in operator is a candidate only after a conversion of the operand. A class converts to another type through a conversion function, and `Meters` has none. So the built-in `+` is not viable. | {rule}`ADD-3`, {rule}`OPR-6`, {principle}`CVF-P1` |
| The non-member `operator+` | Lookup finds the non-member function. Implicit conversions apply to every operand of a non-member. | {rule}`OPR-6` |
| The call | The expression calls `operator+` with the operands as arguments. | {rule}`OPR-7` |
| `start` to parameter `a` | Each parameter is copy-initialized from its argument. A `const` reference binds to the lvalue `start` directly. | {rule}`PFX-6`, {principle}`REF-P1` |
| `2` to parameter `b` | `2` is an `int`. A `const` reference can bind to an expression of a different type that converts to its type. The reference then binds to a temporary object that holds the converted value. | {rule}`PFX-6`, {rule}`REF-2` |
| `int` to `Meters` | `Meters(int)` is not `explicit`, so `Meters(int)` is a converting constructor. The implicit conversion uses the constructor once. | {rule}`CTOR-10`, {rule}`CTOR-22` |
| `return Meters(a.v + b.v);` | The cast gives a prvalue. A prvalue that initializes an object of the same type initializes the object directly, with no copy. | {rule}`VC-2`, {rule}`VC-4` |
| The value of the call | A call to a function that returns a type that is not a reference is a prvalue. | {rule}`VC-2` |
| `total` binds to the value | A reference that binds to a prvalue makes a temporary object. A `const` reference binds to a class rvalue directly. | {principle}`VC-P2`, {rule}`REF-2` |
| The life of the result | The reference variable `total` binds to the temporary, so the temporary lives as long as `total`. | {rule}`REF-4` |
| The `;` after `start + 2` | The initializer is a full expression. The temporary `Meters` made from `2` ends here and writes `'2'`. The parameter `b` does not extend the life of the temporary. | {rule}`VC-9`, {rule}`VC-6`, {rule}`REF-5` |
| The inner `}` | `total` ends, and the temporary with the value 7 ends with `total`. The destructor writes `'7'`. | {rule}`REF-4` |
| `return ... ? 0 : 2;` | The value that `main` returns is the exit status. | {rule}`TR-2` |

The program breaks no `reject` rule. No execution reaches an
`undefined` rule, because `total` is used only while the temporary
lives. So the program is valid, and every implementation gives exit
status 0.

## Step 6: Read the tests behind a rule

Each rule lists its tests in `:tests:`. A test is one C++ file under
`tests/`, in a directory that mirrors `specification/`. Comment lines at the top
of the file tell the runner what to expect (`documentation/specification-format.md`).

This test supports {rule}`OPR-15`:

```cpp
// tidal-rules: OPR-15
// tidal-expect: reject
// tidal-error-clang: invalid operands to binary expression
// tidal-error-gcc: no match for .operator\+
// tidal-error-edg: no operator .\+. matches
struct S { S(int) {} S operator+(S) const { return *this; } };
S f() { return 1 + S(2); }
```

| Line | Meaning |
|---|---|
| `tidal-rules` | The rules that the test supports. |
| `tidal-expect` | The result on every compiler. `accept` means that the program compiles. `reject` means that the program fails to compile. `run` means that the program runs and exits with 0. `link-fail` and `run-fail` mean failure at those stages. |
| `tidal-error-<family>` | A regular expression that a diagnostic of that compiler family must match. A rejection for another reason fails the test. |
| `tidal-flags` | Extra compiler flags, such as a sanitizer. |
| `tidal-sources` | Extra translation units that the test links with. |

Every compile uses the strict C++23 mode of the compiler. A `run` test
checks its own result and returns nonzero on failure. `tests/RESULTS.md`
gives the result of each test on each compiler of the matrix.

A test shows what the rule means in practice. The test above has a
member `operator+` and an `int` left operand. Implicit conversions do
not apply to the left operand of a member ({rule}`OPR-6`), so no
candidate is viable, and the expression is not valid ({rule}`OPR-15`).

## Step 7: Read overrides and questions

The compilers of the matrix sometimes disagree. A test then has an
*override*, a `tidal-expect-<x>` line for one compiler family or one
compiler. An override always comes with a `tidal-note` that names a
question.

```cpp
// tidal-rules: OPR-10
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: literal operator cannot have a default argument
// tidal-error-edg: literal operator cannot have default arguments
// tidal-note: Q-0060. GCC accepts a default argument on a literal operator. Clang and EDG reject it.
using Size = decltype(sizeof 0);
int operator""_d(const char*, Size = 0);
```

Read the override as follows:

- `tidal-expect` gives the result that the rule requires.
- `tidal-expect-gcc` records the result that GCC gives instead.
- The question, here {question}`Q-0060`, holds the facts, the options,
  and a provisional answer.

The rule follows the provisional answer. The Evidence segment of the
page names the compilers that diverge and links the question. For the
example above, {rule}`OPR-10` says that a literal operator has no
default argument, and the Evidence segment of {doc}`/classes/operators`
records that GCC accepts one. A rule text can also link a question
directly, as {rule}`REF-4` does for one of its cases.

A program that depends on a rule with an open question can get
different results on different compilers. The question says which
result Tidal adopts for now.

## Step 8: Read referenced-topic markers

Some topics have no page in Tidal yet. A rule that depends on such a
topic marks the dependency with the topic name in italics and
parentheses. {rule}`CPY-13` has the marker *(library referenced)*, and
the Syntax segment of {doc}`/classes/operators` has
*(coroutines referenced)*. `REFERENCED_TOPICS` in
`tools/check_specification.py` lists the allowed topics. The glossary
introduction describes the markers too.

A marker means that the rule relies on the topic and does not specify
the topic. The reader must bring that knowledge from elsewhere or test
compiler behavior. The marker limits what the rules can decide:

- In the example, only one candidate is viable, so no ranking is
  needed, and the rules decide the result.
- If two candidates were viable, the rules could not say which one
  wins. {rule}`OPR-15` covers only an ambiguity and the case of no
  viable candidate.

## Mistakes the rules forbid

Each block changes one part of the example.

An `explicit` constructor does not do the conversion of an argument
({rule}`CTOR-9`, {rule}`PFX-16`). No candidate is then viable
({rule}`OPR-15`).

```cpp
struct Meters {
  int v;
  explicit Meters(int n) : v(n) {}
};
Meters operator+(const Meters& a, const Meters& b) { return Meters(a.v + b.v); }
Meters start(5);
Meters total = start + 2;   // rejected
```

A member operator function does not convert its left operand
({rule}`OPR-6`), so `2 + start` has no viable candidate
({rule}`OPR-15`). The non-member form in the worked example accepts
both orders.

```cpp
struct Meters {
  int v;
  Meters(int n) : v(n) {}
  Meters operator+(const Meters& b) const { return Meters(v + b.v); }
};
Meters start(5);
Meters a = start + 2;       // valid
Meters b = 2 + start;       // rejected
```

A reference that a function returns does not extend the life of a
temporary ({rule}`REF-5`). The temporary ends with its full expression
({rule}`VC-9`), and a later use of the reference is undefined
({rule}`REF-5`). {rule}`REF-5` is an `undefined` rule, so
implementations accept the program.

```cpp
struct Meters {
  int v;
  Meters(int n) : v(n) {}
};
const Meters& keep(const Meters& m) { return m; }
int main() {
  const Meters& kept = keep(3);  // the temporary ends at the semicolon
  return kept.v;                 // undefined
}
```

The same code in constant evaluation is rejected ({rule}`TR-10`,
{rule}`CE-9`).

```cpp
struct Meters {
  int v;
  constexpr Meters(int n) : v(n) {}
};
constexpr const Meters& keep(const Meters& m) { return m; }
constexpr int read() {
  const Meters& kept = keep(3);
  return kept.v;
}
static_assert(read() == 3);   // rejected
```
