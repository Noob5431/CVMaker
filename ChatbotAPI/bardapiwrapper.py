from bardapi import Bard 
import os
import re

def askBard (choice,input):
  os.environ["_BARD_API_KEY"] = "bwgpThdVSUgQPwFjw6KnCRPuFoUpvK3cnuft_UJLoe65SRwoUXBBnj2fppnJlSWtscvZZg."

  choice = 0 # 0 - work experience, 1 - project experience, 2 - extra curricular

  if (choice == 0):
    f = open("work.txt", "r")

  if (choice == 1):
    f = open("project.txt", "r")

  if (choice == 2):
    f = open("extrac.txt", "r")

  prompt = f.read()

  input = prompt + input

  def find_strings(text):
    """Finds all strings in a text that start with a • and end before going to the next line.

    Args:
      text: A string.

    Returns:
      A list of strings.
    """

    strings = []
    for match in re.finditer(r"\~(.*?)\n",text):
      strings.append(match.group(1))
    return strings

  text = (Bard().get_answer(str(input))['content'])

  text=text.replace("*","")

  strings = find_strings(text)

  return strings