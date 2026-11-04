// tidal-rules: TRY-2
// tidal-expect: run
// tidal-note: A handler of a reference to a pointer that is not const matches a pointer to a derived class, a pointer that gains a const pointee, and nullptr, on every compiler that runs programs. A gap review reads the C++23 text as allowing these conversions only for a by-value handler or a reference to a const pointer (documentation/decisions/pending.md, row 4).
struct B {};
struct D : B {};
int main() {
  D d;
  int i = 0;
  int r = 0;
  try { throw &d; } catch (B*&) { r += 1; } catch (...) {}
  try { throw &i; } catch (const int*&) { r += 2; } catch (...) {}
  try { throw nullptr; } catch (int*&) { r += 4; } catch (...) {}
  return r == 7 ? 0 : 1;
}
