# SpaceWarp++

SpaceWarp++ is a complete remake of the original [SpaceWarp](https://github.com/Wam25/SpaceWarp) game, now implemented in [Rust](https://www.rust-lang.org/).

## Planned Features
- **Level Editor**: Create and customize your own levels.
- **Improved Levels**: Enjoy more challenging and engaging levels.
- **Enhanced Art**: Artwork designed by [txtur](https://github.com/txtur) to provide a visually appealing experience.
- **Texture Packs**: Apply different texture packs to change the game's appearance.
- **Puzzles**: Encounter actual puzzles within the game.
- **A Satisfying Ending**: Experience a conclusive and rewarding finale.

## Level Format (`.sw`)
The level format in SpaceWarp++ consists of several components:

### Emoji Representation
The first 16 lines of the level file represent an emoji-based visualization of the level. Each emoji corresponds to a specific tile in the game. Here's an example:
```
⬜⬜⬅️⬇️⬇️⬇️⬇️3️⃣⬇️⬇️⬇️⬇️⬇️⬇️⬇️3️⃣
⬜⬜1️⃣👇👇⬜⬜1️⃣⬜⬜⬜⬜⬜⬜⬜⬅️
⬜⬜1️⃣⬜⬜⬜⬜⏬⬜⬜⬜⬜⬜⬜⬜⬅️
⬜⬜1️⃣⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜❤️⬜⬅️
⬜⬜1️⃣⬜⬜⬜⬜🟨⬜⬜⬜⬜⬜⬜⬜⬅️
⬜⬜1️⃣🔴⬜⬜⬜⏪0️⃣0️⃣0️⃣0️⃣0️⃣0️⃣0️⃣3️⃣
⬜⬜⬅️0️⃣⏩⬜⬜⬜⬜⬜⬜⬜⬜⬜👇⬅️
⬜⬜1️⃣⬜⬜⬜⬜⬜🟥⬜⬜⬜⬜💙⬜⬅️
⬜⬜⏬⬜⬜⬜⬜⬜↖️⬆️⬆️↗️⬜⬜⬜⬅️
⬜⬜⬜⬜⬜⏪0️⃣0️⃣⬇️⬇️⬇️⬇️0️⃣0️⃣0️⃣5️⃣
⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬅️
⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🟦⬜💛⬜⬅️
⬜⬜⏪0️⃣0️⃣0️⃣0️⃣0️⃣0️⃣0️⃣0️⃣0️⃣0️⃣0️⃣0️⃣⬇️
⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜
⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜🟥🟨⬜
⬆️⬆️⬆️⬆️⬆️⬆️⬆️⬆️⬆️⬆️⬆️⬆️⬆️⬆️⬆️⬆️
```

### Level Exits
The following 4 lines in the level file indicate which level to go to based on the direction the player exits. A value of -1 indicates that the corresponding direction is not an available exit. For example:
```
2
-1
-1
-1
```
In this case, exiting in the upward direction leads to level 2, while the other directions don't provide an exit.

### Player Reset Location
The subsequent 2 lines in the level file represent the X and Y coordinates of the player's reset position within the map. These coordinates determine where the player respawns after resetting:
```
13
3
```
In this example, the player's reset position is at coordinates (13, 3) on the map.
