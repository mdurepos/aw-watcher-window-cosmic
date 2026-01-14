import logging
from typing import Optional

from .exceptions import FatalError
from .wayland_toplevel import get_watcher

logger = logging.getLogger(__name__)


def get_current_window_wayland() -> Optional[dict]:
    """
    Get the current window using Wayland foreign-toplevel protocol.

    Returns:
        dict with 'app' and 'title' keys, or None if no window is active
    """
    watcher = get_watcher()
    return watcher.get_current_window()


def get_current_window(strategy: Optional[str] = None) -> Optional[dict]:
    """
    Get the current window information.

    For Wayland compositors like cosmic-comp, we use the foreign-toplevel protocol.

    :raises FatalError: if a fatal error occurs
    """
    return get_current_window_wayland()
