# Command Alias: `mg` ↔ `multigit`

MultiGit now provides two command names for the same functionality:

- **`multigit`** - Full command name (explicit, great for scripts)
- **`mg`** - Short alias (fast typing, great for interactive use)

## Quick Reference

Both commands are **100% identical** - use whichever you prefer!

```bash
# These are exactly the same:
mg init              ←→  multigit init
mg sync              ←→  multigit sync
mg remote add ...    ←→  multigit remote add ...
mg status            ←→  multigit status
mg --help            ←→  multigit --help
```

## When to Use Which?

### Use `mg` when:
- Working interactively in your terminal
- You want to type less
- Quick one-off commands
- Muscle memory from other short Git commands (`git`, `gh`, etc.)

### Use `multigit` when:
- Writing scripts or automation
- Documentation that needs to be explicit
- CI/CD pipelines
- When clarity is more important than brevity

## Examples

### Interactive Terminal Use (mg)
```bash
cd my-project
mg init
mg remote add github myusername
mg remote add gitlab myusername
mg sync
```

### Script/Automation (multigit)
```bash
#!/bin/bash
# Sync script

cd /path/to/project
multigit sync --dry-run
if [ $? -eq 0 ]; then
    multigit sync
fi
```

## Installation

Both binaries are installed automatically:

```bash
# Install from cargo
cargo install multigit

# Both commands are now available
mg --version
multigit --version
```

## Help Text

Each binary shows its own name in usage examples:

```bash
$ mg --help
Usage: mg [OPTIONS] <COMMAND>
...

$ multigit --help  
Usage: multigit [OPTIONS] <COMMAND>
...
```

The help content is otherwise identical.

## Technical Details

- Both binaries are built from the same `src/main.rs`
- Binary name detection uses `env!("CARGO_BIN_NAME")`
- No performance difference - they're the exact same code
- File size: Both binaries are identical (can be hardlinked)

## Migration Guide

### If you've been using `multigit`:
- Everything still works exactly the same
- Start using `mg` whenever you want
- No action required

### If you prefer `mg`:
- Just start using it!
- All documentation examples work with either name
- Your existing `multigit` commands still work

## FAQ

**Q: Which is the "official" name?**  
A: `multigit` is the package name, but both are equally supported.

**Q: Will one be deprecated?**  
A: No. Both will be maintained indefinitely.

**Q: Can I use them interchangeably?**  
A: Absolutely! Mix and match as you like.

**Q: Is there any difference in features?**  
A: None. They're the exact same binary with different names.

**Q: What about shell completion?**  
A: Generate completion for both:
```bash
mg completion bash > ~/.local/share/bash-completion/completions/mg
multigit completion bash > ~/.local/share/bash-completion/completions/multigit
```

## Summary

Choose the name that feels right for your use case. There's no wrong choice! 🚀
