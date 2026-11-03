// tidal-rules: UNI-15 UNI-10
// tidal-expect: run
// tidal-note: Two bit-fields of one width and layout-compatible types match, so the sequence goes on past them to the int members.
struct Packet { unsigned kind : 4; int length; };
struct Header { unsigned tag : 4; int size; };
union Frame { Packet packet; Header header; };
int main() {
  Frame f{.packet = {9, 120}};
  return f.header.tag == 9 && f.header.size == 120 ? 0 : 1;
}
