# Brainrust

Brainfuck Virtual Machine written in Rust.

## Help

Available via `brainrust --help`.

Usage: `brainrust [OPTIONS] FILE`.

### Options

<ul>
  <li><code>c</code>,<code>--clean</code>&nbsp;&nbsp;&nbsp;&nbsp;Clean bytecode from <code>NOP</code> instructions.</li>
  <li><code>g</code>,<code>--dbg-level</code>&nbsp;&nbsp;&nbsp;&nbsp;Adjust debug level. <code>dbg_level</code> is an unsigned 32 bits integer.<br>
    &nbsp;&nbsp;&nbsp;&nbsp;0: Do not show any debug information.<br>
    &nbsp;&nbsp;&nbsp;&nbsp;1: Show generated instructions.
    &nbsp;&nbsp;&nbsp;&nbsp;2 or more: Show memory table, memory pointer and cleaning information.
</li>
</ul>

## License

This software is licensed under the GNU General Public License version 3.0 and later.
