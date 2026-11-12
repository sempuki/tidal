// tidal-rules: LIST-4 LIST-6
// tidal-expect: accept
// tidal-note: A string literal initializes a whole character array member, so the next value goes to the next member.
struct Tag { char name[4]; int code; };
struct Entry { Tag tag; int count; };
constexpr Tag tag = {"abc", 1};
constexpr Entry entry = {"xy", 2, 3};
static_assert(tag.name[2] == 'c' && tag.name[3] == 0 && tag.code == 1);
static_assert(entry.tag.name[1] == 'y' && entry.tag.code == 2 && entry.count == 3);
