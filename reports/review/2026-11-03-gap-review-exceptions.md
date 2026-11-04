# Gap review: the exceptions chapter

Date: 2026-11-03
Role: source reader

Reference: the C++23 working draft in the local reference tree. The
review compares the live rules of the three exceptions pages (throw
expressions, try blocks and handlers, unwinding and termination) and
the one open question that names a rule on these pages, Q-0179, with
that draft only. Later changes are mentioned only where the compilers
may follow them. Two quick probes on the host Clang and GCC informed
the doubt about THR-3.

## Result

Q-0179 rests on a point the reference does not address, so the
reference is silent on it.

Most rules agree. The gaps are these:

- THR-1 promises a move from a named local in cases where the
  reference also lets the implementation skip the move and build the
  object in place. The same holds for the copy into a by-value handler
  parameter (TRY-2, TRY-18). In the reference, an implementation that
  skips the handler copy never reaches the termination of TRY-18.
- THR-1 moves any local variable or parameter. The reference moves
  only a local object that is not volatile, or a local rvalue reference that names such an object. A volatile
  local and a local lvalue reference are copied. THR-1 also says nothing
  about an operand that is an xvalue.
- THR-3 leaves out a pointer to an incomplete type, which the
  reference rejects as the type of the exception object.
- THR-3 and THR-P1 ask only for the constructor that runs. The C++23
  text also asks for the copy constructor from an lvalue, even when no
  copy runs. The compilers do not enforce that. See "THR-3 and the
  copy constructor" below.
- TRY-2 uses "accessible" base where the reference asks for a public
  base. TRY-2 lets a reference to a pointer match by conversion, where
  the reference allows that only for a reference to a const pointer.
  TRY-2 leaves out the function pointer conversion for a pointer to a
  member function.
- TRY-5 limits the incomplete-type check to classes. The reference
  rejects a pointer or reference to any incomplete type other than
  pointer to void, so a pointer to an array of unknown bound is
  rejected too.
- TRY-9 links the end of a handler to the end of the function only for
  a function that returns nothing. TRY-10 does not say that a
  destructor's function try block also guards the destruction of the
  members and bases. TRY-11 covers reads and writes of data members
  only. The reference covers any use of a non-static member, member
  functions included.
- The handlers page does not say that the function try block of main
  misses exceptions from the construction and destruction of static
  objects.
- UNW-1 leaves out the case of an exception during the cleanup of a
  return statement. UNW-2 leaves out aggregate initialization, the
  closure object of a lambda, and a destructor that exits by an
  exception.
- UNW-3 picks the deallocation function by the allocation that ran.
  The reference picks it by lookup on the allocated type, and fixes
  the sized global form for a single object of complete type.
- UNW-5 leaves out the throwing initialization and destruction of
  objects with static storage duration. UNW-7 does not say that the
  other cases of UNW-5 unwind no further.

## Rules

