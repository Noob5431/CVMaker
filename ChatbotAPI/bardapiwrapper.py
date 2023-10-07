from bardapi import Bard 

import os

import re

os.environ["_BARD_API_KEY"] = "bwgpTnZFjZDtLeL1ExJjOb-rOc4i0dubgyL7Z-_VLnzPE7U7z2YxrmQqfQgRp3kYTXWU5Q."

message = input("")  

def find_strings(text):
  """Finds all strings in a text that start with a • and end before going to the next line.

  Args:
    text: A string.

  Returns:
    A list of strings.
  """

  strings = []
  for match in re.finditer(r"\•(.*?)\n", text):
    strings.append(match.group(1))
  return strings

text = (Bard().get_answer(str(message))['content'])

strings = find_strings(text)

print(strings)