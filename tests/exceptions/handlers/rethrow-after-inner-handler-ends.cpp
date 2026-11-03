// tidal-rules: TRY-8
// tidal-expect: run
int main() {
  try {
    try { throw 1; }
    catch (int) {
      try { throw 2.0; } catch (double) {}  // the inner handler starts and ends
      throw;                                 // the handled exception is 1 again
    }
  }
  catch (int v) { return v == 1 ? 0 : 1; }
  catch (double) { return 2; }
  return 3;
}
