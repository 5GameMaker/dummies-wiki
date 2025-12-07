# Wiki article structure

Wiki is stored in markdown format.

All articles start with the header that is then used as the page's title, following
a short description.

## Routing documents

If a document is meant to be an intermediate for its subdocuments (usually `readme.md`),
at the end it has a `## Contents` section with a list of wikilinks pointing to the
other markdown documents without the extension.

## References

References must use Markdown's citation syntax (aka footnotes)[^1]. Old documents may still use the `# Reference`
section.

Footnotes are ordered based on when they were added. Re-ordering them is unnecessary.

[^1]: https://markdownguide.offshoot.io/extended-syntax/
