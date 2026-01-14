import logging
import os
import subprocess
import json
import threading
from typing import Optional

logger = logging.getLogger(__name__)


class WaylandToplevelWatcher:
    """
    Watches Wayland toplevels using a Rust helper binary.

    The Rust helper uses the wlr-foreign-toplevel-management-v1 protocol
    and outputs window information as JSON to stdout.
    """

    def __init__(self):
        self._current_window: Optional[dict] = None
        self._lock = threading.Lock()
        self._running = False
        self._thread: Optional[threading.Thread] = None
        self._process: Optional[subprocess.Popen] = None

    def start(self):
        """Start the Rust helper in a separate thread."""
        if self._running:
            return

        self._running = True
        self._thread = threading.Thread(target=self._run_helper, daemon=True)
        self._thread.start()
        logger.info("Wayland toplevel watcher started")

    def stop(self):
        """Stop the Rust helper."""
        self._running = False
        if self._process:
            self._process.terminate()
            try:
                self._process.wait(timeout=2.0)
            except subprocess.TimeoutExpired:
                self._process.kill()
        if self._thread:
            self._thread.join(timeout=2.0)
        logger.info("Wayland toplevel watcher stopped")

    def get_current_window(self) -> Optional[dict]:
        """
        Get the current active window information.

        Returns:
            dict with 'app' and 'title' keys, or None if no window is active
        """
        with self._lock:
            return self._current_window

    def _run_helper(self):
        """Run the Rust helper and parse its output."""
        try:
            # Find the Rust helper binary
            # First, check if we're in development mode (running from source)
            current_dir = os.path.dirname(os.path.abspath(__file__))
            project_root = os.path.abspath(os.path.join(current_dir, "..", ".."))

            # When running via poetry in editable mode, __file__ points to the source
            # But the path might need adjustment if we're in a virtualenv
            # Check if target directory exists at project_root
            if not os.path.exists(os.path.join(project_root, "target")):
                # Try to find the actual project root by looking for Cargo.toml
                search_path = current_dir
                while search_path != "/":
                    if os.path.exists(os.path.join(search_path, "Cargo.toml")):
                        project_root = search_path
                        break
                    search_path = os.path.dirname(search_path)

            bin_path = os.path.join(
                project_root, "target", "release", "aw-watcher-window-cosmic-helper"
            )

            logger.debug(f"Looking for Rust helper at: {bin_path}")

            # If release binary doesn't exist, try debug
            if not os.path.exists(bin_path):
                bin_path = os.path.join(
                    project_root, "target", "debug", "aw-watcher-window-cosmic-helper"
                )
                logger.debug(f"Release not found, trying debug: {bin_path}")

            # If still not found, try to find it in the current directory or PATH
            if not os.path.exists(bin_path):
                # Check if it's in the same directory as the script
                bin_path = os.path.join(current_dir, "aw-watcher-window-cosmic-helper")
                logger.debug(f"Debug not found, trying current dir: {bin_path}")

                # If not, try to find it in PATH
                if not os.path.exists(bin_path):
                    from shutil import which

                    bin_path = which("aw-watcher-window-cosmic-helper")
                    logger.debug(f"Current dir not found, checking PATH: {bin_path}")

            if not bin_path or not os.path.exists(bin_path):
                logger.error(
                    f"Rust helper binary not found. Searched in: {project_root}/target/{{release,debug}} and PATH"
                )
                logger.error(f"Current directory: {current_dir}")
                logger.error(f"Project root: {project_root}")
                return

            self._process = subprocess.Popen(
                [bin_path], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True
            )

            while self._running and self._process.poll() is None:
                line = self._process.stdout.readline()
                if not line:
                    break

                try:
                    data = json.loads(line.strip())

                    if isinstance(data, dict):
                        if data.get("Window"):
                            window = data["Window"]
                            with self._lock:
                                self._current_window = {
                                    "app": window.get("app_id") or "unknown",
                                    "title": window.get("title") or "unknown",
                                }
                        elif data.get("NoWindow"):
                            with self._lock:
                                self._current_window = None
                        elif data.get("Error"):
                            logger.error(f"Rust helper error: {data['Error']}")

                except json.JSONDecodeError as e:
                    logger.warning(f"Failed to parse JSON from helper: {line.strip()}")
                    logger.debug(f"JSON decode error: {e}")
                except Exception as e:
                    logger.warning(
                        f"Error processing helper output: {type(e).__name__}: {e}"
                    )

        except FileNotFoundError:
            logger.error("Rust helper binary not found")
        except Exception as e:
            logger.exception(f"Error running Rust helper: {e}")


# Global instance
_watcher: Optional[WaylandToplevelWatcher] = None


def get_watcher() -> WaylandToplevelWatcher:
    """Get or create the global Wayland toplevel watcher instance."""
    global _watcher
    if _watcher is None:
        _watcher = WaylandToplevelWatcher()
    return _watcher
