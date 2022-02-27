from pathlib import Path

BOOK_SRC = (Path() / "book" / "src").resolve()
DREAM = (BOOK_SRC / "the_dream.md").resolve()


def process_feature(feature: str) -> tuple[str, str]:
    title = [title_part for index, title_part in enumerate(feature.split("**")) if index % 2 == 1]
    return (" ".join(title), "".join(feature.split("**")))


def main():
    with open(DREAM) as f:
        for line in f:
            if line.strip() == "## Key Features":
                break
        
        sub_heading = None
        for line in f:
            if line.startswith("###"):
                sub_heading = line.strip()[4:]
                print("\n" + sub_heading)
            elif line.startswith("- "):
                print(process_feature(line.strip()[4:]))


if __name__ == "__main__":
    main()
