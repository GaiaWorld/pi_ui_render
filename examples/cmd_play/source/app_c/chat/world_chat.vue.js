_$pi.define("app_c/chat/world_chat.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./world_chat.vue.tpl", "pi_common/im/server/db/message.struct", "app_c/chat/server/chat.struct", "app_c/chat/server/chat.topic", "pi_common/ui/main_root", "app_a/util/new_store", "app_a/util/constant", "app_c/util/tools", "app_a/util/setup", "app_a/widget/tips/tips.vue", "./chat.vue", "./contact.vue"], function (require, exports, module, direct_1, world_chat_vue_tpl_1, message_struct_1, chat_struct_1, chat_topic_1, main_root_1, new_store_1, constant_1, tools_1, setup_1, tips_vue_1, chat_vue_1, contact_vue_1) {
    "use strict";

    exports.initMeta = void 0;
    let w = null;
    class WorldChatVue {
        constructor() {
            this.groupLogs = [];
            this.msg = '';
            this.rid = '';
            this.btnText = '发送';
            this.timer = 0;
        }
        create() {
            w = this;
            let list = new_store_1.newStore.find('groupChat');
            if (list) {
                tools_1.dealWorldChatList(list).then(r => {
                    this.groupLogs = r;
                });
            }
        }
        inputMsg(e) {
            this.msg = e.current.value;
        }
        sendMsg() {
            if (!this.msg) return main_root_1.open(tips_vue_1.default, { text: "消息不能为空" });
            if (this.timer > 0) return main_root_1.open(tips_vue_1.default, { text: `请在${this.timer}秒后发送` });
            if (this.msg.length >= 50) return main_root_1.open(tips_vue_1.default, { text: "不能超过50个字符" });
            const log = new chat_struct_1.SendMsgParam();
            log.conv_type = message_struct_1.CONV_TYPE.CONV_GROUP;
            log.to = new_store_1.newStore.find('gid');
            log.mtype = message_struct_1.MSG_TYPE.TXT;
            log.msg = this.msg;
            log.msg_class = message_struct_1.MSG_CLASS.MSG;
            setup_1.clientRpc(chat_topic_1.sendMessage, log).then(r => {
                console.log('sendMessage', constant_1.parseRes(r));
                main_root_1.open(tips_vue_1.default, { text: constant_1.parseRes(r) });
                if (constant_1.parseRes(r) === "success") {
                    this.timer = 10;
                    this.startTimer();
                }
            });
        }
        goBack() {
            main_root_1.close(this);
        }
        startTimer() {
            clearInterval(this.interval);
            this.interval = setInterval(() => {
                this.timer--;
                if (this.timer > 0) {
                    this.btnText = `${this.timer}秒`;
                } else {
                    clearInterval(this.interval);
                    this.btnText = '发送';
                }
            }, 1000);
        }
        destroy() {
            clearInterval(this.interval);
        }
        chat() {
            main_root_1.open(chat_vue_1.default);
        }
        contact() {
            main_root_1.open(contact_vue_1.default);
        }
    }
    exports.default = WorldChatVue;
    new_store_1.newStore.register(`groupChat`, r => {
        console.log('register groupChat', r);
        if (!w) return;
        tools_1.dealWorldChatList(r).then(list => {
            w.groupLogs = list;
        });
    });
    exports.initMeta = () => {
        let _$tpl = "app_c/chat/world_chat.vue.tpl.ts",
            _$cssPath = "app_c/chat/world_chat.vue.wcss",
            _$cssHash = 3069379716;
        WorldChatVue["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: world_chat_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(WorldChatVue, ["groupLogs", "msg", "btnText"]);
    direct_1.addField(WorldChatVue, ['rid', 'interval', 'timer']);
});