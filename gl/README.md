# gl (grocery list)

This is personal software. I have never found a grocery list solution that has the 3 key features I want:

[x] autocomplete from previous entries
[] sharing/multiplayer with members of my household
[] categorization to sort by the order I move through the grocery store

`gl` aims to accomplish these three things.

Hosted at [gl.ryangeary.dev]()

## User Manual

### Autocomplete

#### Entry Format

Entries can be written in these formats:

- `item name` - just the item (e.g., `apples`)
- `2lb item name` - quantity + item (e.g., `2lb ground beef`)  
- `item name - notes` - item + notes (e.g., `apples - organic`)
- `2lb item name (notes)` - all three (e.g., `2lb ground beef (80% lean)`)

**Important:** No spaces in quantities! Use `2lb` not `2 lb`.

#### Autocomplete behavior

**How it works:**
1. As you type an item name, autocomplete suggestions appear based on previous entries
2. Uses prefix matching: typing `car` shows `carrots` if you've added carrots before
3. Quantity-aware: typing `2x car` will suggest `carrots` and complete to `2x carrots`
4. Keyboard navigation: use ↑/↓ arrows, Enter to select, Escape to close
5. Click suggestions to select them

**Data Storage:**
- Input is automatically parsed into separate quantity, item, and notes fields
- Autocomplete matches against item names only, ignoring quantities
- This enables quantity-independent suggestions (e.g., `2x carrots` and `3x carrots` both suggest `carrots`)
