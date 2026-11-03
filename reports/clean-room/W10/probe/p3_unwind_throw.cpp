// A noexcept(false) rollback that throws during unwinding.
struct Rollback { ~Rollback() noexcept(false) { throw 2; } };
int main() {
  try { Rollback r; throw 1; } catch (...) { return 0; }
  return 3;
}
