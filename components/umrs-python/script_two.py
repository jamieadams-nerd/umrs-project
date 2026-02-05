# script_two.py — standalone GTK4 app

import gi
gi.require_version(“Gtk”, “4.0”)
from gi.repository import Gtk
import sys


class ScriptTwoWindow(Gtk.ApplicationWindow):
    def init(self, app):
        super().init(application=app, title=“Script Two”)
        self.set_default_size(300, 200)
        self.set_child(Gtk.Label(label=“Hello from Script Two”))


class ScriptTwoApp(Gtk.Application):
    def init(self):
        super().init(application_id=“org.umrs.scripttwo”)

    def do_activate(self):
        win = self.get_active_window()
        if not win:
            win = ScriptTwoWindow(self)
        win.present()


if name == “main”:
    app = ScriptTwoApp()
    app.run(sys.argv)
