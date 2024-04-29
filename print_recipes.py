import os, random, shutil

source=input("Enter the recipe Directory : ")

for entry in os.scandir(source):
    print("%s"%(entry.name))
    with open(entry.path) as file:
        print("%s"%(file.read()))

