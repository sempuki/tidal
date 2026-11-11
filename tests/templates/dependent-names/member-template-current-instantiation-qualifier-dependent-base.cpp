// tidal-rules: DEP-24
// tidal-expect: reject
// tidal-error-clang: use 'template' keyword
// tidal-error-gcc: expected primary-expression
// tidal-error-edg: type name is not allowed
// tidal-note: Gap review RV6-B-9. The qualifier Wrap names the current instantiation, but lookup at the definition finds no make in Wrap, because only the dependent base T declares it. The name is then treated as after a dependent qualifier, and needs template. Every compiler rejects the form without template.
struct Base { template <class U> static U make() { return U(5); } };
template <class T> struct Wrap : T {
  static int build() { return Wrap::make<int>(); }
};
int main() { return Wrap<Base>::build() == 5 ? 0 : 1; }
