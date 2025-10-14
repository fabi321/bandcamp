# Generate python stubs
from typing import Optional
import sys

RELEVANT_FILES: list[str] = ["album.rs", "artist.rs", "search.rs", "util.rs"]
BLACKLISTED_CLASSES: list[str] = ["ImageId"]
STUB_FILE_NAME: str = "python_bindings/bandcamp_lib/bandcamp_lib.pyi"

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
    def __init__(self, name: str, type: str, description: Optional[str] = None):
        self.name: str = name
        self.type: str = type
        self.description: Optional[str] = description


class Class:
    def __init__(self, name: str, fields: list[Field], description: Optional[str] = None):
        self.name: str = name
        self.fields: list[Field] = fields
        self.description: Optional[str] = description


def get_class_fields(lines: list[str]) -> list[Field]:
    result: list[Field] = []
    current_comments: str = ""
    for line in lines:
        line = line.strip()
        if line.startswith("///"):
            current_comments += "\n" + line.removeprefix("///").strip()
        elif line.startswith("pub"):
            name, type_ = line.split(":", 1)
            result.append(Field(
                name.removeprefix("pub").strip(),
                type_.removesuffix(",").strip(),
                current_comments[1:] or None,
            ))
            current_comments = ""
    return result

def get_classes(file: str) -> list[Class]:
    result: list[Class] = []
    current_class: Optional[Class] = None
    current_lines: list[str] = []
    current_comments: str = ""
    for line in file.splitlines():
        if "pub struct" in line and "$" not in line:
            name = line.split("struct", 1)[-1].removesuffix("{").strip()
            current_class = Class(name, [], current_comments[1:] or None)
        elif line.startswith("///"):
            current_comments += "\n" + line.removeprefix("///").strip()
        elif "{" in line:
            current_comments = ""
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


def rust_doc_to_python(doc: str, indent: int = 2) -> str:
    indentation: str = " " * indent * 4
    if '\n' in doc:
        lines = '\n'.join(f"{indentation}{line}" for line in doc.split('\n'))
        return f'{indentation}"""\n{lines}\n{indentation}"""\n\n'
    return f'{indentation}"""{doc}"""\n\n'


def handle_image_resolution(lines: list[str]) -> str:
    if all("ImageResolution" not in line for line in lines):
        return ""
    result = "class ImageResolution:\n"
    first_line: int = [i for i, line in enumerate(lines) if "ImageResolution" in line] [0]
    comment: str = ""
    for i in range(first_line + 1, len(lines)):
        line: str = lines[i].strip()
        if line.startswith("///"):
            comment += "\n" + line.removeprefix("///").strip()
        elif line.startswith("}"):
            break
        else:
            result += "    " + line.removesuffix(",") + "\n"
            if comment:
                result += rust_doc_to_python(comment[1:], 1)
                comment = ""
    if not result.endswith("\n\n"):
        result += "\n"
    return result

def main():
    with open(STUB_FILE_NAME) as f:
        old_content: str = f.read()
    new_file = ""
    for line in old_content.splitlines(keepends=True):
        new_file += line
        if line.startswith("# DO NOT EDIT"):
            break
    for file in RELEVANT_FILES:
        with open(f"src/{file}") as f:
            lines = f.read()
        classes = get_classes(lines)
        for class_ in classes:
            if class_.name in BLACKLISTED_CLASSES:
                continue
            new_file += f"class {class_.name}:\n"
            if class_.description:
                new_file += rust_doc_to_python(class_.description, indent=1)
            for field in class_.fields:
                new_file += f"    @property\n"
                py_type = rust_type_to_python(field.type)
                new_file += f"    def {field.name}(self) -> {py_type}:"
                if field.description:
                    new_file += '\n'
                    new_file += rust_doc_to_python(field.description)
                else:
                    new_file += " ...\n"
            if not new_file.endswith("\n\n"):
                new_file += "\n"
        new_file += handle_image_resolution(lines.splitlines())
    if new_file.endswith("\n\n"):
        new_file = new_file[:-1]
    if new_file != old_content:
        if '-c' in sys.argv or '--check' in sys.argv:
            print(f"File {STUB_FILE_NAME} is outdated, please run codegen.py")
            exit(1)
        else:
            with open(STUB_FILE_NAME, "w") as f:
                f.write(new_file)


if __name__ == "__main__":
    main()
