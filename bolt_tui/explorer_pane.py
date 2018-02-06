# ============================================================================
# FILE: explorer_pane.py
# AUTHOR: David Johansson
# License: MIT license
# ============================================================================

class explorer_pane(object):
    current_line = 0

    def __init__(self, focused, objects, explorer_id, cwd, width, height):
        self.focused = focused
        self.objects = objects
        self.explorer_id = explorer_id
        self.cwd = cwd
        self.width = width
        self.height = height

    def header(self):
        bar = '=' * (self.width - 2)

        if(self.focused):
            leadingC = '# '
        else:
            leadingC = '" '

        ret = []
        ret.append(leadingC + bar)
        ret.append(leadingC + 'Bolt (alpha)')

        # Shall be highlighted
        ret.append(leadingC + '  $>' + self.cwd)
        qhStr = '  Quik Help: <Ret>:Open   <C-q>:Quit   <C-s>:Set CWD'
        ret.append(leadingC + qhStr)
        qhStr = '             <C-f>:Search <C-p>:Create File'
        ret.append(leadingC + qhStr)
        qhStr = '             <F2>:Rename  <F5>:Copy    <F6>:Move   '
        ret.append(leadingC + qhStr)
        qhStr = '             <F7>:Mkdir   <F8>:Delete   '
        ret.append(leadingC + qhStr)
        ret.append(leadingC + bar)
        return ret


    def set_objects(self, objects):
        self.current_line = 0
        self.objects = objects

    def setCwd(self, cwd):
        self.cwd = cwd

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
        buf = self.header()

        for i in range(0, len(self.objects)):
            obj = self.objects[i]

            if self.current_line == i and self.focused:
                str = '--> '
            else:
                str = '    '

            if obj.type == 'folder':
                str += '+'
            else:
                str += ' '

            str += obj.name

            if obj.type == 'folder':
                str += '/'
            else:
                str += ''
            buf.append(str)

        return buf
