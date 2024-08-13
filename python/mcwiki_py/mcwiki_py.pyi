from typing import List

class WikiPage:
    """Represents the wiki page."""

    text: str

    def __init__(self, text: str): ...
    def parse(self) -> List["Node"]: ...

class Node:
    class Title:
        level: int
        text: str

    class Text:
        text: str
