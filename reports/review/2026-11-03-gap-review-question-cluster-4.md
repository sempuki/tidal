# Gap review: question cluster 4

Date: 2026-11-03
Role: source reader

Reference: the C++23 working draft in the local reference tree (the
final C++23 draft). The review compares each answer with that draft
only. The draft already contains the change, adopted as a defect report
against C++23, that lets a constant evaluation use a reference or an
object whose identity is unknown. Later changes are mentioned only
where they matter, and they are not in the reference.

## Result

One of the 14 answers conflicts with the reference.

- Q-0192 contradicts the reference, with doubt. In the draft, the
  default initialization of a class always calls a constructor, a
  trivial one included, so the draft treats the class differently from
  a scalar. The draft supports option 2, the answer Clang gives.

The other 13 answers agree with the reference. Q-0001, Q-0117, Q-0193,
and Q-0318 carry a doubt.

- Q-0320 follows one family (Clang) against two (GCC and EDG). The
  draft supports the one family, through two separate rules.
- Q-0307 has a case that no family rejects, the read through the old
  base pointer. The draft still makes that read undefined, so the
  provisional answer holds for both cases.
- Q-0193: the draft supports the read of the nested temporary when
  both temporaries have a const type. The front end's interpreter
  rejects that read, which does not match the provisional answer.

## Questions in the cluster

| Question | Answer (Tidal) | Verdict | Option the reference supports | Note |
|---|---|---|---|---|
| Q-0001 | Floating overflow during constant evaluation is undefined (option 1). | agrees, with doubt | option 1 | See below. |
| Q-0002 | Keep CE-28, EDG's acceptance of a floating division by zero is a defect (option 1). | agrees | option 1 | The draft makes a zero divisor undefined for every arithmetic type, floating types included, and gives no exception for a type that follows IEEE 754. Undefined behavior keeps an expression from being constant. |
| Q-0003 | The most negative int divided by minus one is undefined for both quotient and remainder (option 1). | agrees | option 1 | The draft states that when the quotient cannot be represented, both the quotient and the remainder operations are undefined. So the remainder is undefined even though its mathematical value is zero. |
| Q-0013 | A read of an array element that was never written is undefined, as for a scalar (option 1). | agrees | option 1 | See below. |
| Q-0090 (settled) | The use is valid when the evaluation does not read through the reference (option 1). | agrees | option 1 | See below. |
| Q-0117 | A union with no active member is a valid constant and a valid template argument (option 1). | agrees, with doubt | option 1 | See below. |
| Q-0192 | The zero counts for a class with a trivial default constructor (option 1). | contradicts, with doubt | option 2 | See below. |
| Q-0193 | Every extended temporary counts, and a read needs a const type (option 1). | agrees, with doubt | option 1 | See below. |
| Q-0212 | The cast from a private base gives a null pointer, and EDG's success is a defect (option 1). | agrees | option 1 | The draft's run-time check for a cast down or across needs the operand to designate a public base subobject. A private base subobject fails both branches of the check, in both probes, and a failed cast to a pointer gives a null pointer. Constant evaluation follows the same check. |
| Q-0303 | Constant evaluation accepts the nothrow forms (option 1). | agrees | option 1 | See below. |
| Q-0307 | Both uses are undefined, and constant evaluation rejects them (option 1). | agrees | option 1 | See below. |
| Q-0309 | The read after the destructor call is undefined (option 1). | agrees | option 1 | See below. |
| Q-0318 | A member call through a null pointer is undefined (option 1). | agrees, with doubt | option 1 | See below. |
| Q-0320 | A read of a volatile object through a plain glvalue is rejected (option 1). | agrees | option 1 | See below. |

### Q-0001 in more detail

- The draft makes an evaluation undefined when its result lies outside
  the values that the type can represent, and undefined behavior keeps
  an expression from being constant. A floating overflow then makes the
  evaluation not constant, as GCC and EDG find.
- The doubt is that the draft never defines that set of values for a
  floating type. A reader could count an infinity as a value of an IEEE
  754 type and treat the rounded result as in range, which is Clang's
  reading. The draft gives no example of floating overflow either way.
