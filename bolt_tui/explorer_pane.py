# ============================================================================
# FILE: explorer_pane.py
# AUTHOR: David Johansson
# License: MIT license
# ============================================================================

class explorer_pane(object):
    current_line = 0

    def __init__(self, focused, objects, explorer_id):
        self.focused = focused
        self.objects = objects
        self.explorer_id = explorer_id

    def set_objects(self, objects):
        self.current_line = 0
        self.objects = objects

    def get_current_object(self):
        return self.objects[self.current_line]

    def get_explorer_id(self):
        return self.explorer_id 

    def cmd_up(self):
        self.current_line = max(0, self.current_line-1)

    def cmd_down(self):
        self.current_line = min(len(self.objects)-1, self.current_line+1)

    def set_focus(self, focused):
        self.focused = focused

    def get_pane(self):
        buf = []

        for i in range(0, len(self.objects)):
            if self.current_line == i and self.focused:
                str = '--> '
            else:
                str = '    '
            str += self.objects[i].name
            buf.append(str)

        return buf
