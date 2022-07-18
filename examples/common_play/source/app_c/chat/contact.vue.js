_$pi.define("app_c/chat/contact.vue", ["require", "exports", "module", "pi_gui/widget/direct", "./contact.vue.tpl", "pi_common/im/server/db/user.struct", "app_c/chat/server/chat.topic", "pi_common/ui/main_root", "pi_common/store/store", "app_a/util/constant", "app_a/util/new_store", "app_a/util/setup", "app_a/widget/tips/tips.vue"], function (require, exports, module, direct_1, contact_vue_tpl_1, user_struct_1, chat_topic_1, main_root_1, store_1, constant_1, new_store_1, setup_1, tips_vue_1) {
    "use strict";

    exports.initMeta = void 0;
    let w = null;
    class ContactVue {
        constructor() {
            this.contact = store_1.find(user_struct_1.Contact);
            this.sysMsg = new_store_1.newStore.find('sysMessage');
            this.rid = '';
        }
        create() {
            w = this;
        }
        goBack() {
            main_root_1.close(this);
        }
        inputRid(e) {
            this.rid = e.current.value;
        }
        addUser() {
            if (!this.rid) return;
            setup_1.clientRpc(chat_topic_1.addFriend, Number(this.rid)).then(r => {
                console.log('addFriend', constant_1.parseRes(r));
                main_root_1.open(tips_vue_1.default, { text: constant_1.parseRes(r) });
            });
        }
        deal(e) {
            this.sysMsg = this.sysMsg.filter(v => {
                var _a;
                let from = (_a = JSON.parse(v.msg)) === null || _a === void 0 ? void 0 : _a.from;
                return from !== e.from;
            });
            new_store_1.newStore.notify('sysMessage', this.sysMsg);
        }
    }
    exports.default = ContactVue;
    store_1.register(user_struct_1.Contact, r => {
        if (!w) return;
        w.contact = r;
    });
    new_store_1.newStore.register('sysMessage', r => {
        console.log('register sysMessage', r);
        if (!w) return;
        w.sysMsg = r;
    });
    exports.initMeta = () => {
        let _$tpl = "app_c/chat/contact.vue.tpl.ts",
            _$cssPath = "app_c/chat/contact.vue.wcss",
            _$cssHash = 3992927675;
        ContactVue["_$meta"] = { tpl: _$tpl, css: _$cssPath, tplFunc: contact_vue_tpl_1.BW2, cssHash: _$cssHash };
    };
    exports.initMeta();
    direct_1.defineAccessors(ContactVue, ["contact", "sysMsg", "rid"]);
});