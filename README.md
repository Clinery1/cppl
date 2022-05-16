# What is this?
An experimental language I am making.
I was inspired by
[this](https://softwareengineering.stackexchange.com/questions/95126/how-does-a-static-type-system-affect-the-design-of-a-prototype-based-language)
Stack Exchange question, and the [Structural type system](https://en.wikipedia.org/wiki/Structural_type_system) Wikipedia page.

# What does "statically-constrained object-oriented" mean?
It means that the objects can contain any fields, but the possible objects are constrained at "compile-time."
Lets use JavaScript for an example.
In JS, all objects are mutable; fields can be added and removed at any time.
The difference is that JS is a dynamically-typed language, so you have no idea if the fields you want to access actually exist.
With CPPL, the fields that you specify are guaranteed to exist at runtime within "safe" CPPL.

If you have a better term for this, then please file an issue suggesting it.

# Is this a memory-safe language?
While this is not the goal, I believe it is possible to make the language at least mostly safe.
As of now, I only have plans for an interpreter, but I have ideas for how to compile the language (see [compiled_idea.md](compiled_idea.md)).

# What is this thing called?
CPPL.
It is an acronym for "Clinery's prototype programming language."
It is required to be in all caps when referring to it in a document, but file extensions are all lowercase.
