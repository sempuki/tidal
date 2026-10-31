// tidal-rules: ODR-4
// tidal-expect: run
struct Tag { Tag(); Tag(const Tag&); Tag(Tag&&); };
Tag::Tag() {}
Tag make() { return Tag(); }
int main() { Tag made = make(); Tag direct = Tag(Tag()); (void)made; (void)direct; return 0; }
