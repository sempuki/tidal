// tidal-rules: UNI-9
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: anonymous types declared in an anonymous union
// tidal-error-edg: types cannot be declared in anonymous unions
// tidal-note: Gap review RV6-A-17. Q-0424. An anonymous union inside an anonymous union declares an unnamed union type inside the outer one (UNI-9). EDG rejects the program, and Clang rejects it under -pedantic-errors, where Clang calls the form an extension. GCC accepts it under -pedantic-errors.
struct Cell {
  union { union { int whole; float real; }; char tag; };
  Cell() {}
};
int main() {
  Cell c;
  c.whole = 2;
  return c.whole == 2 ? 0 : 1;
}
