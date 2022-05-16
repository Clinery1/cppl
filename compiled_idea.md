# Memory layout
*size_in_bytes: name*

## Object
- 8: object_size
- 2: field_count
- object_size: fields/data

## Fields
- 7: field_id (compile-time generated number based on the name of the field)
- 1: (0..5) item_type; (5..8) flags
- 1..=object_size: data

# Flags
- 0: private, immutable
- 1: private, mutable
- 2: public, immutable
- 3: public, mutable
- 4: public, mutable(self)

# Item types
*name; ID (0..32); bytes taken up (1..=object_size) [?; comment]*
- Object; 0; object_size
- Bool; 1; 1
- Char; 2; 4
- Byte; 3; 1; unsigned, but not specified in the type name
- Int; 4; 8
- Uint; 5; 8
- SingleFloat; 6; 4; IEEE 745 single precision float
- Float; 7; 8; IEEE 754 double precision float
- Pointer; 8; 8
- MutablePointer; 9; 9
- AtomicBool; 10; 1
- AtomicByte; 11; 1
- AtomicInt; 12; 8
- AtomicUint; 13; 8
We keep both `SingleFloat` and `Float` for compatibility with graphics programs since GPUs have crippled `Float` performance for now.
`Byte` is in a similar boat, but more for the implementation of `String` in the standard lib and access to random bytes with pointers.

# Possible Implementation
Obviously this would not work in Rust, safe or unsafe, but it is basically how it would be implemented. For a VM, we would replace the `[ObjectField]` with `Vec<ObjectField>`. The major problem is that `ObjectFieldType` would be as large as `Object` which is not what we want, but it is an acceptable trade-off for a VM.
```rust
#[repr(u8)]
enum ObjectFieldType<'a> {
    Object(Object),
    Int(isize),
    Uint(usize),
    Char(u32),
    Float(f32),
    DoubleFloat(f64),
    Bool(bool),
    Ref(&'a Object),
    RefMut(&'a mut Object),
    Pointer(u64),
    PointerMut(u64),
    AtomicBool(AtomicBool),
    AtomicByte(AtomicU8),
    AtomicInt(AtomicI64),
    AtomicUInt(AtomicU64),
}


struct ObjectField<'a> {
    field_id:[u8;7],
    item_type:ObjectFieldType<'a>,
}
struct Object<'a> {
    /// includes this field and `field_count` sizes
    size:u64,
    field_count:u16,
    objects:[ObjectField<'a>],
}
```
