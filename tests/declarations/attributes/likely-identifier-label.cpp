// tidal-rules: ATTR-8
// tidal-expect: run
// tidal-expect-clang: reject
// tidal-error-clang: .likely. attribute cannot be applied to a declaration
// tidal-note: Q-0347. [[likely]] on a label that is an identifier, the target of a goto. GCC and EDG accept. Clang rejects the attribute on an identifier label, though it accepts the attribute on a case label.
int main() {
  int n = 2;
  if (n == 2) goto done;
  return 1;
  [[likely]] done:
  return n == 2 ? 0 : 2;
}
