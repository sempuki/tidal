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

A try block names the statements it guards and the handlers that can
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
:rules: TRY-2 TRY-5

A handler accepts an exception by its type, with only the conversions
that keep the exception object what it is:

- adding cv-qualifiers,
- going from a derived class to an accessible unambiguous base,
- adjusting a pointer's qualification or pointee base,
- the decay of an array or function to a pointer.

Arithmetic conversions and user-defined conversions change the value, so
these conversions take no part. The form of the handler decides what the
handler receives:

| Handler | Receives |
|---|---|
| By value | A copy, which slices a derived exception to the base |
| By reference | The exception object itself, which the handler can change before passing the object on |

The handler's parameter is a declaration of a narrow kind. The parameter
has:

- a type and at most a name,
- no storage class, no default, and no `auto`,
- a type that an object can have.
:::

:::{principle} TRY-P3
:rules: TRY-6 TRY-14 TRY-7 TRY-15

The try block and each handler are scopes of their own
({principle}`NAM-P1`). Names declared in the try block are not visible
in the handlers of the try block or after the try block, and a handler's
parameter belongs to that handler alone, so two handlers can use one
name. Control can leave a handler by any jump, and the objects of the
handler end as they would on any exit ({principle}`OBJ-P2`). Control
cannot enter a try block or a handler by a jump, because entering a
handler without an exception, or a try block past its start, has no
meaning.
:::

:::{principle} TRY-P4
:rules: TRY-8

While a handler runs, the exception it caught is still the current
exception. `throw` with no operand passes that same object on to the
next enclosing try block, from the handler or from any function the
handler calls, and a change made through a reference parameter goes with
the object. A throw of a new value from a handler also goes outward. The
handlers of the same try block are not candidates for an exception that
one of these handlers throws. Leaving the handler by any route ends the
handled exception. `throw` with no operand and no current exception has
nothing to pass on, so the program ends.
:::

:::{principle} TRY-P5
:rules: TRY-9 TRY-10 TRY-17 TRY-11 TRY-12 TRY-13

A function try block makes the whole body of a function a try block,
including the member initializers of a constructor, and the handlers of
the function try block see the parameters. For an ordinary function the
function try block is a plain try block around the body. For a
constructor the handler runs on an object that failed to come into
being. The parts built so far are already destroyed when the handler
starts, so the handler cannot use those parts or return as if
construction succeeded. When control reaches the end of the handler, the
exception continues outward.

A destructor's handler works the same way. Flowing off the end of a
handler in a function that returns a value is as wrong as flowing off
the end of the function, and an implementation should warn.
:::

## Syntax

```{eval-rst}
.. productionlist:: try
   try_block: "try" `compound_statement` `handler` {`handler`}
   handler: "catch" "(" `exception_decl` ")" `compound_statement`
   exception_decl: `type_specifiers` [`declarator`] | "..."
   function_try_block: "try" [`ctor_initializer`] `compound_statement` `handler` {`handler`}
```

A `function_try_block` takes the place of a function body. The
*handler parameter* is the declaration in a handler's parentheses, and
the *catch-all handler* is the one written with `...`.

## Constraints

:::{rule} TRY-16
:kind: reject
:tests: exceptions/handlers/try-without-handler.cpp

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
:tests: exceptions/handlers/param-abstract-reference-ok.cpp exceptions/handlers/param-abstract.cpp exceptions/handlers/param-array-decays-to-pointer.cpp exceptions/handlers/param-auto.cpp exceptions/handlers/param-default-argument.cpp exceptions/handlers/param-function-type.cpp exceptions/handlers/param-incomplete-pointer.cpp exceptions/handlers/param-incomplete.cpp exceptions/handlers/param-qualified-name.cpp exceptions/handlers/param-reference-to-unknown-bound-array.cpp exceptions/handlers/param-rvalue-reference.cpp exceptions/handlers/param-storage-class.cpp exceptions/handlers/param-unnamed.cpp exceptions/handlers/param-void.cpp

The handler parameter must have a complete type that an object or a
pointer can have. The handler parameter must not:

- be an abstract class by value, or an incomplete class by value or by
  pointer,
- have an rvalue reference type, a reference to an array of unknown
  bound, or the type `void`,
- have a storage class, a default argument, a qualified name, or
  `auto`.

An array type decays to a pointer, as in a function parameter. The
parameter can have no name.
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
:tests: exceptions/handlers/ctor-try-block-return-rejected.cpp

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
:tests: exceptions/handlers/match-ambiguous-base-no.cpp exceptions/handlers/match-array-decays.cpp exceptions/handlers/match-base-by-reference.cpp exceptions/handlers/match-base-by-value-slices.cpp exceptions/handlers/match-cv-qualified.cpp exceptions/handlers/match-function-decays.cpp exceptions/handlers/match-no-arithmetic-conversion.cpp exceptions/handlers/match-no-user-conversion.cpp exceptions/handlers/match-nonconst-reference-to-exception-object.cpp exceptions/handlers/match-nullptr-to-pointer.cpp exceptions/handlers/match-pointer-qualification.cpp exceptions/handlers/match-pointer-to-base.cpp exceptions/handlers/match-private-base-no.cpp exceptions/handlers/match-void-pointer.cpp

