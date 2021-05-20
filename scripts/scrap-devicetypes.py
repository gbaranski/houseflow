# pip install pyquery
from pyquery import PyQuery as pq

URL = "https://developers.google.com/assistant/smarthome/guides"

d = pq(url = URL)

# https://stackoverflow.com/questions/19053707/converting-snake-case-to-lower-camel-case-lowercamelcase
def to_camel_case(snake_str):
    components = snake_str.split('_')
    return components[0] + ''.join(x.title() for x in components[1:])

for type in d('td > a > code'):
    original_name = type.text
    elem_id = type.text.lower()
    description = d("#{} > td:nth-child(2)".format(elem_id)).text()
    new_name = to_camel_case(elem_id).capitalize()
    out = """
/// Google Assistant name: {}
/// 
/// {}
{},
    """.format(original_name, description, new_name)
    print(out)
