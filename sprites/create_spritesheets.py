#!/usr/bin/env python3
"""Create sprite sheets from separated icon packs and generate atlas txt files."""

import os
import math
import re
from PIL import Image


def natural_sort_key(s):
    """Sort strings with embedded numbers naturally (fa1, fa2, ..., fa10, fa11)."""
    return [int(c) if c.isdigit() else c.lower() for c in re.split(r'(\d+)', s)]


def create_spritesheet_from_dir(icon_dir, output_png, output_atlas, icon_size, name_fn=None):
    """
    Create a sprite sheet from a directory of individual icon PNGs.

    Args:
        icon_dir: Directory containing individual icon PNGs
        output_png: Output sprite sheet path
        output_atlas: Output atlas txt path
        icon_size: Expected (width, height) of each icon
        name_fn: Optional function to derive icon name from filename
    """
    # Collect all PNGs
    files = sorted(
        [f for f in os.listdir(icon_dir) if f.lower().endswith('.png')],
        key=natural_sort_key
    )

    if not files:
        print(f"  No PNGs found in {icon_dir}")
        return

    count = len(files)
    cols = math.ceil(math.sqrt(count))
    rows = math.ceil(count / cols)

    w, h = icon_size
    sheet_w = cols * w
    sheet_h = rows * h

    print(f"  {count} icons → {cols}x{rows} grid → {sheet_w}x{sheet_h}px sheet")

    sheet = Image.new('RGBA', (sheet_w, sheet_h), (0, 0, 0, 0))
    atlas_lines = []

    for idx, fname in enumerate(files):
        col = idx % cols
        row = idx // cols
        x = col * w
        y = row * h

        icon = Image.open(os.path.join(icon_dir, fname)).convert('RGBA')
        # Resize if needed
        if icon.size != icon_size:
            icon = icon.resize(icon_size, Image.NEAREST)

        sheet.paste(icon, (x, y))

        if name_fn:
            name = name_fn(fname)
        else:
            name = os.path.splitext(fname)[0]

        atlas_lines.append(f"{name}\t{x}\t{y}\t{w}\t{h}")

    sheet.save(output_png)

    with open(output_atlas, 'w') as f:
        f.write(f"# Sprite Sheet Atlas: {os.path.basename(output_png)}\n")
        f.write(f"# Total sprites: {count}\n")
        f.write(f"# Sheet size: {sheet_w}x{sheet_h}\n")
        f.write(f"# Icon size: {w}x{h}\n")
        f.write(f"# Format: name\\tx\\ty\\twidth\\theight\n")
        f.write("\n".join(atlas_lines) + "\n")

    print(f"  Saved: {output_png}")
    print(f"  Atlas: {output_atlas}")


def create_spritesheet_from_categorized_dirs(base_dir, output_png, output_atlas, icon_size):
    """
    Create a sprite sheet from icons organized in subdirectories (categories).
    Icon names include the category prefix.
    """
    files_with_paths = []

    for category in sorted(os.listdir(base_dir)):
        cat_dir = os.path.join(base_dir, category)
        if not os.path.isdir(cat_dir):
            continue
        for fname in sorted(os.listdir(cat_dir), key=natural_sort_key):
            if fname.lower().endswith('.png'):
                name = f"{category}/{os.path.splitext(fname)[0]}"
                files_with_paths.append((name, os.path.join(cat_dir, fname)))

    if not files_with_paths:
        print(f"  No PNGs found in {base_dir}")
        return

    count = len(files_with_paths)
    cols = math.ceil(math.sqrt(count))
    rows = math.ceil(count / cols)

    w, h = icon_size
    sheet_w = cols * w
    sheet_h = rows * h

    print(f"  {count} icons → {cols}x{rows} grid → {sheet_w}x{sheet_h}px sheet")

    sheet = Image.new('RGBA', (sheet_w, sheet_h), (0, 0, 0, 0))
    atlas_lines = []

    for idx, (name, filepath) in enumerate(files_with_paths):
        col = idx % cols
        row = idx // cols
        x = col * w
        y = row * h

        icon = Image.open(filepath).convert('RGBA')
        if icon.size != icon_size:
            icon = icon.resize(icon_size, Image.NEAREST)

        sheet.paste(icon, (x, y))
        atlas_lines.append(f"{name}\t{x}\t{y}\t{w}\t{h}")

    sheet.save(output_png)

    with open(output_atlas, 'w') as f:
        f.write(f"# Sprite Sheet Atlas: {os.path.basename(output_png)}\n")
        f.write(f"# Total sprites: {count}\n")
        f.write(f"# Sheet size: {sheet_w}x{sheet_h}\n")
        f.write(f"# Icon size: {w}x{h}\n")
        f.write(f"# Format: name\\tx\\ty\\twidth\\theight\n")
        f.write("\n".join(atlas_lines) + "\n")

    print(f"  Saved: {output_png}")
    print(f"  Atlas: {output_atlas}")


