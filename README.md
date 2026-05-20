Regxact is a multi-platform tool designed to help developers not shoot themselves in the foot while using regex by providing an additional safety layer, and provides several macros that developers can build off of.

Regxact is a multi-language regex safety layer, with macros for common use cases that developers can easily build off of.

What it solves:
This simple regex pattern took down Cloudflare, cost over x billion dollars to fix, and was completely invisible until it was too late.
Regex is one of software's most common footguns. A malformed pattern can bypass authentication, crash a server, or allow an attacker into your system with nothing in your toolchain to warn you. If billion dollar companies with dedicated security teams don't catch these in testing, there's no reason to assume you will either. 

The traditional developer approach to regex is to avoid writing it entirely, by googling, asking AI, or copying from StackOverflow. But it doesn't adress the core problem: Regex is hard to read, hard to write, and extremely easy to mess up.

Regxact fixes this by providing developers with macros that cover common use cases of regex, so you don't have to write or even see raw regex in your codebase. For everything else, Regxact allows you to write Regex with an additional safety checker, that can also be used to extend off of the macros. Much of software is built around catching failures before they reach production - Regxact does that for Regex. 

Experienced developers can adopt Regxact quickly. It gets out of your way and makes regex safer without slowing you down.

`Example side by side`

In 2025, several platforms had catastrophic failures all caused by a malformed Regex pattern causing an unintended side effect (examples here). The traditional approach to writing regex by developers is to avoid thinking about them, usually meaning getting AI to generate a pattern, or by pasting regex from google. However, Regex is a language by itself, and can result in billions of dollars in damage if not thought about. The only way to avoid them is to know Regex like the back of your hand, or have enough test cases so that mistakes will get caught before being deployed. But if billion dollar companies like AWS and Cloudflare don't have enough test cases, then why assume that you do?

What it solves:
Image of a regex pattern, that caused a massive error. It shows a side by side, without regxact vs with. In the with, it shows that it produces an error. 
If you were unable to identify the error in the regex above, i dont blame you. 


Regxact solves this problem by giving premade, safe regex patterns, meaning you dont have to google or rely on AI to create them for you. Regxact gets out of your way, experienced developers can adopt Regxact quickly and make them even quicker. In fact, regxact makes using regex extremely simple, you don't evne have to touch it unless you want to (example of ipv4 or email here). But what if your use case isnt covered? You can expand or create your own regexes safely, using Regxact. Pictures here.

It uses WASM to translate rust code into multiple supported languages such as Go, Rust, Python, Typescript, etc and are available in several packages (see full availability here)

Full documentation
Much of software is aimed to stop failures before production
Regex is hard to read.


---

