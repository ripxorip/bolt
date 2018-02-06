from bolt_tui.explorer_pane import explorer_pane
from bolt.bolt import bolt
from math import floor

class tui(object):
    panes = []

    def __init__(self, width, height):
        pane_width = floor((width - 2)/2)
        self.pane_width = pane_width
        self.height = height

        self.bolt = bolt()

        self.panes.append(explorer_pane(True, self.bolt.getListing('exp1'), 'exp1'))
        self.panes.append(explorer_pane(False, self.bolt.getListing('exp2'), 'exp2'))

        self.selected = 0

    def cmd_up(self):
        self.panes[self.selected].cmd_up()

    def cmd_down(self):
        self.panes[self.selected].cmd_down()

    def cmd_tab(self):
        self.panes[self.selected].set_focus(False)
        self.selected = (self.selected + 1) % 2
        self.panes[self.selected].set_focus(True)

    def cmd_backspace(self):
        pane = self.panes[self.selected]

        # execute
        explorer_id = pane.get_explorer_id()
        self.bolt.cd(explorer_id, -1)

        # update pane
        pane.set_objects(self.bolt.getListing(explorer_id))

    def cmd_enter(self):
        pane = self.panes[self.selected]

        # exucute
        obj = pane.get_current_object()
        if obj.type == 'folder':
            explorer_id = pane.get_explorer_id()
            self.bolt.cd(explorer_id, obj.id)

            # update pane
            pane.set_objects(self.bolt.getListing(explorer_id))

    def filter(self, str):
        pane = self.panes[self.selected]
        explorer_id = pane.get_explorer_id()
        self.bolt.updateListing(str, explorer_id)
        pane.set_objects(self.bolt.getListing(explorer_id))

    def get_panes(self):
        left_pane = self.panes[0].get_pane()
        right_pane = self.panes[1].get_pane()
        width = self.pane_width

        buf = []

        for i in range(0, self.height):
            if i < len(left_pane):
                left = left_pane[i]
            else:
                left = ''

            if i < len(right_pane):
                right = right_pane[i]
            else:
                right = ''

            str = left.ljust(width) + ' | ' + right.ljust(width)

            buf.append(str)

        return buf
