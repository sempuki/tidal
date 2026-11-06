// helper.cpp: `if consteval` takes one path in constant evaluation and
// another at run time (IF-18, IF-P4, TR-5). Checked in both modes.

enum class Mode { compile, run };

struct Result {
  int value;
  Mode mode;
};

consteval int cube_ct(int v) { return v * v * v; }  // compile time only (CEV-1)

// The same answer by two routes. The first branch is an immediate function
// context, so it can call cube_ct with a parameter (IF-19, IF-24).
constexpr Result cube(int v) {
  if consteval {
    return {cube_ct(v), Mode::compile};
  } else {
    int r = 1;
    for (int k = 0; k < 3; ++k) r *= v;
    return {r, Mode::run};
  }
}

constexpr Mode mode() {
  if !consteval {
    return Mode::run;
  } else {
    return Mode::compile;
  }
}

// Constant evaluation: static_assert and constexpr variables (CE-P4).
static_assert(cube(3).value == 27);
static_assert(cube(3).mode == Mode::compile);
static_assert(mode() == Mode::compile);
constexpr Result at_compile = cube(-2);
static_assert(at_compile.value == -8 && at_compile.mode == Mode::compile);

// A namespace-scope initializer that is a constant expression is
// constant-evaluated (IF-25, SINIT-1).
Mode global_mode = mode();

int main(int argc, char**) {
  // Run time: a non-const local with a non-constant argument.
  Result r = cube(argc + 1);  // argc is 1 with no arguments
  if (argc == 1 && r.value != 8) return 1;
  if (r.mode != Mode::run) return 2;

  // A non-const local is not one of the places of IF-25, so the call
  // runs at run time even though its argument is a constant.
  Mode m = mode();
  if (m != Mode::run) return 3;

  // These are places of IF-25, so the call is constant-evaluated:
  // a local const integer or enumeration (CE-3) and a static local.
  const Mode cm = mode();
  if (cm != Mode::compile) return 4;
  static Mode sm = mode();
  if (sm != Mode::compile) return 5;
  if (global_mode != Mode::compile) return 6;

  // Same value in both modes (TR-5).
  if (cube(-2).value != at_compile.value) return 7;
  return 0;
}