| Rule | Verdict | Note |
|---|---|---|
| THR-1 | agrees in part | See "THR-1" below. |
| THR-2 | agrees | The reference strips top-level cv-qualifiers and turns array and function types into pointers. |
| THR-3 | missing condition, and a doubt | See "THR-3 and the copy constructor" below. |
| THR-4 | agrees | The type comes from the static type of the operand. |
| THR-5 | agrees | The reference gives the type void and allows the expression as an operand of the conditional operator. |
| THR-6 | agrees | The reference reuses the same object and makes no new one. |
| THR-7 | agrees | A rethrow with no handled exception ends the program. |
| THR-8 | agrees, one omission | The order of the handler's locals, its parameter, and the exception object matches the reference. The reference also keeps the object alive while a library handle to it exists, which THR-8 does not mention *(library)*. The reference lets the last of several possible end points be unspecified, which matters only across threads. |
| THR-9 | agrees | The search reaches the outer block of a non-throwing function and the program ends. A non-throwing function with no written specifier counts. |
| THR-10 | agrees | In C++23 a throw evaluated during constant evaluation is not constant. A later language change (C++26) allows exceptions inside constant evaluation. Compilers in a newer mode may accept programs that THR-10 and CE-8 reject. |
| THR-12 | silent | The reference does not say where an exception from the destructor of the exception object goes. Since the exception is caught by then, termination does not apply, and the propagation of THR-12 is a fair reading. |
| THR-13 | agrees | The reference leaves the storage unspecified and says that no global allocation function supplies it. A class allocation function is called only by a new expression, so it is not used either. |
| TRY-16 | agrees | The grammar needs at least one handler. |
| TRY-3 | agrees | |
| TRY-5 | missing condition | See "TRY-5" below. |
| TRY-14 | agrees | |
| TRY-7 | agrees | A jump or a switch label cannot enter a try block or a handler from outside. |
| TRY-17 | agrees | A return anywhere in the handler of a constructor's function try block is ill-formed. |
| TRY-1 | agrees | The search continues in the try block that dynamically encloses the current one. |
| TRY-2 | agrees in part | See "TRY-2" below. |
| TRY-6 | agrees | |
| TRY-15 | agrees, with one doubt | The reference ends the handler's locals, then its parameter, then the exception object. The reference does not say outright that the handler is still active while the locals end. The reading in TRY-15 fits the order the reference gives. |
| TRY-8 | agrees | The handlers of the same try block are not candidates, because the reference looks only at try blocks that control has entered and not yet left. |
| TRY-9 | agrees, one missing link | See "Function try blocks" below. |
| TRY-10 | agrees, one missing case | See "Function try blocks" below. |
| TRY-18 | agrees, with a missing condition | The reference ends the program when the copy into the handler parameter throws. The reference also lets the implementation skip that copy and treat the parameter as the exception object, as long as only constructor and destructor calls change. The reference's own example says the program ends only if the copy is not skipped. TRY-18 says the program always ends. |
| TRY-11 | narrower than the reference | See "Function try blocks" below. |
| TRY-4 | no counterpart | The reference only notes that a handler can be unreachable. It recommends no warning. The rule is Tidal's own. |
| TRY-12 | no counterpart | The reference makes the end of such a handler the same as the end of the function, which is undefined for a function that returns a value. The reference recommends no warning. |
| TRY-13 | no counterpart | The reference recommends no warning. |
| UNW-1 | agrees, one missing case | See "Unwinding" below. |
| UNW-2 | agrees, missing cases | See "Unwinding" below. |
| UNW-3 | partly different | See "Storage from a new expression" below. |
| UNW-4 | agrees | The reference retries the initialization of a static local the next time control reaches it. Unwinding touches only automatic objects. |
| UNW-5 | agrees, missing cases | See "Termination" below. |
| UNW-6 | agrees | |
| UNW-7 | agrees, one missing statement | See "Termination" below. |
| UNW-8 | no counterpart | The reference has no recommendation here. The reference only forbids an implementation to reject a program because a non-throwing function throws. |
| UNW-10 | agrees, with one detail | The default termination handler of the library aborts, and the status that the environment sees is the implementation's choice. Under the default handler, the reference forbids the destructors of static objects to run, so that part of UNW-10 is fixed by the reference and not a matrix choice *(library)*. |

## Questions

| Question | Provisional answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0179 | Warn when no enclosing handler catches by static type; a rethrow needs the catch-all handler (option 1). | silent | either | The reference recommends no warning at all for a throw in a non-throwing function. The reference only says that such a throw is valid and ends the program. Both options are Tidal's own choice. |

## THR-1

- The reference moves the operand only when the operand names a local
  object that is not volatile, or a local rvalue reference bound to
  such an object. A volatile local, and a local reference
  that is not an rvalue reference, are copied. THR-1 says "a local
  variable or a parameter" and so would move both.
- The scope condition matches. The reference looks at the innermost try
  block, or lambda body, whose block contains the throw. A variable
  inside that block is inside every outer one, so THR-1's "each try
  block" gives the same answer. Parameters may be moved in C++23, as
  THR-1 says. C++17 did not move them.
