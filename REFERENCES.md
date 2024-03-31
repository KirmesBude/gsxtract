# REFERENCES

- [GSTLA docs](https://docs.google.com/document/d/1nsDI-Bx6p65X25CPGdJOk8K-WVsYjYTBEN-i_bR8bdk/edit)
- [Image Palettes](http://forum.goldensunhacking.net/index.php?topic=297)
- [A Guide to GS Compression Format 0](http://forum.goldensunhacking.net/index.php?topic=1456.0)
- [Compression Formats](http://forum.goldensunhacking.net/index.php?topic=2624.15)
- [GBA tech ref](https://problemkaputt.de/gbatek.htm)
- [GSMagic](https://cdn.discordapp.com/attachments/1067969130793279578/1067969131443404850/gsmagic_April_2020_-_height_tiles_saving_treasure_coords_and_class_type_patching.7z?ex=66173342&is=6604be42&hm=8b16f15cb6b9149ef6e777801aa40adb63c71315f67dd6c00c6060f2c258bfdd&)
- [GSTLA Editor](http://forum.goldensunhacking.net/index.php?topic=1936.0)
- [Golden Sun compression tools](https://github.com/romhack/GoldenSunCompression)
- [Golden Sun Editor](https://github.com/RyudoSynbios/golden-sun-editor)
- [FutureFractal Script](https://cdn.discordapp.com/attachments/332622755419652096/1223731963639238678/gslz.py?ex=661aec33&is=66087733&hm=0f33b6edac7dcac08dcd53e63eb8084399db6ef2f4601fa5af5f4fd860be6b1f&)

Sprite Compression formats:
format 0: RLE only
format 1: standard LZ + RLE
format 2: standard LZ + RLE, also sprite GFX is in the filetable instead of being pointed to by the sprite info struct
format 3: sprite LZ + RLE
there are several compression formats involved with sprites: RLE, standard LZ, and sprite LZ
standard LZ is by far the most complex one since it can use one of two separate compression formats of its own which is specified by the first byte (00 or 01) 
in GS2, the decompression routine for standard LZ is at 080128A8, and the one for sprite LZ is at 08000C4C (copied to 03000694 on boot) 
sprite RLE is very simple: bytes < E0 are color indices, bytes >= E0 encode a sequence of transparent (color 00) pixels which is (byte - DF) pixels long
sprite LZ is much simpler, it's basically a simplified version of standard LZ's format 01

here's a list of all the compression formats btw:
LZ (LZ0 or LZ1): can use either LZ0 or LZ1 depending on the format byte at the start (00 or 01). used for general-purpose compression
LZ1: standalone LZ1 without the format byte. also used for general-purpose compression
LZ2: basically a hybrid of LZ0 and icon compression, used for map tilesets. (starts with the format byte 02 despite being handled by a different decompression routine)
Icon compression: used for icons and portraits, optimized for 4bpp GFX
BG compression: used for battle backgrounds and GS2's epilogue art, optimized for 128-color 8bpp GFX
Sprite RLE: simple RLE that only compresses sequences of transparent pixels, used for sprites
Sprite LZ: a slightly-different version of LZ1 used by most sprites on top of sprite RLE
Huffman: text compression