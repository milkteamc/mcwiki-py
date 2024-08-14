# mcwiki.py
Fetch Minecraft Wiki information.

**Procedures**:
1. Fetch a Wiki page holding the markup editor/source. (Python)
2. Parse/process the Wiki text. (Rust)

Simple [2-step](https://youtube.com/watch?v=Z_MvkyuOJgk) program!

```python
import mcwiki_py as wiki

wiki.search("cocoa beans")  # NotImplemented

page = WikiPage("==wiki text==")
page.parse()  # Vec<Node> => List[Node_x | Node_y | ...]
page.get_meta()  # todo!()
```
