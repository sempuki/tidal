// tidal-rules: CTOR-23 JMP-6
// tidal-expect: accept
struct Item { int v; };
void run(bool skip) {
  if (skip) goto done;
  Item item;
done:
  (void)skip;
}
