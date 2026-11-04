// tidal-rules: LSP-1 LSP-2 LSP-3 LSP-4 LSP-5 LSP-6 LSP-7 TR-4 PFX-19
// tidal-expect: run
// tidal-note: Write suite W13, program clink.cpp (reports/clean-room/W13/report.md). The rules are the ones the report's notes made before writing give for this program.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// W13 task 3: a function with C linkage, called from C++ code.
// Rules used: LSP-1, LSP-2, LSP-3, LSP-4, LSP-6, LSP-7, TR-4, PFX-19.

namespace std {
using size_t = decltype(sizeof 0);
}

// LSP-1: the block form. The braces make no scope.
extern "C" {
  // A checksum with a C interface: a pointer and a count.
  unsigned c_checksum(const unsigned char* data, std::size_t count) {
    unsigned sum = 0;
    for (std::size_t i = 0; i < count; ++i) {
      sum = (sum << 1 | sum >> 31) ^ data[i];
    }
    return sum;
  }

  // A C callback type and a C callback interface. C code would call back
  // through the pointer. LSP-6: on EDG the type of fn records C linkage.
  // The alias in the block is outside knowledge: the page does not say
  // that a linkage specification reaches a function type in an alias.
  using c_callback = int(int);
  int c_apply(c_callback* fn, int value) { return fn(value); }

  // A callback with C linkage, so a pointer to it fits c_apply everywhere.
  int c_negate(int v) { return -v; }

  // LSP-1: static in a C block gives internal linkage.
  static int c_double(int v) { return 2 * v; }

  // LSP-3: a definition, because it would be one without the braces.
  int c_calls = 0;
}

// LSP-3: the single form declares and does not define.
extern "C" int c_triple(int v);

// LSP-4: a C name declared in a namespace is the same function as ::c_triple.
namespace driver {
extern "C" int c_triple(int v);
}

// The definition, later. LSP-4: no linkage specification is needed here,
// the function keeps C language linkage.
int c_triple(int v) {
  c_calls += 1;
  return 3 * v;
}

// A C++ function that takes the C callback type declared above.
int cpp_apply_twice(c_callback* fn, int value) { return fn(fn(value)); }

// LSP-4: a C function and a C++ function can overload.
extern "C" int c_scale(int v) { return v * 10; }
int c_scale(double v) { return static_cast<int>(v * 100.0); }

int main() {
  const unsigned char bytes[] = {1, 2, 3, 4};
  unsigned expected = 0;
  for (unsigned char b : bytes) expected = (expected << 1 | expected >> 31) ^ b;
  if (c_checksum(bytes, sizeof bytes) != expected) return 1;

  if (driver::c_triple(4) != 12) return 2;
  if (::c_triple(5) != 15) return 3;
  if (c_calls != 2) return 4;

  // A pointer to the C function and to the C++ function.
  if (c_apply(c_double, 21) != 42) return 5;
  // LSP-6: pass a function with C linkage. A C++ function here is
  // rejected by EDG, which records the linkage in the function type.
  if (c_apply(c_negate, 7) != -7) return 6;
  if (cpp_apply_twice(c_negate, 7) != 7) return 9;

  if (c_scale(3) != 30) return 7;
  if (c_scale(0.5) != 50) return 8;

  return 0;
}
