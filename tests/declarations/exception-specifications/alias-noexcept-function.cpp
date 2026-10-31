// tidal-rules: EXS-2
// tidal-expect: run
int calls = 0;
void work() noexcept { ++calls; }
using Safe = void() noexcept;
Safe* pick() { return work; }
int main() { pick()(); return calls == 1 ? 0 : 1; }
