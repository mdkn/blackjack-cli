# Claude Code Hooks Documentation

This directory contains an example Claude hooks configuration file (`.claude-hooks-example.json`) that demonstrates how to set up custom hooks for Claude Code.

## What are Claude Hooks?

Claude hooks are shell commands that execute automatically in response to Claude's tool usage. They allow you to:
- Add safety checks before dangerous operations
- Automatically format code after edits
- Create backups before file modifications
- Log Claude's actions for audit purposes
- Block certain operations based on custom rules

## Available Hook Types

1. **beforeEdit** - Runs before Claude edits a file
2. **afterEdit** - Runs after Claude edits a file
3. **beforeWrite** - Runs before Claude creates/overwrites a file
4. **afterWrite** - Runs after Claude creates/writes a file
5. **beforeRead** - Runs before Claude reads a file
6. **afterRead** - Runs after Claude reads a file
7. **beforeBash** - Runs before Claude executes a bash command
8. **afterBash** - Runs after Claude executes a bash command

## Hook Configuration

Each hook can have:
- `description`: Human-readable description of what the hook does
- `command`: Shell command to execute
- `condition`: (Optional) Only run if this condition is true
- `blockOnError`: (Optional) If true, blocks Claude's action on hook failure
- `workingDirectory`: (Optional) Directory to run the command in

## Available Variables

- `{{file_path}}`: Path of the file being operated on
- `{{command}}`: Bash command being executed
- `{{line_count}}`: Number of lines in a file
- `{{old_content}}`: Previous content (for edits)
- `{{new_content}}`: New content (for edits/writes)

## Setup Instructions

1. Copy the example file to create your own hooks:
   ```bash
   cp .claude-hooks-example.json ~/.config/claude/hooks.json
   ```

2. Or set the CLAUDE_HOOKS_FILE environment variable:
   ```bash
   export CLAUDE_HOOKS_FILE="/path/to/your/hooks.json"
   ```

3. Customize the hooks according to your needs

## Example Use Cases

### 1. Auto-formatting
```json
{
  "afterEdit": [{
    "description": "Format Rust code",
    "command": "rustfmt {{file_path}}",
    "condition": "{{file_path}} =~ /\\.rs$/"
  }]
}
```

### 2. Safety checks
```json
{
  "beforeBash": [{
    "description": "Block dangerous commands",
    "command": "if [[ '{{command}}' =~ 'rm -rf /' ]]; then exit 1; fi",
    "blockOnError": true
  }]
}
```

### 3. Audit logging
```json
{
  "afterEdit": [{
    "description": "Log edits",
    "command": "echo '[$(date)] Edited: {{file_path}}' >> ~/claude-audit.log"
  }]
}
```

## Security Considerations

- Hooks run with your user permissions
- Be careful with commands that modify files
- Use `blockOnError: true` for critical safety checks
- Test hooks thoroughly before enabling them
- Consider using read-only hooks first

## Debugging

To see hook execution:
1. Check Claude's output for hook-related messages
2. Add echo statements to your hook commands
3. Check command exit codes with `$?`

## Disabling Hooks

To temporarily disable hooks:
```bash
unset CLAUDE_HOOKS_FILE
```

Or rename/remove your hooks configuration file.