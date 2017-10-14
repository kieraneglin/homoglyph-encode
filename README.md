# homoglyph_encode
*Name suggestions welcome*

## What is it?

`homoglyph_encode` is a tool to encode numbers using a custom charset where all characters are visually different.

## Where would I use this?

In some circumstances, strings provided to a user can be visually ambiguous.  Is it an uppercase "I" or a lowercase "l"?  "0" or "O"?  Depending on the user's font, these symbols can look virtually identical.  If a string is written down, the problem is only made worse.

Example use cases:
- Preventing visually ambiguous URLs as an attack vector
- Generating a *NON SECURE* passphrase that's easy to write down or dictate

## Why?

I was bored and wanted to try something new.

## Usage

```rust
// cargo.toml
homoglyph-encode = { git = "https://github.com/kieraneglin/homoglyph-encode" }
```
```rust
extern crate homoglyph_encode;

fn main() {
  homoglyph_encode::encode(<usize>); // Returns a String
  homoglyph_encode::decode(<&str>); // Returns a usize
}
```
See the docs for more info.

## Known Problems

`encode` only takes a `usize`.  I'm not familiar enough with rust to coerce any unsigned int into a `usize`. After a Google/StackOverflow session, it seems to be surprisingly hard to do this without bringing in external dependencies.  

An issue/PR would be appreciated!

## License

Copyright 2017

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
