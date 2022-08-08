_$pi.define("app_c/chat/msg_item.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./msg_item.vue.tpl"], function (require, exports, module, direct_1, msg_item_vue_tpl_1) {
    "use strict";

    exports.initMeta = void 0;
    class Message {
        getTime() {
            return new Date(this.msgLog.time).toLocaleString();
        }
    }
    exports.default = Message;
    exports.initMeta = () => {
        let _$tpl = "app_c/chat/msg_item.vue.tpl.ts",
            _$cssPath = "app_c/chat/msg_item.vue.wcss",
            _$cssHash = 1522088923;
        Message["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: msg_item_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(Message, ["msgLog"]);
});