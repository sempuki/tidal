# Objects, storage, and lifetime

An object is a region of storage with a type. Every object has a storage
duration that says when its storage exists, and a lifetime that says
when the object itself exists in that storage. Subobjects, temporaries,
and the order in which objects end all follow from this model. This
page states it once; the pages on initialization, temporaries,
constructors, and unions cite it.

Depends on: storage duration and linkage, value categories and
temporaries, constructors and destructors, new and delete.
