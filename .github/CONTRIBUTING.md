# Contributing

When contributing to any repositories maintained by or falling under the scope
of Kludge Cyber Systems (hereon referred to as KCS), please first discuss the
change you wish to make via issue, email, or any other method with the
maintainer of the repository or lead members of the team the repository falls
under before making a change (this does not apply if you are an external
contributor submitting a pull request for a fork).

Note that we have a code of conduct (see [below][coc]) - please follow it in all
your interactions with the project.

[coc]: https://github.com/kludge-cs/orgfiles/tree/main/CODE_OF_CONDUCT.md

## Pull Request Process

1. Ensure any dependencies are updated to be as recent as possible,
   and all security issues are audited. Optionally, a dependency
   management automation system, like Depfu or Dependabot, will do
   this for you.
2. Ensure the branch passes all checks (especially lint and tests) and your
   contribution follows all our guidelines.
3. Update the CHANGELOG.md with details of changes and who authored them, except
   in repositories that make use of the automatic release workflow.
4. Increment the version numbers in any example files, README.md, and
   CHANGELOG.md to the new version that this Pull Request would represent.
   The versioning scheme we use is [semantic versioning][semver]. Once again,
   this is not necessary for repositories that use the automatic release
   workflow.
5. You may merge the Pull Request if any of the following are true (and, if you
   cannot merge yourself, you may ask the most recent reviewing contributor to
   do it for you):
   * *a)* The contributions are on `staging` targeted at the main branch, the
   source has been tested properly and passes all workflows *AND* has the
   approval of two organization members.
   * *b)* The contribution is on a `dev/*` branch targeted at `staging` and your
   code passes all tests and workflows.
   * *c)* The contribution is from an external branch targeted at `staging` and
   your code passes all tests and workflows.

[semver]: https://semver.org

## Standard of Work

This section is dedicated to laying out general rules for contributing to any
KCS project. If the repository you are working on has its own style guide, it
overrides this one. Else, all of these guidelines apply. For further
information, consult the respective lint configurations.

### Licensing

As per our license of choice, the [MIT license][license], compatibility checking
is largely not necessary. As long as our copyright header is included in
derivative works, and libraries are under compatible licenses (which is most of
them due to the nature of the MIT license), all is fine.

While you may be tempted to use our design philosophy to perpetuate restrictions
on which libraries are used, such as favouring those under similar licenses,
this practice is discouraged - education and discussion is preferred to cutting
usage of valuable and often superlative libraries based solely on their
licensing rather than quality.

[license]: LICENSE

### Formatting

Above all, ensure you retain our [design philosophy] and [stylistic
guidelines]. Of course, we are scientists, not iron-fisted rulers - if there is
a better way to do something, use it.

Note that we have adopted the inclusive naming initiative, and our acting
director is amongst those on the forefront of the [terminology shift].

* Tabs for indentation, spaces for formatting.
* UNIX style newlines.
* Trim all trailing whitespace.
* 80 characters per line of source code.
* Block declaration braces go on the same line as the condition or declarative
  statement.
* Declare only one variable per declarative statement, rather than comma
  separating.
* Use trailing commas.
* Keep functions at a minimal length, and create other functions to avoid
  complication.
* Make return statements clear and return as early as possible.
* Only use comments for parser / engine runtime statements or to clarify complex
  segments of code.

[design philosophy]: https://github.com/kludge-cs/orgfiles/tree/main/DESIGN.md
[stylistic guidelines]: https://github.com/kludge-cs/orgfiles/tree/main/STYLE.md
[terminology shift]: https://github.com/kludge-cs/transparency/tree/main/open-statements/2021-04-04--TERMINOLOGY.md

## Code of Conduct

See our [CoC document].

[CoC document]: https://github.com/kludge-cs/orgfiles/tree/main/CODE_OF_CONDUCT.md
