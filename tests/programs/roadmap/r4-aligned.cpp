// tidal-rules: LSUP-5 NEW-26 ALN-1
// tidal-expect: run
// tidal-note: Roadmap round 4, programmer sketch aligned.cpp (reports/clean-room/roadmap-4/programmer.md, section 1.5). The rules are the ones the report names for this program and its reductions, and the rules of the main constructs it uses.
// tidal-note: A program test is evidence from use and decides no rule alone (decision 0054).
// Over-aligned elements in allocated storage (LSUP-5 shape).
using size_t = decltype(sizeof 0);
namespace std { enum class align_val_t : size_t {}; }
void* operator new(size_t, std::align_val_t);
void operator delete(void*, std::align_val_t) noexcept;
struct alignas(64) Line { int v; };
int main() {
  void* p = ::operator new(4 * sizeof(Line), std::align_val_t(alignof(Line)));
  Line* a = static_cast<Line*>(p);
  int bad = (reinterpret_cast<decltype(sizeof 0)>(p) % 64) != 0;
  ::operator delete(p, std::align_val_t(alignof(Line)));
  (void)a; return bad;
}
