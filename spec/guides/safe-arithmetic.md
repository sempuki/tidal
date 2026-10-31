# Integer arithmetic without undefined behavior

The task is to compute with integers so that no operation has undefined
behavior. The steps find the range and the type of each operation, and
check the operands before an operation that can leave the range.

## Steps

### 1. Know the range of each type

Every integer type uses two's complement. The size of the type gives
its bounds ({rule}`TYP-4`). The sizes come from the target
({rule}`TYP-1`), and `char` is signed on both matrix targets
({rule}`TYP-3`).

| Type | x86-64 Linux | `-m32` |
|---|---|---|
| `signed char`, `char` | -2<sup>7</sup> to 2<sup>7</sup> - 1 | the same |
| `short` | -2<sup>15</sup> to 2<sup>15</sup> - 1 | the same |
| `int` | -2<sup>31</sup> to 2<sup>31</sup> - 1 | the same |
| `unsigned` | 0 to 2<sup>32</sup> - 1 | the same |
| `long` | -2<sup>63</sup> to 2<sup>63</sup> - 1 | -2<sup>31</sup> to 2<sup>31</sup> - 1 |
| `unsigned long` | 0 to 2<sup>64</sup> - 1 | 0 to 2<sup>32</sup> - 1 |
| `long long` | -2<sup>63</sup> to 2<sup>63</sup> - 1 | the same |

A program that depends on a size can check the size with
`static_assert` ({rule}`SA-1`). The examples in this guide use the
constants below.

```cpp
static_assert(sizeof(int) == 4);         // both matrix targets
static_assert(sizeof(long long) == 8);   // both matrix targets

constexpr int int_max = 2147483647;      // 2^31 - 1
constexpr int int_min = -int_max - 1;    // -2^31
constexpr unsigned uint_max = 0u - 1u;   // 2^32 - 1: the subtraction wraps
static_assert(uint_max == 4294967295u);
```

### 2. Find the type that the operation uses

Two conversions change the operands before the operation:

1. {term}`Integer promotion` changes `bool`, the character types,
   `short`, and `unsigned short` to `int` ({rule}`CONV-1`).
2. The {term}`usual arithmetic conversions` give the two operands one
   common type ({rule}`CONV-19`). The result of `+`, `-`, `*`, `/`, and
   `%` has the common type ({rule}`ADD-4`, {rule}`MUL-5`).

The common type decides whether the operation wraps or is undefined
(step 3). The table shows common types that surprise.

| Operands | Common type, x86-64 | Common type, `-m32` | Step of CONV-19 |
|---|---|---|---|
| `unsigned short`, `unsigned short` | `int` | `int` | 2 |
| `int`, `unsigned` | `unsigned` | `unsigned` | 4 |
| `long`, `unsigned` | `long` | `unsigned long` | 5 on x86-64, 6 on `-m32` |

```cpp
using u16 = unsigned short;
static_assert(u16(1) - u16(2) == -1);    // int: the operands promote
static_assert(1u - 2u == uint_max);      // unsigned: the result wraps
static_assert(1u + -2 == uint_max);      // -2 converts to unsigned
static_assert((-1L + 0u < 1u) == (sizeof(long) == 8));
                                         // long on x86-64,
                                         // unsigned long on -m32
```

A comparison also uses the common type. If an `int` converts to
`unsigned`, the value -1 becomes the largest `unsigned` value
({rule}`CMP-3`). An implementation should warn about such a comparison
({rule}`CMP-4`).

### 3. Know which types wrap

An unsigned result wraps around. A signed result outside the range of
its type is undefined.

| Operation | Unsigned common type | Signed common type |
|---|---|---|
| `+`, `-` | Wraps ({rule}`ADD-5`) | Undefined outside the range ({rule}`ADD-9`) |
| `*` | Wraps ({rule}`MUL-8`) | Undefined outside the range ({rule}`MUL-10`) |
| Unary `-` | 2<sup>N</sup> minus the value ({rule}`UN-3`) | Undefined for the most negative value ({rule}`UN-4`) |
| Prefix `++`, `--` | Adds or subtracts 1 ({rule}`UN-20`) | Undefined outside the range ({rule}`UN-21`) |
| Conversion to a narrower type | Keeps the low bits ({rule}`CONV-2`) | Keeps the low bits ({rule}`CONV-2`) |

A conversion from one integer type to another is never undefined. A compound
assignment computes in the common type and then converts the result to
the type of the left operand ({rule}`ASN-5`). So `+=` on a `short`
computes in `int` and keeps the low bits.

```cpp
static_assert(uint_max + 1u == 0);                   // wraps
static_assert(static_cast<signed char>(200) == -56); // low 8 bits

constexpr short add_to_short(short s, int n) {
  s += n;                    // computed in int, then converted to short
  return s;
}
static_assert(add_to_short(32767, 1) == -32768);
```

### 4. Check a division before the division

`/` drops the fraction, toward zero ({rule}`MUL-6`). A nonzero result
of `%` has the sign of the left operand ({rule}`MUL-7`). Two cases are
undefined:

- a right operand of zero, for `/` and `%` ({rule}`MUL-9`),
- the most negative value divided by -1, for a signed type
  ({rule}`MUL-10`). The quotient does not fit in the type. `%` on the
  same operands is undefined too.

