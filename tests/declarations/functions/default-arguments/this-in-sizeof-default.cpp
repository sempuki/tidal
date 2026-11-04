// tidal-rules: DEFARG-4
// tidal-expect: reject
// tidal-error-clang: this. (outside of a non-static member function|may not be used in this context)
// tidal-error-gcc: this. (at top level|may not be used in this context)
// tidal-error-edg: this. may only be used inside a nonstatic member function
// tidal-note: A default argument cannot use this, also in the operand of sizeof, where a parameter is allowed.
struct Buffer {
  int size;
  int fill(unsigned long n = sizeof(*this));
};
int main() { return 0; }
