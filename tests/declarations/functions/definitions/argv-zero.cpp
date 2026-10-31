// tidal-rules: FN-6
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int main(int argc, char** argv) { return argc >= 1 && argv[0] != nullptr && argv[argc] == nullptr ? 0 : 1; }
