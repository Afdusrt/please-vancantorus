# please-vancantorus

An image viewer with global hotkeys and always-on-top mode (windows only), designed for sequential browsing (e.g. puzzle game speedrun assistance, or slideshows).

## Features:

- Always on top window
- Global hotkeys to change image defined in config
- Images get sorted with 2 keys, the first letter, and a number in the file name. 
> puzzle game file names example: "a_beginner-7" comes before "a_beginner-10", that comes before "b-easypeasy-2" ...
<hr>
<img width="342" height="401" alt="image-1" src="https://github.com/user-attachments/assets/ca6d0935-18aa-45db-9985-8c5d401364d0" />
<hr>

## Config:

```shell
left=q
right=e
left10=a
right10=d
font=ComicRelief-Regular.ttf
font_load_res=30
font_color_hex=FFFFFF
font_background_hex=000000
```
left10 / right10 = skip 10 images, font = file path, font_load_res is the font size at which the font is loaded.

## Build:
Just "cargo build --release", no special build script.
### Dependencies:
device_query, raylib, rfd, windows*
