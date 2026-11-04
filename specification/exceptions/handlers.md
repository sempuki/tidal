# Try blocks and handlers

A try block marks the statements whose exceptions its handlers can
catch. A handler names the type it accepts, and the first handler whose
type matches the exception object runs. This page covers:

- the scope of a try block,
- how a handler is selected,
- handlers by value and by reference,
- the catch-all handler,
- rethrow from a handler,
- nested try blocks,
- function try blocks.

Depends on: throw expressions, standard conversions, derived classes,
constructors and destructors.

## Principles

:::{principle} TRY-P1
:rules: TRY-1 TRY-16 TRY-3 TRY-4

A try block names the statements it guards and the {term}`handlers <handler>` that can
take over when one of those statements throws. When an exception leaves
the block, the handlers are tried in the order written:

```text
exception leaves the try block
        |
        v
first handler accepts its type? --yes--> that handler runs
        | no
        v
next handler accepts its type?  --yes--> that handler runs
        | no
        v
       ...
        | no handler accepts it
        v
the exception leaves the try block as it entered it,
and the next enclosing try block gets the same chance
```

Once a handler runs, the rest are not consulted, so a handler that a
previous handler always shadows can never run, and an implementation
should say so. The catch-all handler accepts everything, so the
catch-all handler must come last. A try block with no handler guards
nothing, so such a try block is not valid.
:::

:::{principle} TRY-P2
:rules: TRY-2 TRY-5 TRY-18

A handler accepts an exception by its type, with only the conversions
that keep the {term}`exception object` what it is:

- adding {term}`cv-qualifiers <cv-qualifier>`,
- going from a derived class to a public unambiguous base,
- adjusting a pointer's qualification or pointee base,
- dropping `noexcept` from a pointer to a function or a pointer to a
  member function,
- adjusting the qualification of a pointer to member.

A thrown array or function is already a pointer ({rule}`THR-2`), so a
handler for the pointer matches it, and a handler of a reference to an
array or a function does not.

Arithmetic conversions and user-defined conversions change the value, so
these conversions take no part. The form of the handler decides what the
handler receives:

| Handler | Receives |
|---|---|
| By value | A copy, which slices a derived exception to the base |
| By reference | The exception object itself, which the handler can change before passing the object on |

A by-value handler needs a copy constructor that it can use. If the copy
for a by-value handler throws, the program ends.

The handler's parameter is a declaration of a narrow kind. The parameter
has:

- a type, at most a name, and attributes,
- no storage class, no default, and no `auto`,
- a complete object type, or a pointer or reference whose target is a
  complete object type or `void`.
:::

:::{principle} TRY-P3
:rules: TRY-6 TRY-14 TRY-7 TRY-15

The try block and each handler are scopes of their own
({principle}`NAM-P1`). Names declared in the try block are not visible
in the handlers of the try block or after the try block, and a handler's
parameter belongs to that handler alone, so two handlers can use one
name. Control can leave a handler by any jump, and the objects of the
handler end as they would on any exit ({principle}`OBJ-P2`). These
objects end while the exception is still handled, so their destructors
can rethrow the exception. Control
cannot enter a try block or a handler by a jump, because entering a
handler without an exception, or a try block past its start, has no
meaning.
:::

:::{principle} TRY-P4
:rules: TRY-8

While a handler runs, the exception it caught is the
{term}`handled exception`. `throw` with no operand passes that same object on to the
next enclosing try block, from the handler or from any function the
handler calls, and a change made through a reference parameter goes with
the object. A handler that starts inside another handler has its own
handled exception until it ends. Then the outer handler's exception is
the handled exception again. A throw of a new value from a handler also goes outward. The
handlers of the same try block are not candidates for an exception that
one of these handlers throws. Leaving the handler by any route ends the
handled exception. `throw` with no operand and no handled exception has
nothing to pass on, so the program ends ({rule}`THR-7`).
:::

:::{principle} TRY-P5
:rules: TRY-9 TRY-10 TRY-17 TRY-11 TRY-12 TRY-13

