// tidal-rules: INST-7 INST-6
// tidal-expect: reject
// tidal-error-clang: cannot be used prior to
// tidal-error-gcc: .missing. is not a member of .int.
// tidal-error-edg: must be a class or namespace name
// tidal-note: I6-16. The call touch(held) needs the definition of the friend function that Wrap<int> defines, so the call instantiates its body, which is not valid for int.
template <class T> struct Wrap {
  friend void touch(Wrap) { T::missing(); }
};
void use() {
  Wrap<int> held;
  touch(held);
}
