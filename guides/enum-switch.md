# Enumerations in switch statements

The task is to define an enumeration and to select an action for each
of its enumerators with a `switch`. The steps also handle a value that
matches no enumerator, and the conversion of an integer to the
enumeration.

## Steps

### 1. Choose a scoped or an unscoped enumeration

| | Unscoped: `enum` | Scoped: `enum class` |
|---|---|---|
| Enumerator names | In the enclosing scope ({rule}`ENUM-2`) | Named through the enumeration ({rule}`ENUM-3`) |
| Conversion to an integer | Implicit, by {term}`integer promotion` ({rule}`ENUM-2`) | Only with an explicit cast ({rule}`ENUM-3`) |
| Arithmetic and comparison with an integer | Valid ({rule}`ADD-2`) | Rejected ({rule}`ENUM-16`) |
| Use as a condition | Valid | Rejected ({rule}`ENUM-11`) |
| Underlying type without `: type` | Chosen by the implementation ({rule}`ENUM-5`) | `int` ({rule}`ENUM-4`) |

A scoped enumeration keeps its values apart from the integers. Each
enumerator without a value gets the value of the previous enumerator
plus one, and the first gets 0 ({rule}`ENUM-1`).

```cpp
enum Light { Off, Dim, Bright };                      // 0, 1, 2
enum class Mode : unsigned char { Read, Write, Append };

int level = Dim;                                // 1: Dim promotes to int
int code = static_cast<int>(Mode::Write);       // 1: a cast is needed
```

### 2. Decide which values the enumeration holds

The underlying type decides which values a variable of the enumeration
can hold ({rule}`ENUM-6`):

| Underlying type | Values the enumeration holds |
|---|---|
| Fixed, with `: type`, or scoped | Every value of the underlying type |
| Not fixed | The values of the smallest set of bits that holds every enumerator |

`Light` above has no fixed type. Its enumerators fit in 2 bits, so
`Light` holds 0 to 3. `Mode` holds every value of `unsigned char`, 0 to
255. A value can be valid for the enumeration and still match no
enumerator, such as 3 for `Light`.

### 3. Write one case for each enumerator

The condition of a `switch` can have an enumeration type
({rule}`SW-2`). For a scoped enumeration, each case value must have the
enumeration type ({rule}`SW-3`). `using enum` puts the enumerators of
the enumeration in the current scope, so the case labels can name the
enumerators without `Mode::` ({rule}`ENUM-9`).

```cpp
constexpr const char* name(Mode m) {
  using enum Mode;
  switch (m) {
    case Read:   return "read";
    case Write:  return "write";
    case Append: return "append";
  }
  return "unknown";              // step 5
}
static_assert(name(Mode::Write)[0] == 'w');
```

### 4. Mark each fall-through with `[[fallthrough]]`

Control goes to the matching label and then runs through the later
labels until a `break` ({rule}`SW-1`). An implementation can warn when
control runs from one case into the next. `[[fallthrough]];`
immediately before the next label marks the fall-through as intended,
and the implementation should then not warn ({rule}`ATTR-7`). The
statement after `[[fallthrough]];` must have a `case` or `default`
label ({rule}`ATTR-2`).

```cpp
constexpr int cost(Light l) {
  int total = 0;
  switch (l) {
    case Bright:
      total += 2;
      [[fallthrough]];           // continues into case Dim
    case Dim:
      total += 1;
      break;
    case Off:
      break;
  }
  return total;
}
static_assert(cost(Bright) == 3);
static_assert(cost(Dim) == 1);
static_assert(cost(Off) == 0);
```

### 5. Handle a value that matches no enumerator

If no case value matches and the `switch` has no `default` label,
control goes to the end of the `switch` ({rule}`SW-1`). A function that
returns a value must not reach the end of its body, because the
behavior is then undefined ({rule}`FN-3`). So a function like `name` in
step 3 returns after the `switch`. An implementation should warn if
control can reach the end of such a function ({rule}`FN-7`).

```cpp
static_assert(name(Mode{7})[0] == 'u');           // 7 is a valid Mode
static_assert(cost(static_cast<Light>(3)) == 0);  // 3 is a valid Light
```

Tidal has no rule yet on a warning for an enumerator that has no
`case`.

### 6. Convert an integer to the enumeration

An integer does not convert implicitly to an enumeration
({rule}`ENUM-15`, {rule}`ENUM-16`). These forms convert:

| Form | Valid for | Rule |
|---|---|---|
| `static_cast<E>(v)` | Every enumeration | {rule}`CAST-1`, {rule}`ENUM-3` |
| `E{v}` | An enumeration with a fixed underlying type, if the conversion to that type does not narrow | {rule}`ENUM-8`, {rule}`ENUM-19` |

For an enumeration without a fixed type, a value outside the range of
step 2 makes the conversion undefined ({rule}`ENUM-6`, {rule}`CAST-14`).
So check the value before the cast.

```cpp
constexpr bool to_light(int v, Light& out) {
  if (v < Off || v > Bright)
    return false;                // no enumerator has this value
  out = static_cast<Light>(v);
  return true;
}

int main() {
  Light l = Off;
  bool ok = to_light(2, l);      // true: l is Bright
  bool bad = to_light(9, l);     // false: l is unchanged
  return ok && !bad && l == Bright ? 0 : 1;
}
```

## Mistakes

| Mistake | Example | Rule |
|---|---|---|
| Assign an integer to an enumeration. | `Light l = 1;` | Rejected ({rule}`ENUM-15`) |
| Initialize with `=` and a braced integer. | `Mode m = {1};` | Rejected ({rule}`ENUM-19`) |
| Initialize from braces with a value that narrows. | `Mode m{300};` | Rejected ({rule}`ENUM-19`) |
| Cast a value outside the range of an enumeration without a fixed type. | `static_cast<Light>(4)` | Undefined ({rule}`ENUM-6`, {rule}`CAST-14`) |
| Name an enumerator of a scoped enumeration without the enumeration. | `Mode m = Read;` without `using enum` | Rejected ({rule}`ENUM-16`) |
| Use an integer case value for a scoped enumeration. | `case 1:` in `switch (m)` | Rejected ({rule}`SW-3`) |
| Give two cases the same value. | `case Dim: case 1:` in `switch (l)` | Rejected ({rule}`SW-3`) |
| Use a scoped value as a condition. | `if (m)` | Rejected ({rule}`ENUM-11`) |
| Compute with a scoped value. | `Mode::Read + 1` | Rejected ({rule}`ENUM-16`) |
| Step through an enumeration with `++`. | `++l` | Rejected ({rule}`ENUM-16`, {rule}`PFX-13`) |
| Put `[[fallthrough]]` before a statement with no label. | `case Dim: [[fallthrough]]; total = 0;` | Rejected ({rule}`ATTR-2`) |
| Jump to a case past an initialized variable. | `case Dim: int n = 1; case Bright:` | Rejected ({rule}`SW-5`) |
| Reach the end of a function after a `switch` with no match. | `name` in step 3 without its last `return` | Undefined when no case matches ({rule}`FN-3`) |