A function try block makes the whole body of a function a try block,
including the member initializers of a constructor, and the handlers of
the function try block see the parameters. For an ordinary function the
function try block is a plain try block around the body, and the end
of a handler is the end of the function. So the end of a handler of
`main` returns zero. The function try block of `main` guards only the
body of `main`. The objects with static storage duration begin before
`main` and end after it, so the handlers of `main` never see their
exceptions. For a constructor the handler runs on an object that failed
to come into being. The parts built so far are already destroyed when the handler
starts, so the handler cannot use those parts or return as if
construction succeeded. When control reaches the end of the handler, the
exception continues outward.

A destructor's function try block also guards the destruction of the
members and bases, and its handler runs after all of them have ended.
The handler then works as the constructor's does. Flowing off the end of a
handler in a function that returns a value is as wrong as flowing off
the end of the function, and an implementation should warn.
:::

## Syntax

```{eval-rst}
.. productionlist:: tidal
   try_block: "try" `compound_stmt` `handler` {`handler`}
   handler: "catch" "(" `exception_decl` ")" `compound_stmt`
   exception_decl: {`attribute_spec`} `type_specifiers` [`declarator` | `abstract_declarator`]
                 : | "..."
   function_try_block: "try" [`ctor_init`] `compound_stmt` `handler` {`handler`}
```

A `function_try_block` takes the place of a function body. The
*handler parameter* is the declaration in a handler's parentheses, and
the *catch-all handler* is the one written with `...`.

## Constraints

:::{rule} TRY-16
:kind: reject
:tests: exceptions/handlers/try-without-handler.cpp exceptions/handlers/function-try-without-handler.cpp

A try block must have at least one handler.
:::

:::{rule} TRY-3
:kind: reject
:tests: exceptions/handlers/catch-all-not-last.cpp exceptions/handlers/catch-all-only.cpp exceptions/handlers/catch-all.cpp

The catch-all handler `catch (...)` accepts every exception and must be
the last handler of its try block.
:::

:::{rule} TRY-5
:kind: reject
:tests: exceptions/handlers/param-deleted-copy.cpp exceptions/handlers/param-private-copy.cpp exceptions/handlers/param-abstract-reference-ok.cpp exceptions/handlers/param-abstract.cpp exceptions/handlers/param-array-decays-to-pointer.cpp exceptions/handlers/param-auto.cpp exceptions/handlers/param-function-type.cpp exceptions/handlers/param-incomplete-pointer.cpp exceptions/handlers/param-incomplete.cpp exceptions/handlers/param-incomplete-reference.cpp exceptions/handlers/param-function-decays-to-pointer.cpp exceptions/handlers/param-qualified-name.cpp exceptions/handlers/param-reference-to-unknown-bound-array.cpp exceptions/handlers/param-rvalue-reference.cpp exceptions/handlers/param-unnamed.cpp exceptions/handlers/param-void.cpp exceptions/handlers/param-pointer-unknown-bound-array.cpp exceptions/handlers/param-attribute.cpp

The handler parameter must not:

- be an {term}`abstract class` by value,
- have an incomplete type, or the type `void`,
- be a pointer or a reference to an incomplete type other than `void`,
  such as an incomplete class or an array of unknown bound,
- have an rvalue reference type,
- have a qualified name or `auto`.

The copy constructor of a handler parameter of class type by value must
be usable and accessible, also when the operand of the `throw` is a
prvalue. An array type or a function type decays to a pointer, as in a function
parameter. A pointer to `void` is valid. The parameter can have no name.
The `exception_decl` production derives no storage class and no default
argument ({rule}`TR-12`). Attributes at the start of the handler
parameter apply to the parameter.
:::

:::{rule} TRY-14
:kind: reject
:tests: exceptions/handlers/handler-param-scope.cpp exceptions/handlers/handler-redeclares-param.cpp exceptions/handlers/try-names-not-after.cpp exceptions/handlers/try-names-not-in-handler.cpp

A name declared in the try block is not visible in the handlers of the
try block or after the try block. The handler parameter is visible only
in its handler, and the outermost block of the handler must not declare
the parameter again.
:::