A handler with parameter type `T` matches an exception object of type
`E` if one of these holds:

- `T` is `E`, or a reference to `E`, with any cv-qualifiers added.
- `T` is an accessible unambiguous base of `E`, or a reference to one.
- `T` and `E` are pointers, and `E` converts to `T` by a qualification
  conversion or a conversion to a base, or `E` is the type of `nullptr`.
- `E` is an array or function type and `T` is the pointer that `E`
  decays to.

An arithmetic conversion or a user-defined conversion does not make a
match. A handler by value copies the exception object, and a copy to a
base type slices the object. A handler by reference refers to the
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
:tests: exceptions/handlers/break-out-of-handler.cpp exceptions/handlers/goto-out-of-handler.cpp exceptions/handlers/return-from-handler.cpp

A handler can end early with one of these:

- `return`,
- `break`,
- `goto`.

The local objects of the handler then end, last made first, and the
exception that the handler was handling is over.
:::

:::{rule} TRY-8
:kind: required
:tests: exceptions/handlers/handler-not-active-for-own-throw.cpp exceptions/handlers/nested-try-in-handler.cpp exceptions/handlers/rethrow-copy-not-modified.cpp exceptions/handlers/rethrow-from-called-function.cpp exceptions/handlers/rethrow-modified-reference.cpp exceptions/handlers/rethrow-outside-handler-terminates.cpp exceptions/handlers/rethrow-to-enclosing.cpp exceptions/handlers/throw-new-from-handler.cpp

`throw` with no operand rethrows the exception that the innermost active
handler caught, to the next enclosing try block. The rethrow can occur
in the handler or in a function that the handler calls. A change made to
the exception object through a reference parameter is kept, and a change
to a by-value parameter is not. A `throw` of a new value in a handler
leaves the handler, and the other handlers of the same try block do not
catch the new exception. A try block inside a handler catches its own
exceptions. If no exception is being handled, `throw` with no operand
ends the program.
:::

:::{rule} TRY-9
:kind: required
:tests: exceptions/handlers/function-try-block-falls-off-end.cpp exceptions/handlers/function-try-block-main.cpp exceptions/handlers/function-try-block-sees-parameters.cpp exceptions/handlers/function-try-block.cpp

A function try block wraps the full body of a function. Inside the
handlers of the function try block the parameters are visible, and a
`return` there gives the function's result, so a handler in `main` can
set the exit status. In a function that returns nothing, reaching the
end of a handler returns from the function.
:::

:::{rule} TRY-10
:kind: required
:tests: exceptions/handlers/ctor-try-block-members-destroyed-first.cpp exceptions/handlers/ctor-try-block-new-throw.cpp exceptions/handlers/ctor-try-block-rethrows-at-end.cpp exceptions/handlers/ctor-try-block-sees-member-init-exception.cpp exceptions/handlers/dtor-try-block-rethrows.cpp

A function try block on a constructor also guards the member
initializers. Before the handler runs, the parts of the object that were
constructed are destroyed. If control reaches the end of the handler,
the exception is rethrown. A handler can throw a new exception instead.
A function try block on a destructor rethrows the same way.
:::

## Undefined Behavior

:::{rule} TRY-11
:kind: undefined
:tests: exceptions/handlers/ctor-try-block-member-named-in-handler.cpp

The members and bases are gone by the time a constructor's or
destructor's function try block reaches a handler. A handler that still
reads or writes one of these members or bases has undefined behavior,
and the program compiles all the same.
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
:tests: exceptions/handlers/function-try-block-non-void-no-return.cpp

When a function returns a value, its function try block's handlers
should not let control run off their end. An implementation should warn
about a handler where control can run off the end.
:::

:::{rule} TRY-13
:kind: recommended
:tests: exceptions/handlers/ctor-try-block-member-use-warns.cpp

An implementation should warn if a handler of a function try block on a
constructor or destructor names a member of the object.
:::

## Examples

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

The results are in `tests/RESULTS.md`. All eight compilers agree on 72
of 74 tests. EDG runs no programs, so its results for the run tests say
only that the programs are valid, and the harness sees a terminating
program only as a nonzero exit.

- TRY-12 and TRY-13: Clang warns under `-Wall`. GCC gives no warning
  for either. EDG rejects the function that flows off the end of its
  handler without any flag.
- TRY-11: the test only shows that the program compiles.

Inputs: clang-sema-stmt, clang-diag-sema, clang-diag-parse,
edg-statements.
