// X24 probe r1: a jump fault inside a lambda body in a requires
// expression. A test of validity would see the fault only if it were
// part of the immediate context.
template <class T>
concept C = requires(T t) {
#ifdef DEP
  [](T u) { (void)u; break; };
#else
  [] { break; };
#endif
};
int main() { return C<int> ? 1 : 0; }
