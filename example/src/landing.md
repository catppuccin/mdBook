# Example Markdown Document

## Information

**crates.io**: [https://crates.io/crates/mdbook-catppuccin](https://crates.io/crates/mdbook-catppuccin)

**repository**: [https://github.com/catppuccin/mdBook](https://github.com/catppuccin/mdBook)

## Text

Here is a paragraph with bold text. **This is some bold text.** Here is a
paragraph with bold text. **This is also some bold text.**

Here is another one with italic text. _This is some italic text._ Here is
another one with italic text. _This is some italic text._

Here is another one with struckout text. ~~This is some struckout text.~~

<div class="warning">
  This is some text in a warning block.

  Warning blocks should be used sparingly in documentation, to avoid "warning
  fatigue," where people are trained to ignore them because they usually don't
  matter for what they're doing.
</div>

## Links

Autolink: <http://example.com>

Link: [Example](http://example.com)

Reference style [link][1].

[1]: http://example.com "Example"

## Images

<img src="https://avatars.githubusercontent.com/u/58985301" width="150">

## Headers

# First level title

## Second level title

### Third level title

#### Fourth level title

##### Fifth level title

###### Sixth level title

### Title with [link](http://localhost)

### Title with ![image](http://localhost)

## Code

Inline `code span in a` paragraph.

```
This
  is
    code
      fence
```

```java
public class Main {
  public static void main(String[] args) {
    System.out.println("Hello, World!");
    System.out.println("This is some Java code!");
  }
}
```

This is a code block:

    /**
     * Sorts the specified array into ascending numerical order.
     *
     * <p>Implementation note: The sorting algorithm is a Dual-Pivot Quicksort
     * by Vladimir Yaroslavskiy, Jon Bentley, and Joshua Bloch. This algorithm
     * offers O(n log(n)) performance on many data sets that cause other
     * quicksorts to degrade to quadratic performance, and is typically
     * faster than traditional (one-pivot) Quicksort implementations.
     *
     * @param a the array to be sorted
     */
    public static void sort(byte[] a) {
        DualPivotQuicksort.sort(a);
    }

## Quotes

> This is the first level of quoting.
>
> > This is nested blockquote.
>
> Back to the first level.

> A list within a blockquote:
>
> - asterisk 1
> - asterisk 2
> - asterisk 3

> Formatting within a blockquote:
>
> ### header
>
> Link: [Example](http://example.com)

## Html

This is inline <span>html</html>.
And this is an html block.

<table>
  <tr>
    <th>Column 1</th>
    <th>Column 2</th>
  </tr>
  <tr>
    <td>Row 1 Cell 1</td>
    <td>Row 1 Cell 2</td>
  </tr>
  <tr>
    <td>Row 2 Cell 1</td>
    <td>Row 2 Cell 2</td>
  </tr>
</table>

## Horizontal rules

---

---

---

## Lists

Unordered list:

- asterisk 1
- asterisk 2
- asterisk 3

Ordered list:

1. First
2. Second
3. Third

Mixed:

1. First
2. Second:

- Fee
- Fie
- Foe

3. Third

Definition list:

Some term
: First definition
: Second definition

Tables:

| Header 1 | Header 2 |
| -------- | -------- |
| Data 1   | Data 2   |