def generate_32rogues_atlas(rogues_dir, output_dir):
    """
    Parse 32rogues existing spritesheets + txt files and generate
    coordinate-based atlas files.
    """
    tile_size = 32
    sheets = [
        ('items', 'items.png', 'items.txt'),
        ('monsters', 'monsters.png', 'monsters.txt'),
        ('animals', 'animals.png', 'animals.txt'),
        ('rogues', 'rogues.png', 'rogues.txt'),
        ('tiles', 'tiles.png', 'tiles.txt'),
        ('animated-tiles', 'animated-tiles.png', 'animated-tiles.txt'),
        ('autotiles', 'autotiles.png', 'autotiles.txt'),
    ]

    for sheet_name, png_file, txt_file in sheets:
        png_path = os.path.join(rogues_dir, png_file)
        txt_path = os.path.join(rogues_dir, txt_file)

        if not os.path.exists(png_path) or not os.path.exists(txt_path):
            print(f"  Skipping {sheet_name}: missing files")
            continue

        img = Image.open(png_path)
        sheet_w, sheet_h = img.size
        cols = sheet_w // tile_size
        rows = sheet_h // tile_size

        print(f"  {sheet_name}: {sheet_w}x{sheet_h} → {cols}x{rows} grid")

        # Pre-read all non-empty lines to determine format
        with open(txt_path) as f:
            all_lines = [l.strip() for l in f if l.strip()]

        atlas_lines = []

        # Detect format by checking first line
        has_row_col = any(re.match(r'^\d+\.[a-z]', l) for l in all_lines)
        has_row_num = any(re.match(r'^\d+\.\s', l) for l in all_lines)

        if has_row_col:
            # Format 1: "row.col_letter. name" (items, monsters, animals, rogues, tiles)
            for line in all_lines:
                m = re.match(r'^(\d+)\.([a-z])\.?\s+(.+)$', line)
                if m:
                    row_num = int(m.group(1)) - 1
                    col_num = ord(m.group(2)) - ord('a')
                    name = m.group(3).strip()
                    x = col_num * tile_size
                    y = row_num * tile_size
                    atlas_lines.append(f"{name}\t{x}\t{y}\t{tile_size}\t{tile_size}")

        elif has_row_num:
            # Format 2: "row_num. name" for animated tiles (each row = animation frames)
            for line in all_lines:
                m = re.match(r'^(\d+)\.\s+(.+)$', line)
                if m:
                    row_num = int(m.group(1)) - 1
                    name = m.group(2).strip()
                    for col_num in range(cols):
                        x = col_num * tile_size
                        y = row_num * tile_size
                        frame_name = f"{name} [frame {col_num}]"
                        atlas_lines.append(f"{frame_name}\t{x}\t{y}\t{tile_size}\t{tile_size}")

        else:
            # Format 3: plain text labels (autotiles) - divide rows evenly among labels
            label_count = len(all_lines)
            rows_per_group = rows // max(1, label_count)
            current_row = 0
            for line in all_lines:
                name = line.strip()
                for r in range(rows_per_group):
                    for c in range(cols):
                        x = c * tile_size
                        y = (current_row + r) * tile_size
                        tile_name = f"{name} [row {r}, col {c}]"
                        atlas_lines.append(f"{tile_name}\t{x}\t{y}\t{tile_size}\t{tile_size}")
                current_row += rows_per_group

        # Copy the spritesheet
        import shutil
        out_png = os.path.join(output_dir, f"32rogues-{sheet_name}.png")
        shutil.copy2(png_path, out_png)

        # Write atlas
        out_atlas = os.path.join(output_dir, f"32rogues-{sheet_name}-atlas.txt")
        with open(out_atlas, 'w') as f:
            f.write(f"# Sprite Sheet Atlas: 32rogues-{sheet_name}.png\n")
            f.write(f"# Total sprites: {len(atlas_lines)}\n")
            f.write(f"# Sheet size: {sheet_w}x{sheet_h}\n")
            f.write(f"# Icon size: {tile_size}x{tile_size}\n")
            f.write(f"# Format: name\\tx\\ty\\twidth\\theight\n")
            f.write("\n".join(atlas_lines) + "\n")

        print(f"  Atlas: {out_atlas} ({len(atlas_lines)} entries)")


