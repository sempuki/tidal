// tidal-rules: TRY-10
// tidal-expect: run
// tidal-note: The destructor of member b throws. The function try block of ~D catches the exception after member a is also destroyed, and the end of the handler rethrows it. The order is 9 (the body), 3, 2, 1, the handler, then the caller.
int seen[8];
int n = 0;
struct M { int id; ~M() noexcept(false) { seen[n++] = id; if (id == 2) throw 7; } };
struct D {
  M a{1}; M b{2}; M c{3};
  ~D() noexcept(false) try { seen[n++] = 9; } catch (int v) { seen[n++] = 100 + v; }
};
int main() {
  try { D d; } catch (int v) { seen[n++] = 200 + v; }
  int want[] = {9, 3, 2, 1, 107, 207};
  if (n != 6) return 1;
  for (int i = 0; i < 6; ++i) if (seen[i] != want[i]) return 1;
  return 0;
}
