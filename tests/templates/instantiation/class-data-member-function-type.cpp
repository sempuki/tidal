// tidal-rules: INST-6
// tidal-expect: reject
// tidal-error-clang: data member instantiated with function type
// tidal-error-gcc: invalidly declared function type
// tidal-error-edg: a function type is not allowed here
// tidal-note: Roadmap round 9, programmer mistake member_of_function_type.cpp. A class template declares a data member of its type parameter. A specialization for a function type would declare a data member of function type, which is not valid.
template <class T> struct Box { T item; };
int twice(int x) { return 2 * x; }
template <class F> int hold(F&) {
  Box<F> b;
  (void)b;
  return 0;
}
int main() { return hold(twice); }