:::{rule} TRY-7
:kind: reject
:tests: exceptions/handlers/case-into-handler.cpp exceptions/handlers/goto-into-handler.cpp exceptions/handlers/goto-into-try.cpp

The following must not transfer control into a try block or into a
handler from outside that try block or handler:

- a `goto`,
- a `case` label,
- a `default` label.
:::

:::{rule} TRY-17
:kind: reject
:tests: exceptions/handlers/ctor-try-block-return-rejected.cpp exceptions/handlers/ctor-try-block-return-nested.cpp

A constructor's function try block has handlers that cannot finish
the construction, so a `return` statement inside one of these handlers
is not valid.
:::

## Runtime Behavior

:::{rule} TRY-1
:kind: required
:tests: exceptions/handlers/basic-catch.cpp exceptions/handlers/handlers-in-order.cpp exceptions/handlers/no-match-propagates.cpp exceptions/handlers/skips-non-matching.cpp exceptions/handlers/try-block-completes-normally.cpp

When an exception leaves a try block, the handlers of the try block are
tried in order, and the first handler that matches runs. If none
matches, the exception leaves the try block. If the try block completes
without an exception, no handler runs.
:::

:::{rule} TRY-2
:kind: required
:tests: exceptions/handlers/match-ambiguous-base-no.cpp exceptions/handlers/match-array-decays.cpp exceptions/handlers/match-base-by-reference.cpp exceptions/handlers/match-base-by-value-slices.cpp exceptions/handlers/match-cv-qualified.cpp exceptions/handlers/match-function-decays.cpp exceptions/handlers/match-no-arithmetic-conversion.cpp exceptions/handlers/match-no-user-conversion.cpp exceptions/handlers/match-nonconst-reference-to-exception-object.cpp exceptions/handlers/match-nullptr-to-pointer.cpp exceptions/handlers/match-pointer-qualification.cpp exceptions/handlers/match-pointer-to-base.cpp exceptions/handlers/match-private-base-no.cpp exceptions/handlers/match-void-pointer.cpp exceptions/handlers/match-void-pointer-keeps-const.cpp exceptions/handlers/match-function-pointer-not-void.cpp exceptions/handlers/match-void-pointer-from-pointer-to-pointer.cpp exceptions/handlers/match-noexcept-function-pointer.cpp exceptions/handlers/match-nullptr-to-member-pointer.cpp exceptions/handlers/match-member-pointer-qualification.cpp exceptions/handlers/match-member-pointer-no-base-to-derived.cpp exceptions/handlers/match-private-base-in-member-no.cpp exceptions/handlers/match-reference-to-pointer-converts.cpp exceptions/handlers/match-noexcept-member-function-pointer.cpp exceptions/handlers/match-reference-to-array-never.cpp exceptions/handlers/match-reference-to-function-never.cpp

A handler with parameter type `T`, or a reference to `T`, matches an
exception object of type `E` if one of these holds:

- `T` is `E`, with any cv-qualifiers added.
- `T` is a public unambiguous base of `E`. A private or protected base
  does not match, also inside a member of `E` where the base is
  accessible.
- `T` and `E` are pointers, and `E` converts to `T` by one or more of
  these conversions:
  - a qualification conversion,
  - a conversion to a pointer to a public unambiguous base class,
  - a conversion of a pointer to an object type to a pointer to `void`
    with the same or more cv-qualifiers,
  - a conversion of a pointer to a `noexcept` function to a pointer to
    a function without `noexcept`.
- `T` and `E` are pointers to members, and `E` converts to `T` by a
  qualification conversion, or by the conversion of a pointer to a
  `noexcept` member function to one without `noexcept`.
- `T` is a pointer or a pointer to member, and `E` is the type of
  `nullptr`.

The pointer conversions apply also when the handler is a reference to
a pointer that is not `const`. The exception object of a thrown array
or function is a pointer ({rule}`THR-2`), so a handler of a reference
to an array or to a function does not match
({question}`Q-0363`).

