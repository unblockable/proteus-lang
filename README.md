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

## Protocol specifications and instances

`protocol_spec::ProtocolSpecification` describes the protocol's structure:
message formats, field semantics, sequence, cipher, encryption directives, and
options. Structural generation choices produce different specifications.

`ProtocolBindings` supplies the literal field values and password. Field bindings
are keyed by both format and field name. `FixedString` and `FixedBytes` semantics
declare these slots; `RANDOM`, `PUBKEY`, and `PAYLOAD` remain runtime instructions.
The literal data includes what Python UPGen emits for greetings, type/version
bytes, reserved fixed bytes, subprotocol length bytes, and passwords.

Build an instance with:

```rust
let instance = ProtocolInstance::builder(specification)
    .bindings(bindings)
    .build(); // Option<ProtocolInstance>
```

The builder owns both inputs. It returns `None` for missing, extra, wrong-kind,
or ambiguous field bindings, or a password-presence mismatch. Completed instances
expose read-only access. These checks establish binding completeness, not protocol
validity: field widths, cryptographic rules, identifiers, and hex syntax are not
validated. The module documentation includes a complete construction example.

## PSF text output

PSF is a text representation of `ProtocolInstance`; there is no separate PSF
object or generator-settings model. Use `instance.to_string()` or
`write!(writer, "{instance}")` for canonical output. Rendering traverses the
specification in order, resolves its bindings, escapes quoted values, and
propagates writer failures. It generates no values and has no runtime dependencies.

The syntax model originated in UPGen's `src/upgen/psf.py` at commit
`8378953f6d26e712475b2fdd93e009b43516a76e`. Protocol generation and lowering are
not implemented yet. Business-rule validation remains separate.

Tests cover the instance builder, focused formatting behavior, unchanged golden
PSF examples, and a pinned snapshot of Proteus's grammar. Grammar compatibility
tests are isolated and do not require a sibling checkout.

Proteus's current parser accepts escape syntax but retains the escape spelling
in fixed strings and passwords instead of decoding it. Correcting that behavior
is a separate parser change; escaped values do not yet round-trip faithfully
through that parser.
