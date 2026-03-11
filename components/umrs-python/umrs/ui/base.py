import gi
gi.require_version(“Gtk”, “4.0”)
from gi.repository import Gtk, Gdk

§# umrs/application.py
import gi
gi.require_version(“Gtk”, “3.0”)
from gi.repository import Gtk, Gio

class UMRSApplication(Gtk.Application):
    def init(self, app_id=“org.umrs.example”, flags=Gio.ApplicationFlags.FLAGS_NONE):
        Gtk.Application.init(self, application_id=app_id, flags=flags)
        self.umrs_env_ok = False
        self.selinux_context = None

    def do_startup(self):
        Gtk.Application.do_startup(self)
        self._init_umrs_environment()
        self._load_umrs_theme()

    def do_activate(self):
        # Subclasses will create their UMRSWindow here
        pass

    def _init_umrs_environment(self):
        # Example: check FIPS, SELinux, MLS, key dirs, etc.
        # Set self.umrs_env_ok accordingly and log or raise if needed.
        self.selinux_context = self._get_selinux_context()

    def _get_selinux_context(self):
        try:
            with open(”/proc/self/attr/current”, “r”, encoding=“utf-8”) as f:
                return f.read().strip()
        except OSError:
            return None

    def get_selinux_context(self):
        return self.selinux_context

    def require_fips(self):
        try:
            with open(”/proc/sys/crypto/fips_enabled”, “r”, encoding=“ascii”) as f:
                if f.read().strip() != “1”:
                    # You could raise or log/audit here
                    raise RuntimeError(“FIPS mode is required but not enabled”)
        except OSError:
            raise RuntimeError(“Cannot determine FIPS mode state”)

    def _load_umrs_theme(self):
        # Load your CSS and apply to the default screen
        pass

    def audit_event(self, event_type, details):
        # Hook into UMRS audit/log system
        pass


class UmrsCssManager:
    _provider = None
    _is_loaded = False

    @classmethod
    def load(cls):
        if cls._is_loaded:
            return

        css = b”””
        /* UMRS dark wizard theme */

        window.umrs-window {
            background-color: #111416;
            color: #d5e3d5;
        }

        .umrs-header {
            font-weight: bold;
            color: #84c991;
        }

        .umrs-accent {
            color: #6fbf73;
        }

        button {
            background-color: #1a1f1a;
            border: 1px solid #355f35;
        }

        button:hover {
            background-color: #223022;
        }
        “””

        provider = Gtk.CssProvider()
        provider.load_from_data(css)

        screen = Gdk.Screen.get_default()
        if screen is not None:
            Gtk.StyleContext.add_provider_for_screen(
                screen,
                provider,
                Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION
            )

        cls._provider = provider
        cls._is_loaded = True


class UmrsWindow(Gtk.ApplicationWindow):
    def init(self, app, title=None):
        super().init(application=app)

        # Ensure CSS is loaded once
        UmrsCssManager.load()

        # Apply shared UMRS window style
        ctx = self.get_style_context()
        ctx.add_class(“umrs-window”)

        if title is not None:
            self.set_title(title)

        self.set_default_size(1024, 640)
        self.set_size_request(800, 480)
        self.set_position(Gtk.WindowPosition.CENTER)

        self.connect(“delete-event”, self._on_delete_event)

    def _on_delete_event(self, widget, event):
        # Central place for exit handling
        return False

    def show_error(self, message, secondary=None):
        dialog = Gtk.MessageDialog(
            transient_for=self,
            modal=True,
            message_type=Gtk.MessageType.ERROR,
            buttons=Gtk.ButtonsType.OK,
            text=message,
        )
        if secondary:
            dialog.format_secondary_text(secondary)
        dialog.run()
        dialog.destroy()

    def show_info(self, message, secondary=None):
        dialog = Gtk.MessageDialog(
            transient_for=self,
            modal=True,
            message_type=Gtk.MessageType.INFO,
            buttons=Gtk.ButtonsType.OK,
            text=message,
        )
        if secondary:
            dialog.format_secondary_text(secondary)
        dialog.run()
        dialog.destroy()
