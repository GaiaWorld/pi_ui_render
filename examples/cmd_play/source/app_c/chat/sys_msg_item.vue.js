_$pi.define("app_c/chat/sys_msg_item.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./sys_msg_item.vue.tpl", "app_a/util/constant", "app_a/util/setup", "app_c/chat/server/chat.topic", "pi_common/im/server/db/message.struct", "pi_gui/widget/direct"], function (require, exports, module, direct_1, sys_msg_item_vue_tpl_1, constant_1, setup_1, chat_topic_1, message_struct_1, direct_2) {
    "use strict";

    exports.initMeta = void 0;
    class SysMsg {
        propsUpdate() {
            var _a;
            if (!this.msgLog) return;
            this.send = (_a = JSON.parse(this.msgLog.msg)) === null || _a === void 0 ? void 0 : _a.from;
            const date = new Date(this.msgLog.time);
            this.time = date.getHours() + ':' + date.getMinutes() + ':' + date.getSeconds();
            if (this.msgLog.mtype === message_struct_1.MSG_TYPE.ADD_USER) {
                this.mess = '申请加好友';
                this.showBtn = true;
            } else if (this.msgLog.mtype === message_struct_1.MSG_TYPE.ADD_USER_REFUSE) {
                this.mess = '拒绝好友申请';
            } else if (this.msgLog.mtype === message_struct_1.MSG_TYPE.ADD_USER_OK) {
                this.mess = '同意好友申请';
            }
        }
        accept() {
            setup_1.clientRpc(chat_topic_1.acceptFriend, this.send).then(r => {
                console.log('acceptFriend', constant_1.parseRes(r));
                direct_2.emit(this, 'ev-deal', { from: this.send });
            });
            return true;
        }
        refuse() {
            setup_1.clientRpc(chat_topic_1.refuseFriend, this.send).then(r => {
                console.log('refuseFriend', constant_1.parseRes(r));
                direct_2.emit(this, 'ev-deal', { from: this.send });
            });
            return true;
        }
    }
    exports.default = SysMsg;
    exports.initMeta = () => {
        let _$tpl = "app_c/chat/sys_msg_item.vue.tpl.ts",
            _$cssPath = "app_c/chat/sys_msg_item.vue.wcss",
            _$cssHash = 3186372753;
        SysMsg["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: sys_msg_item_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(SysMsg, ["msgLog", "send", "mess", "time", "showBtn"]);
});