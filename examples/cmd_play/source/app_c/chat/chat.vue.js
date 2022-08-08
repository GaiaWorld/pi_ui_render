_$pi.define("app_c/chat/chat.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./chat.vue.tpl", "pi_common/im/server/db/message.struct", "app_c/chat/server/chat.struct", "app_c/chat/server/chat.topic", "pi_common/ui/main_root", "app_a/util/new_store", "app_a/util/constant", "app_c/util/tools", "app_a/util/setup", "app_a/widget/tips/tips.vue"], function (require, exports, module, direct_1, chat_vue_tpl_1, message_struct_1, chat_struct_1, chat_topic_1, main_root_1, new_store_1, constant_1, tools_1, setup_1, tips_vue_1) {
    "use strict";

    exports.initMeta = void 0;
    let w = null;
    class ChatVue {
        constructor() {
            this.friendLogs = [];
            this.msg = '';
            this.rid = '';
        }
        create() {
            w = this;
        }
        inputRid(e) {
            this.rid = e.current.value;
        }
        inputMsg(e) {
            this.msg = e.current.value;
        }
        sendMsg() {
            if (!this.rid) return main_root_1.open(tips_vue_1.default, { text: "输入对方rid" });
            if (!this.msg) return main_root_1.open(tips_vue_1.default, { text: "消息不能为空" });
            const log = new chat_struct_1.SendMsgParam();
            log.conv_type = message_struct_1.CONV_TYPE.CONV_C2C;
            log.to = Number(this.rid);
            log.mtype = message_struct_1.MSG_TYPE.TXT;
            log.msg = this.msg;
            log.msg_class = message_struct_1.MSG_CLASS.MSG;
            setup_1.clientRpc(chat_topic_1.sendMessage, log).then(r => {
                console.log('sendMessage', constant_1.parseRes(r));
                main_root_1.open(tips_vue_1.default, { text: constant_1.parseRes(r) });
            });
        }
        goBack() {
            main_root_1.close(this);
        }
        getHistory() {
            if (!this.rid) return;
            tools_1.getUserHistoryMsg(Number(this.rid)).then(r => {
                this.friendLogs = r;
            });
        }
    }
    exports.default = ChatVue;
    new_store_1.newStore.register(`friendChat`, r => {
        if (!w) return;
        console.log('register friendChat', r);
        tools_1.dealUserChatList(r).then(list => {
            w.friendLogs = list;
        });
    });
    exports.initMeta = () => {
        let _$tpl = "app_c/chat/chat.vue.tpl.ts",
            _$cssPath = "app_c/chat/chat.vue.wcss",
            _$cssHash = 366806536;
        ChatVue["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: chat_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(ChatVue, ["friendLogs", "rid", "msg"]);
});