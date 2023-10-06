from bardapi import Bard 

import os

os.environ["_BARD_API_KEY"] = "bwgpTplV7I9Ni9mB9MJJ0ig-JG_rgpbeMCq6KJ6gHrvyRFEQu2Mn2JJGfFuKCjgy_ahNaw."

message = input("Enter Your Prompt:")  

print(Bard().get_answer(str(message))['content'])