def main():
    base = '/home/user/the-cave/sprites'
    output_dir = os.path.join(base, 'output')
    os.makedirs(output_dir, exist_ok=True)

    # 1. Pixel Art Icon Pack - RPG (107 icons, 32x32, organized in category folders)
    print("=" * 60)
    print("1. Pixel Art Icon Pack - RPG")
    print("=" * 60)
    rpg_dir = os.path.join(base, 'extract/pixel-art-rpg')
    create_spritesheet_from_categorized_dirs(
        rpg_dir,
        os.path.join(output_dir, 'pixel-art-rpg-spritesheet.png'),
        os.path.join(output_dir, 'pixel-art-rpg-atlas.txt'),
        (32, 32)
    )

    # 2. Raven Fantasy Icons - Separated files at 3 resolutions
    raven_base = os.path.join(base, 'extract/raven-fantasy/Free - Raven Fantasy Icons/Separated Files')

    for size_name, prefix, icon_size in [('16x16', 'fa', 16), ('32x32', 'fb', 32), ('64x64', 'fc', 64)]:
        print()
        print("=" * 60)
        print(f"2. Raven Fantasy Icons - {size_name}")
        print("=" * 60)

        size_dir = os.path.join(raven_base, size_name)
        if os.path.isdir(size_dir):
            create_spritesheet_from_dir(
                size_dir,
                os.path.join(output_dir, f'raven-fantasy-{size_name}-spritesheet.png'),
                os.path.join(output_dir, f'raven-fantasy-{size_name}-atlas.txt'),
                (icon_size, icon_size),
                name_fn=lambda f: os.path.splitext(f)[0]
            )

    # 3. 32rogues - existing spritesheets, generate coordinate atlases
    print()
    print("=" * 60)
    print("3. 32rogues (existing spritesheets → coordinate atlases)")
    print("=" * 60)
    rogues_dir = os.path.join(base, 'extract/32rogues/32rogues')
    generate_32rogues_atlas(rogues_dir, output_dir)

    # Also copy items-palette-swaps (no txt file, just reference)
    import shutil
    swaps_src = os.path.join(rogues_dir, 'items-palette-swaps.png')
    if os.path.exists(swaps_src):
        shutil.copy2(swaps_src, os.path.join(output_dir, '32rogues-items-palette-swaps.png'))
        print(f"  Copied: 32rogues-items-palette-swaps.png (no atlas - palette swap variant)")

    print()
    print("=" * 60)
    print("Done! All sprite sheets and atlases saved to:")
    print(f"  {output_dir}")
    print("=" * 60)

    # List output files
    for f in sorted(os.listdir(output_dir)):
        fp = os.path.join(output_dir, f)
        size = os.path.getsize(fp)
        print(f"  {f} ({size:,} bytes)")


if __name__ == '__main__':
    main()