An arithmetic conversion or a user-defined conversion does not make a
match. A pointer to a member of a base does not match a handler for a
pointer to a member of a derived class. A handler by value copies the exception object, and a copy to a
base type slices the object. The copy constructor of the base then
initializes the parameter from the base part of the exception object.
A handler by reference refers to the
exception object, also through a reference that is not `const`.
:::

:::{rule} TRY-6
:kind: required
:tests: exceptions/handlers/handler-param-hides-outer.cpp exceptions/handlers/handlers-share-param-name.cpp exceptions/handlers/nested-block-redeclares-param-ok.cpp

The try block and each handler are scopes.

- The handler parameter hides a name of the enclosing scope.
- A block inside the handler can hide the parameter.
- Two handlers of one try block can give their parameters one name.
:::

:::{rule} TRY-15
:kind: required
:tests: exceptions/handlers/break-out-of-handler.cpp exceptions/handlers/goto-out-of-handler.cpp exceptions/handlers/return-from-handler.cpp exceptions/handlers/continue-out-of-handler.cpp exceptions/handlers/handler-local-destructor-rethrows.cpp

A handler can end early with one of these:

- `return`,
- `break`,
- `continue`,
- `goto`.

When a handler ends by any route, its local objects end first, last made
first ({rule}`OBJ-3`). The handled exception of the handler ends after
them, so a destructor of such an object can still rethrow the exception.
:::

:::{rule} TRY-8
:kind: required
:tests: exceptions/handlers/handler-not-active-for-own-throw.cpp exceptions/handlers/nested-try-in-handler.cpp exceptions/handlers/rethrow-copy-not-modified.cpp exceptions/handlers/rethrow-from-called-function.cpp exceptions/handlers/rethrow-modified-reference.cpp exceptions/handlers/rethrow-to-enclosing.cpp exceptions/handlers/throw-new-from-handler.cpp exceptions/handlers/rethrow-after-inner-handler-ends.cpp

`throw` with no operand rethrows the {term}`handled exception`
({rule}`THR-6`) to the next enclosing try block. The rethrow can occur
in the handler or in a function that the handler calls. A change to a
by-value parameter does not reach the exception object. A `throw` of a
new value in a handler leaves the handler, and the other handlers of
the same try block do not catch the new exception. A try block inside a
handler catches its own exceptions. {rule}`THR-7` gives a rethrow with
no handled exception.
:::

:::{rule} TRY-9
:kind: required
:tests: exceptions/handlers/function-try-block-falls-off-end.cpp exceptions/handlers/function-try-block-main.cpp exceptions/handlers/function-try-block-sees-parameters.cpp exceptions/handlers/function-try-block.cpp exceptions/handlers/function-try-block-main-falls-off-end.cpp exceptions/handlers/main-try-block-misses-static-init.cpp exceptions/handlers/main-try-block-misses-static-destructor.cpp

A function try block wraps the full body of a function. Inside the
handlers of the function try block the parameters are visible, and a
`return` there gives the function's result, so a handler in `main` can
set the exit status. Reaching the end of a handler is reaching the end
of the function:

- In a function that returns nothing, the function returns.
- In `main`, the function returns zero ({rule}`FN-8`).
- In another function that returns a value, the behavior is undefined
  ({rule}`FN-3`).

An exception that a static or `thread_local` object throws while it is
built or destroyed never reaches the function try block of `main`
({rule}`UNW-5`).
:::

:::{rule} TRY-10
:kind: required
:tests: exceptions/handlers/ctor-try-block-members-destroyed-first.cpp exceptions/handlers/ctor-try-block-new-throw.cpp exceptions/handlers/ctor-try-block-rethrows-at-end.cpp exceptions/handlers/ctor-try-block-sees-member-init-exception.cpp exceptions/handlers/dtor-try-block-rethrows.cpp exceptions/handlers/dtor-try-block-member-destructor-throws.cpp

A function try block on a constructor also guards the member
initializers. Before the handler runs, the parts of the object that were
constructed are destroyed. If control reaches the end of the handler,
the exception is rethrown. A handler can throw a new exception instead.

