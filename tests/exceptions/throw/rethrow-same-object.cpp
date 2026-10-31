// tidal-rules: THR-6
// tidal-expect: run
int main() {
  const int* first = nullptr;
  try { try { throw 5; } catch (const int& e) { first = &e; throw; } }
  catch (const int& e) { return &e == first && e == 5 ? 0 : 1; }
  return 2; }
