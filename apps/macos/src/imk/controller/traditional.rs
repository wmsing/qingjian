//! 繁体输出快捷键：翻转 `[general] traditional` 并提示。

use super::*;

impl QingjianInputController {
    pub(super) fn toggle_traditional_output(&self, client: TextClient<'_>) -> bool {
        let anchor = client.caret_rect();
        host::with(|h| {
            let on = !h.settings.config().general.traditional;
            if !h.settings.set_bool("general", "traditional", on) {
                return;
            }
            h.apply_config(false);
            let message = if on {
                "繁体输出：开"
            } else {
                "繁体输出：关"
            };
            h.show_notice(message, anchor);
        });
        self.refresh(client);
        true
    }
}