A function try block on a destructor also guards the destruction of the
members and bases. If one of these destructors throws, the remaining
members and bases are still destroyed before the handler runs. If
control reaches the end of the handler, the exception is rethrown, as
for a constructor.
:::

:::{rule} TRY-18
:kind: required
:tests: exceptions/handlers/handler-copy-throws-terminates.cpp exceptions/handlers/handler-base-copy-throws-terminates.cpp

If the copy that initializes a handler parameter by value throws, the
program ends. No handler catches the new exception.
:::

## Undefined Behavior

:::{rule} TRY-11
:kind: undefined
:tests: exceptions/handlers/ctor-try-block-member-named-in-handler.cpp exceptions/handlers/dtor-try-block-member-named-in-handler.cpp exceptions/handlers/ctor-try-block-member-function-called-in-handler.cpp exceptions/handlers/ctor-try-block-member-call-sanitizer.cpp

The members and bases are gone by the time a constructor's or
destructor's function try block reaches a handler. A handler that still
uses a non-static member or a base of the object has undefined
behavior, and the program compiles all the same. A use includes:

- a read or a write of a data member,
- a call of a non-static member function,
- taking the address of a member or a base.
:::

## Recommended Practice

:::{rule} TRY-4
:kind: recommended
:tests: exceptions/handlers/distinct-handlers-no-warning.cpp exceptions/handlers/duplicate-handler-warns.cpp exceptions/handlers/unreachable-handler-warns.cpp

An implementation should warn about a handler that an earlier handler
of the same try block always shadows, such as a handler for a derived
class after one for its base, or a second handler for one type.
:::

:::{rule} TRY-12
:kind: recommended
:tests: exceptions/handlers/function-try-block-non-void-no-return.cpp exceptions/handlers/member-function-try-block-no-return.cpp

When a function returns a value, its function try block's handlers
should not let control run off their end. An implementation should warn
about a handler where control can run off the end.
:::

:::{rule} TRY-13
:kind: recommended
:tests: exceptions/handlers/ctor-try-block-member-use-warns.cpp exceptions/handlers/dtor-try-block-member-use-warns.cpp

An implementation should warn if a handler of a function try block on a
constructor or destructor names a member of the object.
:::

## Examples

% tidal-example: fragment
```cpp
struct Error { int code; };
struct IoError : Error { const char* path; };

int run() {
  try {
    open_all();
  } catch (IoError& e) {       // tried first: matches an IoError
    log(e.path);
    throw;                     // the same object continues outward
  } catch (const Error& e) {   // any other Error, including derived ones
    return e.code;
  } catch (...) {              // everything else; must be last
    return -1;
  }
  return 0;
}

struct Widget {
  Part a, b;
  Widget(int n) try : a(n), b(n) {
  } catch (...) {              // a and b are already gone here
    note("Widget failed");     // then the exception continues
  }
};
```

## Evidence

The results are in `tests/RESULTS.md`. All eight compilers agree on 98
of 105 tests. EDG runs no programs, so its results for the run tests say
only that the programs are valid, and the harness sees a terminating
program only as a nonzero exit.

One question on this page settled on 2026-11-04 under decision 0054 after the source-reader gap review of the rules that phase 7 added (`reports/review/2026-11-04-gap-review-phase-7-additions.md`). For each, the review supports the answer with no doubt, and a principle gives the same answer. Each test keeps its override for the family that still differs:

| Question | Principle | Family that still differs |
|---|---|---|
| {question}`Q-0363` | TRY-P2 | GCC 16 |

