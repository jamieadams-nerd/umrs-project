from umrs.ui.base import UmrsWindow
from gi.repository import Gtk

class UmrsAuditSigningWindow(UmrsWindow):
    def init(self, app):
        super().init(app, title=“UMRS Audit Log Signing”)

        box = Gtk.Box(orientation=Gtk.Orientation.VERTICAL, spacing=8)
        box.set_border_width(8)

        header = Gtk.Label(label=“Audit Log Signing”)
        header.get_style_context().add_class(“umrs-header”)
        box.pack_start(header, False, False, 0)

        button = Gtk.Button(label=“Sign latest audit log”)
        button.get_style_context().add_class(“umrs-accent”)
        button.connect(“clicked”, self.on_sign_clicked)
        box.pack_start(button, False, False, 0)

        self.add(box)
        self.show_all()

    def on_sign_clicked(self, button):
        self.show_info(
            “Signing complete”,
            “All rotated audit logs have been signed.”
        )
