# toolbox.py — GTK4 GridView launcher

import gi
gi.require_version(“Gtk”, “4.0”)
from gi.repository import Gtk, Gio

import subprocess
import os
import sys


class ToolboxWindow(Gtk.ApplicationWindow):
    def init(self, app):
        super().init(application=app, title=“UMRS Toolbox”)
        self.set_default_size(400, 300)

        # List of tool names
        tool_names = [“Script One”, “Script Two”]

        # Map names to commands
        python_exe = sys.executable or “/usr/bin/python3”
        base_dir = os.path.abspath(os.path.dirname(file))

        self.commands = {
            “Script One”: [python_exe, os.path.join(base_dir, “script_one.py”)],
            “Script Two”: [python_exe, os.path.join(base_dir, “script_two.py”)],
        }

        # Model
        model = Gtk.StringList.new(tool_names)

        # Factory
        factory = Gtk.SignalListItemFactory()
        factory.connect(“setup”, self.on_setup)
        factory.connect(“bind”, self.on_bind)

        # GridView (GTK4 replacement for IconView)
        grid = Gtk.GridView(model=model, factory=factory)
        grid.set_single_click_activate(True)
        grid.connect(“activate”, self.on_activate)

        scroller = Gtk.ScrolledWindow()
        scroller.set_child(grid)

        self.set_child(scroller)

    def on_setup(self, factory, list_item):
        box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=6)
        box.set_margin_top(8)
        box.set_margin_bottom(8)
        box.set_margin_start(8)
        box.set_margin_end(8)

        icon = Gtk.Label(label=“🧰”)
        label = Gtk.Label()

        box.append(icon)
        box.append(label)

        list_item.set_child(box)
        list_item.label = label

    def on_bind(self, factory, list_item):
        obj = list_item.get_item()
        list_item.label.set_text(obj.get_string())

    def on_activate(self, grid, position):
        model = grid.get_model()
        name = model.get_item(position).get_string()

        cmd = self.commands.get(name)
        if cmd:
            subprocess.Popen(cmd)


class ToolboxApp(Gtk.Application):
    def init(self):
        super().init(application_id=“org.umrs.toolbox”)

    def do_activate(self):
        win = self.get_active_window()
        if not win:
            win = ToolboxWindow(self)
        win.present()


if name == “main”:
    app = ToolboxApp()
    app.run(sys.argv)