- The draft has a recommendation that results during translation match
  results at run time, and a note that the accuracy of floating
  operations is not fixed. Neither one makes the overflow defined.
- A NaN result fits the draft's other case, a result that has no
  mathematical value, so Clang's rejection of an infinity minus itself
  fits the draft.
- Option 3 is a change of class and is not supported or ruled out by
  the draft beyond this point.

### Q-0013 in more detail

- The draft makes an indeterminate value undefined when an evaluation
  produces it, with narrow exceptions for byte-like types, and its list
  of operations that keep an expression from being constant names a
  read of an object with an indeterminate value on its own.
- Neither rule treats an array element differently from a scalar.
- The next language version gives automatic variables that are never
  written a new class of behavior at run time. Constant evaluation
  still rejects the read there, so the change does not alter this
  answer.

### Q-0090 in more detail

- The draft, through the defect report named in the Reference
  paragraph, treats a reference whose referent is not known as bound to
  some object that lives through the whole evaluation. The evaluation
  can use the reference as long as it does not read a value through it
  or depend on the dynamic type.
- The draft's example for this rule calls a member function that reads
  no member through such a reference, and marks the call valid. That is
  the case of the test.
- Clang 18 predates the defect report, which explains its rejection.

### Q-0117 in more detail

- The draft's checks on the value of a constant look at scalar
  subobjects with an indeterminate value, at references, and at
  pointers. A union with no active member has no member whose lifetime
  has begun, so none of these checks finds a fault.
- The draft's rule for template argument equivalence names the case of
  two unions that both have no active member, and counts them as
  equivalent. That rule only makes sense if such a union can be a
  constant, which supports option 1 for both parts.
- The doubt is that the draft's rule on subobjects says each subobject
  must meet the checks, and does not say in so many words that an
  inactive member is skipped. A reader who counts the inactive member
  as a subobject with no value would reach GCC's result.

### Q-0192 in more detail

- The draft's definition of a constant initialized variable needs one
  of two things: an initializer, or a default initialization that
  performs some initialization. Then the full initialization must be a
  constant expression.
- In the draft, default initialization of a class type always selects
  and calls a constructor. Only a type that is neither a class nor an
  array gets no initialization. A trivial default constructor is still
  a call, so the class case meets the first condition.
- The trivial constructor leaves the members without a value. The
  draft's checks on the value of a constant reject a scalar subobject
  with an indeterminate value. So the variable is not constant
  initialized.
- The draft performs zero initialization only when constant
  initialization is not performed, and it calls every other
  initialization dynamic. The constructor call is then dynamic
  initialization, and the draft makes a constinit variable with dynamic
  initialization ill-formed. This is Clang's result.
- For a scalar and for an array of scalars, default initialization
  performs no initialization at all. Such a variable is not constant
  initialized, but it has no dynamic initialization either, so the
  draft accepts it. The draft therefore separates the class case from
  the scalar case, where option 1 joins them.
- The doubt is whether a call to a trivial constructor, which does
  nothing, should count as dynamic initialization. The draft has a term
  for default initialization by a trivial constructor that it uses for
  lifetime and for jumps past declarations, but it does not use that
  term in the rules for constant initialization. GCC and EDG appear to
  treat such a call as no initialization.

### Q-0193 in more detail

- The draft extends the lifetime of a temporary to that of the
  reference for every form in REF-4, including a member of a class
  temporary. A note in the draft applies the extension again to a
  temporary bound to a reference member of an extended temporary. The
  draft gives all forms the same lifetime.
- For the declaration, a constant reference must refer to an object
  with static storage, and a temporary qualifies if its value passes
  the checks on constant values. The draft gives no rule that depends
  on the form of the binding, so EDG's rejection has no basis in it.
- For the read, the draft lets a constant evaluation read a temporary
  whose lifetime is extended to that of a variable usable in constant
  expressions, when the temporary has a const, non-volatile literal
  type. The type need not be a scalar, so the read of a const class
  temporary is allowed, as Clang and EDG find. A temporary that is not
  const cannot be read, as every compiler finds.
