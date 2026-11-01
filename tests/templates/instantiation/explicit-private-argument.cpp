// tidal-rules: INST-34
// tidal-expect: accept
class Host { struct Secret {}; static int hidden(); };
template <class T> void run() {}
template <class T> struct Box {};
template <int (*F)()> struct Call {};
template void run<Host::Secret>();
template struct Box<Host::Secret>;
template struct Call<&Host::hidden>;
