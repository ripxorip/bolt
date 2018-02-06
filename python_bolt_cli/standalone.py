# ============================================================================
# FILE: standalone.py
# AUTHOR: David Johansson
# License: MIT license
# ============================================================================

import curses
from bolt_tui.tui import tui

def print_panes(stdscr, bolt):
    buffer = bolt.get_panes()
    for i in range(0, len(buffer)):
        stdscr.addstr(i, 0, buffer[i])

def main(stdscr):
    input_buffer = ''

    # calculate width & height
    width = curses.COLS
    height = curses.LINES - 1

    # init bolt
    bolt = tui(width, height)

    # Clear screen
    stdscr.clear()

    # print the panes
    print_panes(stdscr, bolt)
    
    debug_line = curses.LINES - 1

    while True:
        try:
            c = stdscr.getkey()
        except KeyboardInterrupt:
            break
        stdscr.clear()

        if c == '\t':
            bolt.cmd_tab()
        elif c == 'KEY_UP':
            bolt.cmd_up()
        elif c == 'KEY_DOWN':
            bolt.cmd_down()
        elif c == '\x7f':
            if input_buffer == '':
                bolt.cmd_backspace()
            else:
                input_buffer = input_buffer[:-1]
                bolt.filter(input_buffer)

        elif c == '\x0a' or c == '\x0d':
            bolt.cmd_enter()
        else:
            input_buffer += c
            bolt.filter(input_buffer)

        print_panes(stdscr, bolt)
        stdscr.addstr(debug_line, 1, input_buffer)
        stdscr.addstr(debug_line, curses.COLS-10, str(curses.LINES) + ' x ' + str(curses.COLS))


curses.wrapper(main)
