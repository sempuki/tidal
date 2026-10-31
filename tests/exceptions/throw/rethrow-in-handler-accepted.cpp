// tidal-rules: THR-6
// tidal-expect: run
int level = 0;
int main() {
  try { try { throw 3; } catch (int& e) { ++level; throw; } }
  catch (int e) { return level == 1 && e == 3 ? 0 : 1; }
  return 2; }