- For a local that is not a parameter, the reference also lets the
  implementation build the local directly in the exception object,
  with no move at all, when the same scope condition holds. THR-1 says
  the local "is moved", and the moved test counts exactly one move. An
  implementation that skips the move conforms to the reference and
  fails that test. As far as I know, Clang and GCC never skip it.
- An operand that is an xvalue initializes the exception object from
  the xvalue, which runs the move constructor. THR-1 lists prvalues,
  named locals, and "any other lvalue", and leaves xvalues out.
- A handler parameter is a variable with automatic storage duration in
  the reference. A throw of a handler parameter by name inside its own
  handler, with no try block around the throw, therefore moves it.
  THR-1 does not say whether a handler parameter counts as a local
  variable.

## THR-3 and the copy constructor

- The reference rejects an exception object of incomplete type or of
  abstract class type. The reference also rejects one whose type points
  to an incomplete type, unless the pointee is void. THR-3 covers the first two and leaves out the pointer case.
- The C++23 text asks, for a class exception object, that two
  constructors be non-deleted and accessible, even when no copy or move
  happens. One is the constructor chosen for the actual initialization.
  The other is the one a copy from an lvalue of that type would choose.
  Read as written, a type with a deleted copy constructor cannot be
  thrown at all, from a prvalue or by a move from a local.
- THR-P1 says that no copy or move needs to exist for a prvalue
  operand, and the prvalue test expects acceptance. THR-3 asks only for
  the constructor that runs. Clang and GCC on the host accept both a
  prvalue of a type with no copy or move constructor and a move-only
  local. The test results say all eight compilers accept the prvalue.
- The wording dates from before the guarantee that a prvalue
  initializes its target directly. What it means for a prvalue operand
  is unclear. I recall a core defect resolution in the C++26 cycle that
  restated the requirement as a copy from a const lvalue of the
  exception object's type, which would make a type with no copy
  constructor ill-formed to throw in every form. I cannot confirm its
  details or whether it applies to C++23. A question would record the
  doubt, with the compilers' answer as the provisional one.
- The reference always treats the destructor as potentially invoked,
  for a prvalue as well. THR-3 phrases the destructor check inside the
  sentence about copies and moves. The private-destructor test uses a
  prvalue and expects rejection, which matches the reference. The
  sentence in THR-3 can be read as narrower than its test.

## TRY-2

- The reference matches a base class only when the base is public and
  unambiguous. TRY-2 says "accessible". Inside a member of the derived
  class, a protected or private base is accessible. The reference still
  gives no match there. The pointer conversions to a base have the same
  limit in the reference.
- For the pointer cases and the null pointer case, the reference
  allows a by-value handler or a handler of reference to const
  pointer. A handler of non-const reference to pointer matches only the
  exact type, apart from cv-qualifiers on the pointer itself. TRY-2
  does not say this.
- The reference allows the function pointer conversion for pointers to
  members as well. A pointer to a non-throwing member function
  matches a handler for a pointer to the same member function without
  the non-throwing promise. TRY-2 allows only a qualification
  conversion for pointers to members.
- The last bullet of TRY-2 matches an array or function type of the
  exception object. THR-2 already makes that type a pointer, so the
  bullet never applies. The reference adjusts the handler's type
  instead (TRY-5), and notes that a handler of reference to array or
  reference to function never matches anything. The page does not cover
  that last point.
- A by-value handler copies the exception object, but the reference
  lets the implementation skip that copy (see TRY-18).

## TRY-5

- The reference rejects a handler whose type is incomplete, abstract,
  or an rvalue reference, and a pointer or reference to any incomplete
  type other than a pointer to void (with any cv). TRY-5 speaks of
  incomplete classes and names a reference to an array of unknown
  bound. A pointer to an array of unknown bound is also rejected in the
  reference. TRY-5 does not list it.
- The reference allows an attribute list at the start of the handler
  declaration, applying to the parameter. The Tidal grammar has no
  place for it.
- The rest matches. Abstract classes are rejected by value only. Array
  and function types become pointers. Placeholder types, qualified
  names, storage classes and default arguments have no place in a
  handler declaration.