- For the nested case, the draft lets a reference member of a usable
  object be used, and the inner temporary is extended to the lifetime
  of the variable. When both temporaries have a const type, the draft
  allows the read. Clang's acceptance fits. The front end's rejection
  of that read with OBJ-2 does not fit the provisional answer.
- The doubt is that the draft does not say plainly that an extended
  temporary has static storage. A core issue on the storage duration of
  temporaries is still open. The draft's text on static initialization
  speaks of temporaries with static storage, which supports the
  reading.

### Q-0303 in more detail

- The draft lets a constant evaluation use an allocating new expression
  when the selected allocation function is one of the replaceable
  global ones, and the storage is freed within the evaluation.
- The draft lists the nothrow forms among the replaceable global
  allocation functions. The draft does not exclude a new expression
  just because it has placement arguments. So the nothrow forms are
  allowed, which is option 1.
- The draft puts the declarations of these functions and of the
  nothrow tag object in the library header. Whether a program may
  declare them itself without the header is a matter for LSUP-6, and
  the draft gives no help there. That point may explain the front
  end's rejection of the test, but it does not bear on the option.

### Q-0307 in more detail

- The draft lets an old pointer reach a new object only when the old
  object is transparently replaceable by the new one. One condition is
  that neither object is a potentially overlapping subobject. A base
  class subobject is always potentially overlapping, so a new object
  created in a base subobject never replaces it in this sense.
- Creating the new object also ends the lifetime of the enclosing
  derived object, because its storage is reused by an object not
  nested in it.
- The old pointer then designates an object outside its lifetime, and
  the draft makes an access through it undefined. A constant
  evaluation must reject it. The fact that no family rejects the base
  case does not change the draft's answer. All three evaluators miss
  the case.
- For the const case, the draft makes it undefined to create an object
  in the storage of a const complete object with automatic, static, or
  thread storage. That rule applies before any read. So a constant
  evaluation must reject the creation itself, as Clang 22 does.
- The draft's special rule for the library construction function
  during constant evaluation only reduces the call to the constructor
  call. It does not excuse either case.
- I know of no later defect resolution that makes a base subobject
  transparently replaceable.

### Q-0309 in more detail

- In the draft, a class object stops being alive as soon as its
  destructor call begins, for a trivial destructor too.
- The draft defines an active union member as one whose object is
  within its lifetime. After the destructor call the member is not
  active, and the union has no active member.
- The draft's list of operations that keep an expression from being
  constant names a read of a member that is not active, separately from
  undefined behavior. A read of the destroyed member hits both. GCC's
  acceptance has no support in the draft.

### Q-0318 in more detail

- The draft rewrites the arrow form of member access as an indirection
  followed by the dot form. The indirection gives an lvalue for the
  object the pointer points to, and a null pointer points to no object.
- The draft makes a member access undefined when the result of the left
  operand is an object whose type does not match the type of the
  operand. With a null pointer there is no object at all. The draft
  does not say in so many words that the call is undefined, which is
  the doubt. A long-standing core issue on indirection through a null
  pointer was open at the time of this draft.
- A later core defect resolution makes indirection through a pointer
  that does not point to an object or function undefined. That change
  settles the case for option 1.
- The draft's rules on pointers to objects outside their lifetime also
  make a call to a member function through such a pointer undefined,
  which points the same way.

### Q-0320 in more detail

- The draft makes an access to an object of volatile type through a
  glvalue that is not volatile undefined. Undefined behavior keeps an
  expression from being constant.
- Separately, the draft lets a constant evaluation read an object whose
  lifetime began inside the evaluation only when the glvalue is not
  volatile and the object is not volatile. The draft checks the object
  as well as the glvalue. Here the object is volatile, so the read is
  not allowed even apart from the first rule.
- Both rules support option 1 and Clang's result. Option 2 needs the
  check to look at the type of the glvalue alone, and the draft names
  the object explicitly.
