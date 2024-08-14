from typing import Any, Literal, Union, List

class WikiPage:
    """Represents the wiki page."""

    text: str

    def __init__(self, text: str): ...
    def parse(self) -> List[Union["Node.Title", "Node.Text"]]: ...

class Node:
    class Base:
        name: Any

    class Title(Node.Base):
        name: Literal["title"]
        level: int
        text: str

    class Text(Node.Base):
        name: Literal["text"]
        text: str

        def get_meta(self) -> List: ...
