// tidal-rules: DEL-3
// tidal-expect: reject
// tidal-error-clang: with a message is a C\+\+2c extension|expected .;. after delete
// tidal-error-gcc: reason only available with|expected .;. at end of member declaration
// tidal-error-edg: expected a .;.
// tidal-note: A member function deleted with a reason.
struct S {
  void f() = delete("use g");
};
