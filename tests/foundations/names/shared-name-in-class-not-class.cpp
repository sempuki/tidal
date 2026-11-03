// tidal-rules: NAM-41
// tidal-expect: reject
// tidal-error-clang: must use .struct. tag to refer to type .link
// tidal-error-gcc: link. does not name a type
// tidal-error-edg: member .list::link. is not a type name
// tidal-note: Inside the class, after the data member, the name alone finds the data member.
struct list { struct link { int v; }; int link = 0; link* head; };
