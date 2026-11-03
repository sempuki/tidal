// tidal-rules: TRY-15
// tidal-expect: run
int seen = 0;
struct Probe {
  ~Probe() {
    try { throw; }                  // the handler's exception is still handled
    catch (int v) { seen = v; }
  }
};
int main() {
  try { throw 5; }
  catch (int) { Probe p; }          // p ends before the handled exception
  return seen == 5 ? 0 : 1;
}
