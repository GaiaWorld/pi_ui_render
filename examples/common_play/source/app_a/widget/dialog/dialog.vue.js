_$pi.define("app_a/widget/dialog/dialog.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./dialog.vue.tpl", "pi_gui/widget/direct", "pi_common/ui/main_root"], function (require, exports, module, direct_1, dialog_vue_tpl_1, direct_2, main_root_1) {
    "use strict";

    exports.initMeta = void 0;
    class Dialog {
        constructor() {
            this.showCloseTip = true;
            this.keep = 0;
            this.title = "";
            this.widget = "";
            this.handleClose = () => {
                direct_2.emit(this, 'ev-close', {});
                main_root_1.close(this);
            };
            this.maskClick = () => {
                if (this.showCloseTip) this.handleClose();
            };
        }
        empty() {
            return true;
        }
    }
    exports.default = Dialog;
    exports.initMeta = () => {
        let _$tpl = "app_a/widget/dialog/dialog.vue.tpl.ts",
            _$cssPath = "app_a/widget/dialog/dialog.vue.wcss",
            _$cssHash = 2159306038;
        Dialog["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: dialog_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Dialog, ["keep", "title", "widget", "showCloseTip", "textColor"]);
    direct_1.addField(Dialog, ['ok', 'handleClose', 'maskClick']);
});