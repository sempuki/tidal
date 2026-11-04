// tidal-rules: CTOR-12 OBJ-2
// tidal-expect: accept
// tidal-interpreter: run-fail
// tidal-note: The mem_init of a reads b, which the later mem_init of b has not yet initialized (CTOR-12, a case of OBJ-2). No sanitizer in the matrix stops the read, so the matrix compilers only build the program. The interpreter must stop with CTOR-12 or OBJ-2 (decisions 0052, 0053).
struct S {
  int a;
  int b;
  S() : a(b + 1), b(2) {}   // undefined: b has no value yet
};
int main() {
  S s;
  return s.a == 3 ? 0 : 1;
}
