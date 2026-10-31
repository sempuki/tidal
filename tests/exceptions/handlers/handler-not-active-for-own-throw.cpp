// tidal-rules: TRY-8
// tidal-expect: run
int main() { try { try { throw 1; } catch (int) { throw 2; } catch (int) { return 1; } } catch (int v) { return v == 2 ? 0 : 2; } return 3; }
