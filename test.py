import mcwiki_py

text = """{{Infobox block
| group = Age 0
| 1-1 = Cocoa Age 0.png
| 1-1caption = Java Edition
| 1-2 = Cocoa Age 0 BE.png
| 1-2caption = Bedrock Edition
| group2 = Age 1
| 2-1 = Cocoa Age 1.png
| 2-1caption = Java Edition
| 2-2 = Cocoa Age 1 BE.png
| 2-2caption = Bedrock Edition
| group3 = Age 2
| 3-1 = Cocoa Age 2.png
| 3-1caption = Java Edition
| 3-2 = Cocoa Age 2 BE.png
| 3-2caption = Bedrock Edition
| image=Cocoa Beans.png
| *-1class = notpageimage
| *-2class = notpageimage
| invimage = Cocoa Beans
| transparent = Yes
| light = No
| tool = axe
| renewable = Yes
| stackable = Yes (64)
| flammable = No
| lavasusceptible = No
}}
'''Cocoa beans''' are items obtained from cocoa pods and are used to plant them, as well as to craft [[brown dye]] and [[cookie]]s.

'''Cocoa pods''' are [[plant]] blocks placed on [[jungle log]] sides that grow cocoa beans, and can be found naturally in [[jungle]]s.

== Obtaining ==
{{IN|Java}}, cocoa beans are only obtained through the natural generation of cocoa pods, while {{in|Bedrock}}, they can also be found in bonus chests, from [[fishing]] inside the jungle, bamboo jungle and sparse jungle biomes and during a [[trading]] with a [[wandering trader]]."""

page = mcwiki_py.WikiPage(text)
print(page.parse()[-2])
