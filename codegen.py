# Generate python stubs
from typing import Optional

RELEVANT_DIRS: list[str] = ["album", "artist", "search"]
BLACKLISTED_CLASSES: list[str] = ["ImageId"]
STUB_FILE_NAME: str = "python_bindings/bandcamp/__init__.pyi"

TYPE_REPLACEMENTS: dict[str, str] = {
    "Option<": "Optional[",
    "<": "[",
    ">": "]",
    "u64": "int",
    "u32": "int",
    "f32": "float",
    "HashMap": "dict",
    "Vec": "list",
    "String": "str",
    "DateTime[Utc]": "datetime"
}


class Field:
    def __init__(self):
        self.name: str = ""
        self.type: str = ""
        self.description: Optional[str] = None


class Class:
    def __init__(self, name: str, fields: list[Field]):
        self.name: str = name
        self.fields: list[Field] = fields


def get_class_fields(lines: list[str]) -> list[Field]:
    current_field = Field()
    result: list[Field] = []
    for line in lines:
        line = line.strip()
        if line.startswith("///"):
            comment: str = line.removeprefix("///").strip()
            if current_field.description is None:
                current_field.description = comment
            else:
                current_field.description += "\n" + comment
        elif line.startswith("#") or line.startswith("//"):
            pass
        else:
            name, type_ = line.split(":", 1)
            current_field.name = name.removeprefix("pub").strip()
            current_field.type = type_.removesuffix(",").strip()
            result.append(current_field)
            current_field = Field()
    return result

def get_classes(file: str) -> list[Class]:
    result: list[Class] = []
    current_class: Optional[Class] = None
    current_lines: list[str] = []
    for line in file.splitlines():
        if "pub struct" in line:
            name = line.split("struct", 1)[-1].removesuffix("{").strip()
            current_class = Class(name, [])
        elif line.startswith("}") and current_class is not None:
            current_class.fields = get_class_fields(current_lines)
            result.append(current_class)
            current_class = None
            current_lines = []
        elif current_class is not None:
            current_lines.append(line)
    return result


def rust_type_to_python(type_name: str) -> str:
    for old, new in TYPE_REPLACEMENTS.items():
        type_name = type_name.replace(old, new)
    return type_name


def main():
    with open(STUB_FILE_NAME) as f:
        old_content: str = f.read()
    stub_file = open(STUB_FILE_NAME, "w")
    for line in old_content.splitlines(keepends=True):
        stub_file.write(line)
        if line.startswith("# DO NOT EDIT"):
            break
    for dir in RELEVANT_DIRS:
        with open(f"src/{dir}/mod.rs") as f:
            lines = f.read()
        classes = get_classes(lines)
        for class_ in classes:
            if class_.name in BLACKLISTED_CLASSES:
                continue
            stub_file.write(f"class {class_.name}:\n")
            for field in class_.fields:
                stub_file.write(f"    @property\n")
                py_type = rust_type_to_python(field.type)
                stub_file.write(f"    def {field.name}(self) -> {py_type}: ...\n")
            stub_file.write("\n")


if __name__ == "__main__":
    main()
