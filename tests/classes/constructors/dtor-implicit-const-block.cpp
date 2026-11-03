// tidal-rules: DTOR-2
// tidal-expect: run
// tidal-note: The class declares no destructor. Its implicit public destructor runs on a const object at the end of the block and destroys the member.
int destroyed = 0;
struct Part { ~Part() { ++destroyed; } };
struct Whole { Part part; };
int main() {
  {
    const Whole w{};
  }
  return destroyed == 1 ? 0 : 1;
}