- TRY-4, TRY-12, TRY-13: the table gives the warnings of each compiler
  with no warning flag. `-Wall` adds none of these warnings on Clang or
  GCC. The tests use `-Wall -Werror`, so a warning rejects the program,
  and EDG skips the tests. A test whose GCC result is `accept` only
  compiles on GCC.

  | Case | Tests | Clang | GCC | EDG |
  |---|---|---|---|---|
  | TRY-4: a handler for a derived class after one for its base | `unreachable-handler-warns.cpp` | warns | warns | warns |
  | TRY-4: a second handler for one type | `duplicate-handler-warns.cpp` | warns | warns | warns |
  | TRY-12: a handler of a function try block runs off the end of a function that returns `int` | `function-try-block-non-void-no-return.cpp`, `member-function-try-block-no-return.cpp` | warns | none | warns |
  | TRY-13: a handler of a constructor's or destructor's function try block names a member | `ctor-try-block-member-use-warns.cpp`, `dtor-try-block-member-use-warns.cpp` | warns | none | none |
- TRY-12 and TRY-13 are `recommended` rules. Every test of each has an
  override, and each override records a result that the class allows.
- TRY-11: the test only shows that the program compiles.
- TRY-2: every Clang and GCC configuration matches a thrown pointer to
  an object, and a thrown pointer to a pointer, with a handler for a
  pointer to `void` that has at least the same cv-qualifiers. A pointer
  to `const int` does not match `void*`, and a pointer to a function
  does not match `const void*`.
- TRY-5: GCC rejects a qualified name in a handler parameter as a
  syntax error, where it expects a name without a qualifier. Clang and
  EDG say that the name cannot be qualified.
- TRY-2: Clang and GCC match a thrown `nullptr` with a handler for a
  pointer to member, a pointer to member with a handler that adds
  `const`, and a pointer to a `noexcept` function with a handler for a
  pointer to a function without `noexcept`. Neither matches a pointer to
  a member of a base with a handler for a pointer to a member of a
  derived class.
- TRY-5: Clang and GCC reject a by-value handler parameter whose copy
  constructor is deleted or private, also when the thrown operand is a
  prvalue.
- TRY-8: after a handler inside another handler ends, a rethrow in the
  outer handler passes on the outer exception, on Clang and GCC.
- TRY-15: on Clang and GCC, the destructor of a handler's local object
  can rethrow the handled exception and catch it, so the exception is
  still handled while the local objects end.
- TRY-18: Clang and GCC end the program when the copy into a by-value
  handler parameter throws, also when an enclosing try block has a
  matching handler. The source-reader gap review of the exceptions
  chapter (`reports/review/2026-11-03-gap-review-exceptions.md`) reads
  the C++23 text as letting an implementation skip the copy into the
  handler parameter, and then the program does not end. Every compiler
  that runs programs makes the copy, and Tidal requires the copy, so
  TRY-2 and TRY-18 hold on every implementation that follows Tidal.
- TRY-2: Clang and GCC give no match for a handler of a private or
  protected base, also inside a member function of the derived class,
  and match a pointer to a `noexcept` member function with a handler
  for a pointer to a member function without `noexcept`.
- TRY-2: a handler of a reference to a pointer that is not `const`
  matches a pointer to a derived class, a pointer whose pointee gains
  `const`, and `nullptr`, on Clang and GCC. The gap review reads the
  C++23 text as allowing these conversions only for a by-value handler
  or a reference to a `const` pointer. Tidal follows the compilers
  (`documentation/decisions/pending.md`, row 4).
- TRY-2: GCC matches a thrown array with a handler of a reference to an
  array, and a thrown function with a handler of a reference to a
  function. Clang matches neither
  ({question}`Q-0363`).
- TRY-5: Clang and GCC reject a handler parameter that points to an
  array of unknown bound, and accept an attribute at the start of the
  parameter.
- TRY-9, TRY-10: Clang and GCC return zero from the end of a handler of
  `main`. Neither lets the function try block of `main` catch an
  exception from the constructor or the destructor of an object at
  namespace scope. A destructor's function try block catches an
  exception from the destructor of a member after the remaining members
  have ended.
- TRY-11: the `vptr` sanitizer of GCC stops a call of a member function
  of a destroyed member in the handler
  (`ctor-try-block-member-call-sanitizer.cpp`). The Clang sanitizers
  and the address sanitizers do not, and the front end's interpreter
  runs the program to its result.

Inputs: clang-sema-stmt, clang-diag-sema, clang-diag-parse,
edg-statements.