```cpp
static_assert(-7 / 2 == -3);
static_assert(-7 % 2 == -1);

constexpr bool divide_ok(int a, int b) {
  return b != 0 && !(a == int_min && b == -1);
}
static_assert(divide_ok(7, -1));
static_assert(!divide_ok(7, 0));
static_assert(!divide_ok(int_min, -1));
```

### 5. Check a shift count

The two operands of a shift promote separately. The result has the
promoted type of the left operand ({rule}`SHF-4`). A left shift is
defined for every value, also for a negative value and for a result
that changes the sign ({rule}`SHF-5`). A right shift rounds toward
negative infinity ({rule}`SHF-6`). A count is undefined if it is
negative, or if it is equal to or greater than the width of the
promoted left operand ({rule}`SHF-7`).

```cpp
static_assert(-1 << 3 == -8);
static_assert(1 << 31 == int_min);       // the sign changes: defined
static_assert(-7 >> 1 == -4);

constexpr bool shift_ok(int count) {     // for a left operand of type int
  return count >= 0 && count < 32;       // int has 32 bits
}
static_assert(shift_ok(31));
static_assert(!shift_ok(32));
static_assert(!shift_ok(-1));
```

### 6. Check the operands before the operation

A check must not do the operation that it guards. If the check computes
`a + b` first, the undefined behavior has already happened
({rule}`ADD-9`). Each check below uses only operations that stay in the
range:

- For `a + b` with `b > 0`, `int_max - b` cannot overflow. With
  `b <= 0`, `int_min - b` cannot overflow ({rule}`ADD-9`).
- For `a * b`, a product of two `int` values fits in `long long`,
  because `long long` has 64 bits on both matrix targets
  ({rule}`TYP-1`, {rule}`TYP-4`). The cast makes `long long` the common
  type ({rule}`CONV-19`).

```cpp
constexpr bool add_ok(int a, int b) {
  return b > 0 ? a <= int_max - b : a >= int_min - b;
}

constexpr bool multiply_ok(int a, int b) {
  long long product = static_cast<long long>(a) * b;
  return product >= int_min && product <= int_max;
}

constexpr bool checked_add(int a, int b, int& out) {
  if (!add_ok(a, b))
    return false;
  out = a + b;
  return true;
}

int main() {
  int sum = 0;
  bool done = checked_add(int_max, 1, sum);   // false: sum is unchanged
  return done ? 1 : 0;
}
```

### 7. Let constant evaluation find the undefined cases

{term}`Constant evaluation <constant evaluation>` must not evaluate an operation that has
undefined behavior ({rule}`CE-9`). A `static_assert` whose condition is
undefined makes the program not valid ({rule}`SA-7`). So a
`static_assert` that calls a `constexpr` function with the boundary
values tests the function for undefined behavior during translation.

- Call the operation itself with values at each bound. A value that
  overflows makes the implementation reject the program.
- Call each check with values at each bound. If the check itself
  overflows, the implementation rejects the program.

```cpp
constexpr int add(int a, int b) { return a + b; }

static_assert(add(int_max, 0) == int_max);
static_assert(add(int_min, 0) == int_min);
// static_assert(add(int_max, 1) < 0);   rejected: the addition is
//                                       undefined (ADD-9, CE-9)

static_assert(add_ok(int_max, 0) && !add_ok(int_max, 1));
static_assert(add_ok(int_min, 0) && !add_ok(int_min, -1));
static_assert(add_ok(int_min, int_max) && !add_ok(-1, int_min));
static_assert(multiply_ok(-1, int_max) && !multiply_ok(-1, int_min));
static_assert(multiply_ok(int_min, 1) && !multiply_ok(int_max, 2));
```

## Mistakes

| Mistake | Example | Rule |
|---|---|---|
| Add past the bound of a signed type. | `int_max + 1` | Undefined ({rule}`ADD-9`) |
| Check for overflow after the operation. | `if (a + b < a)` with `int` operands | Undefined: the addition comes first ({rule}`ADD-9`) |
| Negate the most negative value. | `-int_min` | Undefined ({rule}`UN-4`) |
| Increment past the largest value. | `++i` where `i` is `int_max` | Undefined ({rule}`UN-21`) |
| Multiply two `unsigned short` values and expect a wrap. | `u16(65535) * u16(65535)` | Undefined: the operands promote to `int` ({rule}`CONV-1`, {rule}`MUL-10`) |
| Compare a signed value with an unsigned value. | `-1 < 0u` is `false` | The value -1 converts to `unsigned` ({rule}`CMP-3`). An implementation should warn ({rule}`CMP-4`). |
| Divide or take the remainder by zero. | `a / 0`, `a % 0` | Undefined ({rule}`MUL-9`) |
| Divide the most negative value by -1. | `int_min / -1`, `int_min % -1` | Undefined ({rule}`MUL-10`) |
| Shift by the width of the type or more. | `1 << 32` | Undefined ({rule}`SHF-7`) |
| Shift by a negative count. | `1 << -1` | Undefined ({rule}`SHF-7`) |
| Shift a narrow type and expect its width. | `char c = 1; c << 20` is `1 << 20` in `int` | The left operand promotes ({rule}`SHF-4`) |
| Assume that `long` has 64 bits. | `long` has 32 bits on `-m32` | Implementation-defined ({rule}`TYP-1`) |
| Assume one common type on every target. | `-1L + 0u` | The type depends on the sizes ({rule}`MUL-15`, {rule}`CONV-19`) |
| Convert a floating-point value that does not fit. | `int big = 1e10;` | Undefined ({rule}`CONV-5`) |