## Function try blocks

- The reference makes the end of a handler in a function try block the
  same as the end of the function body, except for constructors and
  destructors. So the end of a handler of main returns zero, and the
  end of a handler of another function that returns a value is
  undefined. TRY-9 states only the case of a function that returns
  nothing. The undefined case is in the function definitions page, but
  TRY-9 does not link the handler end to it.
- The reference says a destructor's function try block also catches an
  exception from the destruction of the members and bases. TRY-10 says
  this for the member initializers of a constructor only. The
  reference also finishes the destruction of the remaining members and
  bases before the handler runs.
- TRY-11 makes reads and writes of members and bases undefined. The
  reference makes any reference to a non-static member or a base
  undefined in such a handler. That includes a call of a non-static
  member function and taking a member's address.
- The reference says the function try block of main does not catch an
  exception from the constructor of a static object outside any block,
  or from the destructor of a static object. The same holds for thread
  storage and the start function of a thread. The handlers page does
  not cover this.

## Unwinding

- The reference adds one case to UNW-1. If an exception leaves the
  destruction of a local or a temporary while a return statement is
  ending its block, the object being returned is destroyed too, after
  being built. UNW-1 does not mention it. I do not know how the
  compilers handle this case.
- The reference destroys, in reverse order, the parts of an object that
  were fully built, for every way of building the parts. The ways
  include constructors, a defaulted copy or move constructor, an
  inherited constructor, aggregate initialization, the captures of a
  lambda's closure object, and the elements of an array. UNW-2 speaks of
  constructors and arrays only. A throw from the third initializer of
  an aggregate destroys the first two members in the reference.
- If a member is a reference that extended the life of a temporary, the
  reference ends that temporary when the partly built object is
  cleaned up. UNW-2 does not mention it.
- If a destructor exits by an exception, the reference still runs the
  destructors of the members and bases that have not started. The
  unwinding page does not cover this, and the destructor rules of
  Tidal may or may not.

## Storage from a new expression

- The reference finds the deallocation function by lookup. For a class
  type, it looks in the class first, unless the expression asks for the
  global form, and then in the global scope. The choice does not depend
  on which allocation function ran. A class that declares only its own
  allocation function gets the global deallocation function on failure.
  UNW-3 says the class form is used when the allocation was the class
  form, which leaves that case unclear.
- For a placement form, the reference calls the deallocation function
  only when lookup finds exactly one whose extra parameters match. If
  a usual deallocation function of the class would match a placement
  allocation function, the program is ill-formed. UNW-3 does not
  mention that rejection.
- For the global case, the reference selects the sized form when the
  type is complete, except for an array whose element needs no
  destructor, where the choice is unspecified. For a single object of
  complete type the sized form is therefore required. UNW-3 calls the
  choice open. Clang 18's unsized call for a single object conflicts
  with the reference. Newer Clang turns sized deallocation on by
  default.
- The reference also prefers the deallocation form with an alignment
  parameter for a type with extended alignment. UNW-3 does not say so.

## Termination

- The reference ends the program in a few core-language cases that
  UNW-5 does not list. They are an exception that leaves the dynamic
  initialization of a static variable outside any block, or of a
  thread variable, and an exception that leaves the destructor of a
  static object or a thread-local object. UNW-5's "no matching
  handler" covers the first by effect. A static object with a throwing
  destructor is the case UNW-5 misses. The reference also lists
  library cases, such as a function registered to run at exit
  *(library)*.
- In the reference, the general rule behind TRY-18 is wider. Any
  function that the exception machinery calls directly, after the
  exception object exists and before a handler is active, ends the
  program if it exits by an exception. A destructor called by
  unwinding is the other case of this rule.
- UNW-7 matches the two choices that the reference leaves to the
  implementation. In every other case, the reference forbids unwinding
  before the program ends. A destructor that throws during unwinding,
  for example, stops all further unwinding. UNW-7 does not state this.
  The reference also forbids an implementation to stop unwinding early
  because it can see that termination will follow.
