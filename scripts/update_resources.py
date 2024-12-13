import hashlib
import json
from pathlib import Path


def calculate_sha256(file_path: Path) -> str:
    sha256_hash = hashlib.sha256()
    with open(file_path, "rb") as f:
        for byte_block in iter(lambda: f.read(4096), b""):
            sha256_hash.update(byte_block)
    return sha256_hash.hexdigest()


def generate_resources_json(resources_dir: Path) -> dict:
    fonts = []
    fonts_dir = resources_dir / "fonts"
    for file in fonts_dir.iterdir():
        hash = calculate_sha256(file)
        fonts.append({"file": file.name, "hash": hash})
    fonts.sort(key=lambda i: i["file"])

    images = []
    images_dir = resources_dir / "images"
    for file in images_dir.rglob("*"):
        if file.is_file():
            relative_path = file.relative_to(images_dir)
            hash = calculate_sha256(file)
            images.append({"file": str(relative_path), "hash": hash})
    images.sort(key=lambda i: i["file"])

    return {"fonts": fonts, "images": images}


def main():
    resources_dir = Path(__file__).parent.parent / "resources"
    resources = generate_resources_json(resources_dir)
    with open(resources_dir / "resources.json", "w") as f:
        json.dump(resources, f, indent=2)


if __name__ == "__main__":
    main()
