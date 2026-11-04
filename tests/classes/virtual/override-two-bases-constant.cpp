// tidal-rules: VIRT-1
// tidal-expect: accept
// tidal-note: File::id overrides the id of both bases. Constant evaluation reaches File::id through either base.
struct Reader { constexpr virtual int id() const { return 1; } };
struct Writer { constexpr virtual int id() const { return 2; } };
struct File : Reader, Writer { constexpr int id() const override { return 3; } };
constexpr int through_reader(const Reader& r) { return r.id(); }
constexpr int through_writer(const Writer& w) { return w.id(); }
static_assert(through_reader(File{}) == 3);
static_assert(through_writer(File{}) == 3);
