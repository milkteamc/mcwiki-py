from typing import Union, List

class WikiPage:
    """Represents the wiki page."""

    text: str

    def __init__(self, text: str): ...
    def parse(self) -> List[Union["Node_Title", "Node_Text"]]: ...

class Node_Title:
    level: int
    text: str

class Node_Text:
    text: str
