# The Proteus Language for Encrypted Protocol Specification

The Proteus language is designed to express a large set of encrypted network
protocols using a compact, human-readable, and human-writable description. This
language is primarily used with the [Proteus
tool](https://github.com/unblockable/proteus) for programmable censorship
circumvention. [UPGen, the Unidentified Protocol Generator,](https://github.com/unblockable/upgen)
can be used to generate many random protocol specifications.

Eventually, we are planning to move all the `lang` modules for Proteus,
including the compiler, to this repo. For now, the details for how to parse and
interpret a protocol specification file live in `proteus.git/src/lang`.
