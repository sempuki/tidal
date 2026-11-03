// tidal-rules: DEL-5
// tidal-expect: run
// tidal-sources: deleted-overload-in-two-units.tu.cpp
// tidal-note: Both units define the same deleted overload in a namespace. The overload that is not deleted is defined once.
namespace io {
void take(double) = delete;
int take(int v);
}
int other();
int main() { return io::take(2) == 4 && other() == 6 ? 0 : 1; }
