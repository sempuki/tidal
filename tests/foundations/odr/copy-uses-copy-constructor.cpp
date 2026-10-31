// tidal-rules: ODR-4 ODR-8
// tidal-expect: link-fail
// tidal-error-clang: undefined reference to .Tag::Tag\(Tag
// tidal-error-gcc: undefined reference to .Tag::Tag\(Tag
struct Tag { Tag(); Tag(const Tag&); };
Tag::Tag() {}
int main() { Tag first; Tag second = first; (void)second; return 0; }
