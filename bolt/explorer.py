# ============================================================================
# FILE: explorer.py
# AUTHOR: Philip Karlsson <philipkarlsson at me.com>
# License: MIT license
# ============================================================================
import os
import shutil
from bolt.filter import filter


class listingEntry(object):
    def __init__(self, name, type, id):
        self.type = type
        self.name = name
        self.id = id


class explorer(object):
    """ Class for an explorer that is used in the panes """
    def __init__(self, cwd):
        self.isSearcher = False
        # Instance of the filter
        self.filter = filter()
        self.cwd = cwd
        self.createNewListing()

    def createNewListing(self):
        rawFiles = os.listdir(self.cwd)
        self.currentListing = []
        # Create current files
        for idx, f in enumerate(rawFiles):
            if(os.path.isdir(os.path.join(self.cwd, f))):
                type = 'folder'
            else:
                type = 'file'
            self.currentListing.append(listingEntry(f, type, idx))
        self.filteredListing = self.currentListing[:]

    def rename(self, newName):
        os.rename(self.getEntryAtId(id).name, os.path.join(self.cwd, newName))
        self.cd('.')

    def copy(self, id, dest):
        selFile = self.getEntryAtId(id).name
        if os.path.isdir(selFile):
            shutil.copytree(selFile, dest)
        else:
            shutil.copy(selFile, dest)

    def delete(self, id, yesno):
        path = self.getEntryAtId(id).name
        if yesno == "y":
            selFile = path
            if os.path.isdir(selFile):
                shutil.rmtree(selFile)
            else:
                os.remove(selFile)

    def move(self, id, dest):
        path = self.getEntryAtId(id).name
        os.rename(path, dest)

    def mkdir(self, name):
        os.makedirs(os.path.join(self.cwd, name))

    def createFile(self, name):
        open(os.path.join(self.cwd, name), 'a').close()

    def cd(self, id):
        path = self.getEntryAtId(id).name
        if(id == -1):
            path = '..'
        self.cwd = os.path.abspath(os.path.join(self.cwd, path))
        self.createNewListing()

    def updateListing(self, pattern):
        self.pattern = pattern
        self.filter.filter(self.currentListing, pattern, self.filteredListing)

    def getEntryAtId(self, id):
        for entry in self.currentListing:
            if(entry.id == id):
                return entry

    def getListing(self):
        return self.filteredListing
