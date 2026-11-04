// tidal-rules: UNW-2 UNW-6
// tidal-expect: run
// tidal-note: The body of a destructor throws. The members of the object are still destroyed, in reverse order, before the handler runs.
int seen[8];
int n = 0;
struct M { int id; ~M() { seen[n++] = id; } };
struct D { M a{1}; M b{2}; ~D() noexcept(false) { throw 3; } };
int main() {
  try { D d; } catch (int) { seen[n++] = 9; }
  return n == 3 && seen[0] == 2 && seen[1] == 1 && seen[2] == 9 ? 0 : 1;
}
