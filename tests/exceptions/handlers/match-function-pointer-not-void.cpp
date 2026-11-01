// tidal-rules: TRY-2
// tidal-expect: run
void act() {}
int main() {
  try { throw &act; }
  catch (const void*) { return 1; }
  catch (void (*f)()) { return f == &act ? 0 : 2; }
  return 3;
}
