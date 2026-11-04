// tidal-rules: VIRT-1
// tidal-expect: run
// tidal-note: File::id overrides Reader::id and Writer::id. A call through either base reaches File::id.
struct Reader { virtual int id() const { return 1; } };
struct Writer { virtual int id() const { return 2; } };
struct File : Reader, Writer { int id() const override { return 3; } };
int main() {
  File f;
  const Reader& r = f;
  const Writer& w = f;
  return r.id() == 3 && w.id() == 3 ? 0 : 1;
}
