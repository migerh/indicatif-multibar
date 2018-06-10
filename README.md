# indicatif-multibar
Showcase some indicatif multibar examples with weird behavior

indicatif displays some weird behavior when it comes to displaying
spinners while running a child process with Command::new(). This example
program showcases a few of them.

## Single spinner
This mostly works fine, the spinner spins but it gets duplicated once
the child process finishes.

## Multibar spinner
Using multiple spinners at the same time breaks the spinners - they
simply don't spin.

## Multibar spinner extended
If the spinners spin longer than the command runs they will start
spinning as soon as the child process finishes.
