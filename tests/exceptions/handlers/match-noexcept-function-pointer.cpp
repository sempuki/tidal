// tidal-rules: TRY-2
// tidal-expect: run
void act() noexcept {}
int main() {
  try { throw &act; }                       // void (*)() noexcept
  catch (void (*f)()) { return f == &act ? 0 : 1; }
  catch (...) { return 2; }
  return 3;
}
