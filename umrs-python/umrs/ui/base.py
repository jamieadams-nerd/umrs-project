§import gi
§gi.require_version(“Gtk”, “4.0”)
§from gi.repository import Gtk, Gdk
§
§
§class UmrsCssManager:
§    _provider = None
§    _is_loaded = False
§
§    @classmethod
§    def load(cls):
§        if cls._is_loaded:
§            return
§
§        css = b”””
§        /* UMRS dark wizard theme */
§
§        window.umrs-window {
§            background-color: #111416;
§            color: #d5e3d5;
§        }
§
§        .umrs-header {
§            font-weight: bold;
§            color: #84c991;
§        }
§
§        .umrs-accent {
§            color: #6fbf73;
§        }
§
§        button {
§            background-color: #1a1f1a;
§            border: 1px solid #355f35;
§        }
§
§        button:hover {
§            background-color: #223022;
§        }
§        “””
§
§        provider = Gtk.CssProvider()
§        provider.load_from_data(css)
§
§        screen = Gdk.Screen.get_default()
§        if screen is not None:
§            Gtk.StyleContext.add_provider_for_screen(
§                screen,
§                provider,
§                Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
§            )
§
§        cls._provider = provider
§        cls._is_loaded = True
§
§
§class UmrsWindow(Gtk.ApplicationWindow):
§    def init(self, app, title=None):
§        super().init(application=app)
§
§        # Ensure CSS is loaded once
§        UmrsCssManager.load()
§
§        # Apply shared UMRS window style
§        ctx = self.get_style_context()
§        ctx.add_class(“umrs-window”)
§
§        if title is not None:
§            self.set_title(title)
§
§        self.set_default_size(1024, 640)
§        self.set_size_request(800, 480)
§        self.set_position(Gtk.WindowPosition.CENTER)
§
§        self.connect(“delete-event”, self._on_delete_event)
§
§    def _on_delete_event(self, widget, event):
§        # Central place for exit handling
§        return False
§
§    def show_error(self, message, secondary=None):
§        dialog = Gtk.MessageDialog(
§            transient_for=self,
§            modal=True,
§            message_type=Gtk.MessageType.ERROR,
§            buttons=Gtk.ButtonsType.OK,
§            text=message,
§        )
§        if secondary:
§            dialog.format_secondary_text(secondary)
§        dialog.run()
§        dialog.destroy()
§
§    def show_info(self, message, secondary=None):
§        dialog = Gtk.MessageDialog(
§            transient_for=self,
§            modal=True,
§            message_type=Gtk.MessageType.INFO,
§            buttons=Gtk.ButtonsType.OK,
§            text=message,
§        )
§        if secondary:
§            dialog.format_secondary_text(secondary)
§        dialog.run()
§        dialog.destroy()
