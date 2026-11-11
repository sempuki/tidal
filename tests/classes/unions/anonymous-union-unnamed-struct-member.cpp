// tidal-rules: UNI-9
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: anonymous types declared in an anonymous union
// tidal-error-edg: types cannot be declared in anonymous unions
// tidal-note: Roadmap round 10, programmer probe anonymous_struct_member_in_anonymous_union.cpp. Q-0424. The member pair of the anonymous union has an unnamed class type that its own declaration defines, so the anonymous union declares a nested type (UNI-9). EDG rejects the program, and Clang rejects it under -pedantic-errors, where Clang calls the form an extension. GCC accepts it under -pedantic-errors.
struct Bucket {
  union { struct { int key; int value; } pair; };
  Bucket() {}
};
int main() {
  Bucket b;
  b.pair.key = 2;
  return b.pair.key == 2 ? 0 : 1;
}
