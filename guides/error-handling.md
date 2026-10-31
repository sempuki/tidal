# Handling errors

The task is to report a failure to a caller that can deal with the
failure. Every resource is released on the way. The
steps use exceptions. Step 7 shows where a function returns an error
value instead.

The examples in the steps form one program.

## Steps

### 1. Throw a class object that describes the failure

A `throw` expression makes the exception object from its operand. A
prvalue operand becomes the exception object directly, with no copy or
move ({rule}`THR-1`). A class hierarchy lets a handler choose between a
general and a specific failure (step 2).

```cpp
struct Error { int code; };
struct ParseError : Error { int line; };

int digit(char c, int line) {
  if (c < '0' || c > '9') throw ParseError{{1}, line};
  return c - '0';
}
```

These rules decide the exception object:

- The type of the exception object is the static type of the operand.
  A derived object thrown through a reference to its base gives a base
  object ({rule}`THR-4`).
- The operand must have a complete type that is not abstract. If the
  operand is copied or moved, the copy or move constructor must be
  usable ({rule}`THR-3`).
- The exception object lives until the last handler for it ends
  ({rule}`THR-8`).

### 2. Catch by reference, the specific type first

The handlers of a try block are tried in order, and the first match
runs ({rule}`TRY-1`). A handler for a base class matches a derived
exception object ({rule}`TRY-2`). So the handler for `ParseError` comes
before the handler for `Error`. An implementation should warn about a
handler that an earlier handler always shadows ({rule}`TRY-4`).

| Handler | Receives | Rule |
|---|---|---|
| `catch (const Error& e)` | The exception object itself | {rule}`TRY-2` |
| `catch (Error e)` | A copy of the `Error` part only. The derived part is sliced off. | {rule}`TRY-2`, {rule}`THR-8` |
| `catch (...)` | Any exception. This handler must be the last one. | {rule}`TRY-3` |

```cpp
int read_or_default(const char* text) {
  try {
    return digit(text[0], 1);
  } catch (const ParseError& e) {   // tried first
    return -e.line;
  } catch (const Error& e) {        // any other Error
    return -100 - e.code;
  } catch (...) {                   // everything else
    return -1000;
  }
}
```

A handler matches by type only. An arithmetic conversion or a
user-defined conversion does not make a match ({rule}`TRY-2`). If no
handler matches, the exception leaves the try block ({rule}`TRY-1`).

### 3. Pass the exception on with `throw;`

`throw` with no operand passes on the exception that the handler is
handling ({rule}`TRY-8`). The rethrown object is the same object. A
change through a reference parameter goes with the object
({rule}`THR-6`).

```cpp
void check(const char* text) {
  try {
    digit(text[0], 7);
  } catch (Error& e) {
    e.code += 10;                   // changes the exception object
    throw;                          // the same object continues outward
  }
}
```

### 4. Let destructors release resources during unwinding

When an exception leaves a block, the automatic objects of the block
are destroyed in reverse order. The destruction repeats for each block
up to the handler, and completes before the handler runs
({rule}`UNW-1`). Objects declared after the `throw` are not created
({rule}`UNW-1`).

So a resource that a class object owns is released on every route out
of the block. The guide "A class that owns a resource" shows such a
class. The `Buffer` below is a short version with deleted copy
functions ({rule}`DEL-1`).

```cpp
int trace = 0;

struct Buffer {
  int* data;
  explicit Buffer(int count) : data(new int[count]) {
    if (count > 100) { delete[] data; throw Error{2}; }   // see step 6
  }
  ~Buffer() { delete[] data; trace = trace * 10 + 1; }
  Buffer(const Buffer&) = delete;
  Buffer& operator=(const Buffer&) = delete;
};

void load(const char* text) {
  Buffer scratch(16);
  digit(text[0], 3);                // if digit throws, scratch is destroyed
  Buffer more(16);                  // not created if digit throws
}
```

Unwinding does not destroy objects with static storage duration
({rule}`UNW-4`). An object from `new` has no owner block, and only a
`delete` ends the object ({rule}`OBJ-5`).

### 5. Keep exceptions inside a `noexcept` function

A `noexcept` function promises that no exception leaves the function.
If an exception does leave, the program terminates, and no handler in a
caller runs ({rule}`THR-9`, {rule}`UNW-5`). The rule also applies when
the throw is in a function that the `noexcept` function calls
({rule}`UNW-5`).

A `noexcept` function can throw and catch exceptions inside itself
({rule}`UNW-6`). So a `noexcept` function that calls a throwing function
catches the exceptions of the call.

```cpp
int size_of(const char* text) noexcept {
  try {
    return digit(text[0], 1);
  } catch (const Error&) {          // the exception stays inside
    return 0;
  }
}
```

These functions are `noexcept` without the keyword:

