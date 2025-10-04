# Running UMT on Linux

> ⚠️
>
> Drag&drop freezes the application!
>
> Use `File`/`Open` or `^o` to open files instead.

Windows GUI binaries can be used on Linux via Wine.

`$ WINE_D3D_CONFIG="renderer=vulkan" wine UndertaleModTool.exe`

Setting renderer to `vulkan` fixes dialog windows being rendered pitch black.

## Fonts

Fonts can be fixed by running `$ winetricks arial` 

## Dialog window crash

When unfocusing window with a popup open, UMT may crash with `XI_BadDevice`.

To fix this, unset `DISPLAY` or set it to empty: `$ DISPLAY= ..`

## Reference
- [How to run UndertaleModTool on Wine by WilsontheWolf](https://gist.github.com/WilsontheWolf/0eca47bf15f3a1811d03db93af960df5)
