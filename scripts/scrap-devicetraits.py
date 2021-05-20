# pip install pyquery
from pyquery import PyQuery as pq

URL = "https://developers.google.com/assistant/smarthome/traits"

d = pq(url = URL)

for tr in d('tr').items():
    id = tr.attr["id"]
    if id == None:
        continue
    name = tr("td:nth-child(1)").text()
    description = tr("td:nth-child(3)").text().replace("\n", "\n/// ")


    out = """
/// ID: {}
/// 
/// {}
{},
    """.format(id, description, name)
    print(out)