| Function | Rule |
|---|---|
| A destructor whose bases and members have non-throwing destructors | {rule}`DTOR-4` |
| An implicit or defaulted special member function whose calls are all non-throwing | {rule}`EXS-4` |
| A deallocation function `operator delete` | {rule}`EXS-4` |

A function that the program writes, without the keyword, is
potentially throwing ({rule}`EXS-4`). An implementation should warn
about a `throw` written directly in a `noexcept` function outside a try
block that catches the exception ({rule}`UNW-8`).

### 6. Release partial resources when a constructor throws

If a constructor throws, the bases and members that the constructor
completed are destroyed in reverse order. The destructor of the object
itself does not run ({rule}`UNW-2`). Two designs follow from this rule:

| Design | What releases the resource |
|---|---|
| The constructor holds a raw pointer and then throws | Nothing, unless the constructor deletes the pointer before the `throw`. The `Buffer` constructor in step 4 does this. |
| Each resource is a member that owns the resource | The destructor of each completed member ({rule}`UNW-2`) |

```cpp
struct Pair {
  Buffer first;
  Buffer second;
  Pair(int a, int b) : first(a), second(b) {}   // if second throws,
};                                              // first is destroyed
```

If the constructor in a `new` expression throws, the storage goes back
through the matching deallocation function ({rule}`UNW-3`).

A function try block on a constructor also catches exceptions from the
member initializers. The parts are already destroyed when the handler
runs. At the end of the handler, the exception continues outward
({rule}`TRY-10`).

### 7. Return an error value where a `throw` cannot go

Two rules exclude a `throw` as the report:

- Constant evaluation must not evaluate a `throw` ({rule}`CE-8`). A
  function that reports a failure during constant evaluation returns
  the failure as a value.
- An exception that leaves a `noexcept` function terminates the program
  ({rule}`UNW-5`). A `noexcept` function reports a failure as a value.

A `constexpr` function can contain a `throw` on a path that constant
evaluation does not take. A call at run time then throws normally
({rule}`THR-10`).

```cpp
struct Parsed { bool ok; int value; };

constexpr Parsed parse_digit(char c) {
  if (c < '0' || c > '9') return {false, 0};
  return {true, c - '0'};
}
static_assert(parse_digit('7').value == 7);
static_assert(!parse_digit('x').ok);
```

The program below calls the functions of steps 1 to 6.

```cpp
int main() {
  if (read_or_default("x") != -1) return 1;
  if (read_or_default("4") != 4) return 2;
  try { check("x"); } catch (const Error& e) { if (e.code != 11) return 3; }
  trace = 0;
  try { load("x"); } catch (const Error&) { if (trace != 1) return 4; }
  if (size_of("x") != 0) return 5;
  trace = 0;
  try { Pair p(10, 1000); } catch (const Error& e) { if (trace != 1 || e.code != 2) return 6; }
  return 0;
}
```

## Mistakes

| Mistake | Example | Rule |
|---|---|---|
| Put `catch (...)` before another handler. | `catch (...) {} catch (const Error&) {}` | Not valid ({rule}`TRY-3`) |
| Rethrow when no exception is being handled. | `void cleanup() { throw; }` called outside a handler | The program terminates ({rule}`THR-7`) |
| Let an exception leave a `noexcept` function. | `int count(int c) noexcept { return parse(c); }` and `parse` throws | The program terminates ({rule}`THR-9`, {rule}`UNW-5`) |
| Throw from a destructor. | `~File() { if (handle < 0) throw Error{handle}; }` | The program terminates: the destructor is `noexcept` ({rule}`DTOR-4`, {rule}`UNW-5`) |
| Throw from a destructor that runs during unwinding. | A `noexcept(false)` destructor throws while another exception is unwound | The program terminates ({rule}`UNW-5`) |
| Throw from a constructor after a `new` into a raw pointer member. | `Pair(int a, int b) : first(allocate(a)), second(allocate(b)) {}` and the second `allocate` throws | The destructor does not run ({rule}`UNW-2`), so the first array is never deleted ({rule}`OBJ-5`) |
| Use a member in the handler of a constructor's function try block. | `catch (...) { int last = a.value; }` | Undefined: the member is already destroyed ({rule}`TRY-11`) |
| Return from the handler of a constructor's function try block. | `catch (...) { return; }` | Not valid ({rule}`TRY-17`) |
| Reach a `throw` during constant evaluation. | `static_assert(digit('x') == 0);` with a `constexpr` `digit` that throws | Not valid ({rule}`CE-8`) |
| Throw an lvalue of a class whose copy constructor is deleted. | `throw b;` where `b` is a global `Buffer` | Not valid ({rule}`THR-3`) |
| Let an exception leave `main`, or throw with no matching handler. | `int main() { throw Error{1}; }` | The program terminates ({rule}`UNW-5`) |
