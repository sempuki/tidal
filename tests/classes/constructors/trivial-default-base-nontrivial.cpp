// tidal-rules: CTOR-23 JMP-6
// tidal-expect: reject
// tidal-error-clang: cannot jump from this goto statement
// tidal-error-gcc: crosses initialization
// tidal-error-edg: bypasses initialization
struct Base { Base() {} };
struct Item : Base {};
void run(bool skip) {
  if (skip) goto done;
  Item item;
done:
  (void)skip;
}
