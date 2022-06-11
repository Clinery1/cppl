# gibberish I have to save before sleeping
Objects are exact constraints. They are basically structs, but they can have fields added and
removed to make them different objects.

Interfaces are a way to define predetermined behaviour for an object or constraint. They do not
store any data in the types they are implemented for and are limited to functions and types.

Constraints are a way to require certain features of an object, but they are more like generics.

Functions can specify either objects or constraints, but they act the same to the developer with the
exception that objects can only have one type fit into the variable/field, but constraints can have
a range of types.

Anything "undefined" cannot be accessed unless if it is through a pointer, which is unsafe.

# Unsafe
Anything relating to pointers except for their creation. Anything unsafe has to be enclosed in an
`unsafe` block.

# Builtin types
## UInt
64bit unsigned number

## Int
64bit signed number

## Byte
8bit unsigned number

## Float
32bit IEEE 754 single-precision floating point number

## DFloat
64bit IEEE 754 double-precision floating point number

## Slice (`[T]`)
An unsized list of type T. Usually referred to by reference since references have a size.

## Reference (`&T`)
A smart pointer to type T. There can be multiple references to a value at one time.

## Mutable reference (`&mut T`)
A smart mutable pointer to type T. Has exclusive access to the value. There can only be one
mutable reference to the value at a time.

## Reference (`&T`)
A pointer to type T.

## Mutable pointer (`&mut T`)
A mutable pointer to type T. Allows for changing the type pointed to by T, but only within the
allowed values

# Definitions
## Interface
A list of functions and types that are not stored in a type, but instead imported when the interface
is imported. The functions are virtual methods of the type the interface is implemented on. There is
no limit on the number of types an interface can be implemented on.

## Type
A type is a list of the properties of a specific value. The properties include: publicity,
mutability, the types of fields, and the signatures of methods.

Most types have a defined size, but all have a minimum size.

## Object
An object is the definition of the methods and fields on a type. When creating an object, the
methods and fields are guaranteed to exist and no others are allowed to exist. All methods and
fields defined in an object are guaranteed to exist even if they are private.

Objects can be thought of as constraints that have the exact methods and fields listed and no more.
This is known as an "exact constraint."

## Constraint
A constraint is a list of required methods and fields for a type. This is statically checked, so
code will not compile if a constraint is not met. The requirements are guaranteed to exist, but
other methods and fields may exist on a type however, these are hidden and are always assumed to be
undefined from the view of a constraint. Private fields are also viewed as undefined because private
fields cannot be accessed outside of their defined scope.

Since constraints can represent types of all sizes, they are unsized.

## Field
A named location containing a value
