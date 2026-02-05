# This module is responsible for:
#
# • Initializing the global CSS provider (once).
# • Attaching it to the default screen.
# • Exposing small helpers like style_window() and style_primary_button().

from gi.repository import Gtk, Gdk

_UMRS_CSS = b”””
window.umrs-main-window {
    background-color: #111111;
    color: #e0e0e0;
}

window.umrs-main-window headerbar,
headerbar.umrs-headerbar {
    background-color: #121212;
    color: #e0e0e0;
}

label.umrs-section-title {
    font-weight: bold;
    color: #88ff88; /* subtle green accent; tweak as needed */
}

button.umrs-primary-action {
    background-image: none;
    background-color: #1b401b;
    color: #e0ffe0;
}

button.umrs-primary-action:hover {
    background-color: #285f28;
}
”””

_css_provider = None

def init_umrs_theme():
    “””
    Initialize the UMRS CSS provider and attach it to the default screen.
    Safe to call multiple times; it only does real work once.
    “””
    global _css_provider

    if _css_provider is not None:
        return

    provider = Gtk.CssProvider()
    provider.load_from_data(_UMRS_CSS)

    screen = Gdk.Screen.get_default()
    if screen is not None:
        Gtk.StyleContext.add_provider_for_screen(
            screen,
            provider,
            Gtk.STYLE_PROVIDER_PRIORITY_APPLICATION,
        )
    _css_provider = provider

def style_umrs_window(window: Gtk.Window):
    “””
    Apply the UMRS main-window style class to a window.
    “””
    init_umrs_theme()
    ctx = window.get_style_context()
    ctx.add_class(“umrs-main-window”)

def style_section_title(label: Gtk.Label):
    “””
    Apply the UMRS section title style to a label.
    “””
    init_umrs_theme()
    ctx = label.get_style_context()
    ctx.add_class(“umrs-section-title”)

def style_primary_button(button: Gtk.Button):
    “””
    Apply the UMRS primary-action style to a button.
    “””
    init_umrs_theme()
    ctx = button.get_style_context()
    ctx.add_class(“umrs-primary-action”)
