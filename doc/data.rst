Data
====

Programs create and manipulate data.
Data are immutable: once created, they cannot be changed.
Instead, new data can be created from old data,
perhaps with some parts being different.

Constituents
------------

Every datum consists of three parts.
How these parts are stored in memory is an
implementation detail,
but you may expect it to happen so efficiently.

enchantment
    A sigil used for dynamic dispatch.

pointer array
    Pointers to other values,
    used to create more complex data structures.

auxiliary part
    Arbitrary bytes,
    for efficiently storing information
    that does not consist of other data.

S-expressions
-------------

An S-expression is a textual representation of a datum.
The most basic type of S-expression is
a literal representation of a datum by its three parts.
Other types of S-expressions
are short-hands for common data structures.
Let us take a look at some examples of S-expressions::

    ;; A comment begins with a semicolon and extends until the end of the line.
    ;; Comments are ignored and may be used at will.

    ;; A literal datum, by its three parts, is delimited by #[ and ].
    #[ enchantment (pointer-array) auxiliary-part ]

    ;; A Boolean is either true or false.
    #t                  ; #[ bool () "\x01" ]
    #f                  ; #[ bool () "\x00" ]

    ;; An integer is 64-bit, little-endian, two's complement, and signed.
    123456              ; #[ int () "\x40\xE2\x01\x00\x00\x00\x00\x00" ]
    -123456             ; #[ int () "\xC0\x1D\xFE\xFF\xFF\xFF\xFF\xFF" ]

    ;; A list is either empty, or the prepending of an element onto a list.
    ()                  ; #[ nil  ()        "" ]
    (#t)                ; #[ cons (#t ())   "" ]
    (#t #f)             ; #[ cons (#t (#f)) "" ]
