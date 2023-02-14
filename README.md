# TextUtil
A small application to run assorted text operations.

#### Q:  Why?
I find myself opening various websites on the internet to paste in JWT tokens to decode, base64 to encode, JSON to format, etc.  I wanted something that would pop up when I needed it to and then go away.  Something extensible and easy to use with support for plugins like visual studio, but with a much smaller memory footprint.

#### Q: How?

That question is ambiguous, but I'm going to guess you mean, "How do I write a plugin for TextUtil?"

TextUtil uses a basic JS interpreter with dynamically loaded JS plugin files.  Each plugin 
