# SpaceWarp++

SpaceWarp++ is a full remake of [SpaceWarp](https://github.com/Wam25/SpaceWarp) in [Rust](https://www.rust-lang.org/)

### Planned Features:
 - Level editor
 - Better levels
 - Better art *(created by [txtur](https://github.com/txtur))*
 - Texture packs
 - Actual Puzzles
 - An actual ending

### Level Format (`.sw`)
The first 16 lines are an ASCII representation of the level.

```
#############.##
#.............##
###...........##
#............###
#...........####
################
################
################
################
################
################
################
################
################
################
################
```
The next 4 lines represent which level to go to based on which direction they exit in (-1 means you cant exit in that direction)
```
2
-1
-1
-1
```
The next 2 lines represent the X and Y location of the player when they reset in the map.
```
13
3
```